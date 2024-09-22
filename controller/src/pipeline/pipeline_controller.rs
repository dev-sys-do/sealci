use actix_multipart::form::{tempfile::TempFile, text::Text as MpText, MultipartForm};
use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::{io::Read, sync::Arc};
use tracing::info;

use crate::{parser::pipe_parser::ParsingError, pipeline::pipeline_service::PipelineService};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
    repo_url: MpText<String>,
}

#[derive(Deserialize)]
struct PipelineByIDQuery {
    id: i64,
}

#[derive(Deserialize)]
struct PipelineQueryParams {
    verbose: Option<bool>,
}

#[get("/pipeline")]
pub async fn get_pipelines(
    pipeline_service: web::Data<Arc<PipelineService>>,
    query: web::Query<PipelineQueryParams>,
) -> impl Responder {
    let verbose = query.verbose.unwrap_or(false);
    let pipelines = pipeline_service.find_all(verbose).await;
    HttpResponse::Ok().json(pipelines)
}

#[get("/pipeline/{id}")]
pub async fn get_pipeline(
    path: web::Path<PipelineByIDQuery>,
    pipeline_service: web::Data<Arc<PipelineService>>,
    query: web::Query<PipelineQueryParams>,
) -> impl Responder {
    let id = path.id;
    let verbose = query.verbose.unwrap_or(false);
    info!("Fetching pipeline with id: {}, verbose: {}", id, verbose);
    let pipeline = pipeline_service.find(i64::from(id), verbose).await;
    match pipeline {
        Some(p) => HttpResponse::Ok().json(p),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/pipeline")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    pipeline_service: web::Data<Arc<PipelineService>>,
) -> impl Responder {
    info!(
        "Uploaded file {} with repository {}",
        form.file.size,
        form.repo_url.as_str()
    );
    let repo_url = form.repo_url.as_str();
    let f = form.file;
    let file_name = match f.file_name {
        Some(file_name) => file_name,
        None => return HttpResponse::UnprocessableEntity().body("Invalid file name"),
    };
    let _path = format!("./tmp/{}", file_name);
    let mut fd_manifest = match f.file.reopen() {
        Ok(fd) => fd,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let mut buffer = String::new();
    match fd_manifest.read_to_string(&mut buffer) {
        Err(e) => {
            if e.kind() == std::io::ErrorKind::InvalidData {
                return HttpResponse::UnprocessableEntity().body("Invalid data");
            }
        }
        Ok(_) => {
            println!("Read {} bytes", buffer);
        }
    }

    match pipeline_service.try_parse_pipeline(buffer) {
        Ok(workflow) => {
            if let Ok(pipeline) = pipeline_service
                .create_pipeline_with_actions(workflow, repo_url.to_string())
                .await
            {
                for action in &pipeline.actions {
                    info!("Sending action: {:?}", action);
                    pipeline_service
                        .send_action(Arc::new(action.clone()), repo_url.to_string())
                        .await
                        .unwrap();
                }
                return HttpResponse::Ok().json(pipeline);
            } else {
                info!("Error while creating pipeline");
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(ParsingError::YamlNotCompliant) => HttpResponse::BadRequest().body("Invalid yaml"),
        Err(err) => HttpResponse::BadRequest().body(format!("{:?}", err)), //TODO: replace this by exhaustive match
    }
}
