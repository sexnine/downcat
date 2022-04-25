use actix_web::{get, web, HttpResponse, Responder};

use crate::models::{AppState, GetInfoRes};

#[get("/info")]
pub async fn get_info(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(GetInfoRes {
        version: data.version.clone(),
        path: data.path.clone(),
    })
}
