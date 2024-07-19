use std::{borrow::Borrow, fmt::Display, io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::StatusCode, post, web, HttpResponse, Responder};
use tracing::info;

use crate::parser::pipe_parser::{ManifestParser, MockManifestParser, ParsingError};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "body")]
    file: TempFile,
}

#[post("/pipeline")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    parser: web::Data<Arc<MockManifestParser>>, //TODO: replace with the real implementation
) -> impl Responder {
    let result = parser.parse("".to_string()).expect("c'est mocké");
    info!("Uploaded file {}", form.file.size);
    let f = form.file;
    let path = format!("./tmp/{}", f.file_name.unwrap());
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

    HttpResponse::Ok().status(StatusCode::CREATED).json(result)
}
