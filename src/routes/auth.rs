use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    models::{AppState, AuthReq, AuthRes, ErrRes},
    util,
};

#[post("/auth")]
pub async fn auth(req: web::Json<AuthReq>, data: web::Data<AppState>) -> impl Responder {
    if data.password.is_none() {
        return HttpResponse::Ok().json(AuthRes { key: None });
    }

    if req.password != data.password.clone().unwrap() {
        return HttpResponse::BadRequest().json(ErrRes::from_static(
            "incorrectPasswordErr",
            "Incorrect password",
        ));
    }

    let key = util::get_uuid();
    let mut auth_keys = data.auth_keys.lock().unwrap();
    auth_keys.push(key.clone());
    drop(auth_keys);

    HttpResponse::Ok().json(AuthRes { key: Some(key) })
}
