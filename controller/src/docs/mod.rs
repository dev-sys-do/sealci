use actix_web::{get, HttpResponse, Responder};
use scalar_doc::scalar_actix::ActixDocumentation;

#[get("/openapi")]
pub async fn openapi() -> impl Responder {
    let open = include_str!("../../../api/openapi/controller/controller.openapi.yaml");
    HttpResponse::Ok().body(open)
}

#[get("/docs")]
pub async fn doc() -> impl Responder {
    ActixDocumentation::new("SealCI - Open API", "/openapi")
        .theme(scalar_doc::Theme::Kepler)
        .service()
}
