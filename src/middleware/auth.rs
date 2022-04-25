use std::future::{ready, Ready};

use crate::models::{self};
use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use qstring::QString;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let state = req.app_data::<Data<models::AppState>>().unwrap();
        // println!("Version: {}", state.version);
        let auth_keys = state.auth_keys.lock().unwrap();

        // println!("Path: {}", req.path());
        if state.password.is_some() && req.path() != "/api/auth" {
            let key = req.headers().get("X-Api-Key");
            let query_params = QString::from(req.query_string());
            let key_p = query_params.get("key");
            if (key.is_none()
                || !vec_contains(key.unwrap().to_str().unwrap().to_string(), &auth_keys))
                && (key_p.is_none() || !vec_contains(key_p.unwrap().to_string(), &auth_keys))
            // TODO: fix this ugly ass code
            {
                drop(auth_keys);
                let (request, _pl) = req.into_parts();
                let response = HttpResponse::Unauthorized().finish().map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        }

        drop(auth_keys);
        let res = self.service.call(req);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

fn vec_contains(key: String, keys: &std::sync::MutexGuard<'_, Vec<String>>) -> bool {
    keys.iter().any(|x| x == &key)
}
