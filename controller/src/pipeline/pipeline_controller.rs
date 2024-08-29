use std::{f32::consts::PI, io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
use tracing::info;

use crate::{
    parser::pipe_parser::{ManifestParser, MockManifestParser, ParsingError},
    pipeline::pipeline_service::PipelineService,
};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
}

#[post("/pipeline")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    pipeline_service: web::Data<Arc<PipelineService>>,
) -> impl Responder {
    info!("Uploaded file {}", form.file.size);
    let f = form.file;
    let _path = format!("./tmp/{}", f.file_name.unwrap());
    let mut fd_manifest = f.file.reopen().unwrap(); //TODO: handle this error
    let mut buffer = String::new();
    match fd_manifest.read_to_string(&mut buffer) {
        Err(e) => {
            if e.kind() == std::io::ErrorKind::InvalidData {
                return HttpResponse::BadRequest().body("Invalid data");
            }
        }
        Ok(_) => {
            println!("Read {} bytes", buffer);
        }
    }

    match pipeline_service.try_parse_pipeline(buffer) {
        Ok(pipeline) => match pipeline_service.send_actions(pipeline).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(ParsingError::YamlNotCompliant) => HttpResponse::BadRequest().body("Invalid yaml"),
        Err(err) => HttpResponse::BadRequest().body(format!("{:?}", err)), //TODO: replace this by exhaustive match
    }
}
