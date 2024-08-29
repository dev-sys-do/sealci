use std::{f32::consts::PI, io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text as MpText, MultipartForm};
use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
use serde::Deserialize;
use tracing::info;

use crate::{
    parser::pipe_parser::{ManifestParser, MockManifestParser, ParsingError},
    pipeline::pipeline_service::PipelineService,
};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
    repo_url: MpText<String>,
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
        Ok(pipeline) => match pipeline_service
            .send_actions(pipeline, repo_url.to_string())
            .await
        {
            Ok(_) => HttpResponse::Created().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(ParsingError::YamlNotCompliant) => {
            HttpResponse::UnprocessableEntity().body("Invalid yaml")
        }
        Err(err) => HttpResponse::UnprocessableEntity().body(format!("{:?}", err)), //TODO: replace this by exhaustive match
    }
}
