use std::{io::Read, sync::Arc};

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::info;

use crate::{application::ports::{pipeline_service::PipelineService, scheduler_service::SchedulerService}, domain::entities::pipeline::ManifestPipeline};

#[get("/pipelines")]
pub async fn get_pipelines(
  service: web::Data<Arc<Box<dyn PipelineService + Send + Sync>>>
) -> impl Responder {
  let pipelines = service.find_all().await;

  HttpResponse::Ok().json(pipelines)
}

#[get("/pipelines/{pipeline_id}")]
pub async fn get_pipeline(
  service: web::Data<Arc<dyn PipelineService + Send + Sync>>,
  pipeline_id: web::Path<i64>
) -> impl Responder {
  let pipeline = service.find_by_id(pipeline_id.into_inner()).await;

  match pipeline {
    Ok(pipeline) => HttpResponse::Ok().json(pipeline),
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

#[derive(Debug, MultipartForm)]
pub struct UploadPipelineForm {
  #[multipart(rename = "body")]
  file: TempFile,
  repo_url: Text<String>
}

#[post("/pipelines")]
pub async fn create_pipeline(
  MultipartForm(form): MultipartForm<UploadPipelineForm>,
  pipeline_service: web::Data<Arc<Box<dyn PipelineService + Send + Sync>>>,
  scheduler_service: web::Data<Arc<Box<dyn SchedulerService + Send + Sync>>>
) -> impl Responder {
  let repo_url = form.repo_url.into_inner();
  let temp_file = form.file;

  let mut file_content = Vec::new();
  let mut file = temp_file.file;
  if let Err(_) = file.read_to_end(&mut file_content) {
    return HttpResponse::BadRequest().finish();
  }

  let manifest_pipeline: Result<ManifestPipeline, serde_yaml::Error> = serde_yaml::from_slice(&file_content);

  match manifest_pipeline {
    Ok(manifest_pipeline) => {

      info!("Creating pipeline: {:?}", manifest_pipeline);


      let pipeline = pipeline_service.create_manifest_pipeline(manifest_pipeline, repo_url)
      .await;

      match pipeline {
        Ok(pipeline) => {
          let schedule = scheduler_service.execute_pipeline(pipeline.id).await;
          if let Err(_) = schedule {
            return HttpResponse::BadRequest().finish();
          }
          return HttpResponse::Ok().json(pipeline);
        },
        Err(_) => {
          return HttpResponse::BadRequest().finish();
        }
      }

    },
    Err(_) => {
      return HttpResponse::BadRequest().finish();
    }
  }
}