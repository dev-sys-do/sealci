use std::{borrow::Borrow, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use tracing::info;

use crate::parser::pipe_parser::{ManifestParser, MockManifestParser, ParsingError};

#[derive(Debug, MultipartForm)]
struct UploadPipelineForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/pipeline")]
pub async fn create_pipeline(
    MultipartForm(form): MultipartForm<UploadPipelineForm>,
    parser: web::Data<Arc<MockManifestParser>>,
) -> impl Responder {
    let result = parser.parse("".to_string()).expect("c'est mocké");
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        info!(path);
    }
    HttpResponse::Ok().json(result)
}
