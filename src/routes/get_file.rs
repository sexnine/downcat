use std::path::Path;

use actix_codec::Framed;

use actix_web::{get, web, HttpResponse, Responder};

use tokio::fs;

use crate::{
    codec,
    models::{AppState, ErrRes, GetFileReq},
    util,
};

#[get("/get_file")]
pub async fn get_file(query: web::Query<GetFileReq>, data: web::Data<AppState>) -> impl Responder {
    let path = Path::new(&query.path);
    if !util::check_path(&path, &data) {
        return HttpResponse::BadRequest()
            .json(ErrRes::from_static("notFileErr", "Path is not a file"));
    }
    let view_file = &query.view.is_some();

    if !path.is_file() {
        return HttpResponse::BadRequest()
            .json(ErrRes::from_static("notFileErr", "Path is not a file"));
    }

    let file = match fs::File::open(path).await {
        Ok(i) => i,
        _ => {
            return HttpResponse::BadRequest().json(ErrRes::from_static(
                "fileOpenErr",
                "Failed to open the file or invalid path",
            ))
        }
    };

    let file_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace("\"", "%22");
    let stream = Framed::new(file, codec::WebBytesCodec {});

    return HttpResponse::Ok()
        .insert_header((
            "Content-Disposition",
            format!(
                "{}filename=\"{file_name}\"",
                if *view_file { "" } else { "attachment; " }
            ),
        ))
        .streaming(stream);
}
