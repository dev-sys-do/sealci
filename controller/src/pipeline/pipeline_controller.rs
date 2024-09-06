use actix_multipart::form::{tempfile::TempFile, text::Text as MpText, MultipartForm};
use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::{io::Read, sync::Arc};
use tracing::info;

use crate::{
    parser::pipe_parser::ParsingError, pipeline::pipeline_service::PipelineService, scheduler,
};

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

#[get("/pipelines")]
pub async fn get_pipelines(pipeline_service: web::Data<Arc<PipelineService>>) -> impl Responder {
    let pipelines = pipeline_service.find_all().await;
    HttpResponse::Ok().json(pipelines)
}

#[get("/pipeline/{id}")]
pub async fn get_pipeline(
    path: web::Path<PipelineByIDQuery>,
    pipeline_service: web::Data<Arc<PipelineService>>,
) -> impl Responder {
    let id = path.id;
    info!("Fetching pipeline with id: {}", id);
    let pipeline = pipeline_service.find(i64::from(id)).await;
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
            if let Ok(_pipeline) = pipeline_service
                .create_pipeline_with_actions(workflow, repo_url.to_string())
                .await
            {
                for action in _pipeline.actions {
                    info!("Sending action: {:?}", action);
                    pipeline_service
                        .send_action(Arc::new(action), repo_url.to_string())
                        .await
                        .unwrap();
                }
            } else {
                info!("Error while creating pipeline");
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Ok().finish()
        }
        Err(ParsingError::YamlNotCompliant) => HttpResponse::BadRequest().body("Invalid yaml"),
        Err(err) => HttpResponse::BadRequest().body(format!("{:?}", err)), //TODO: replace this by exhaustive match
    }
}
