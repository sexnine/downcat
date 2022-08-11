use std::path::Path;

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web, HttpRequest, HttpResponse, Responder,
};

use crate::{
    models::{AppState, ErrRes, GetFileReq},
    util,
};

#[get("/get_file")]
pub async fn get_file(
    req: HttpRequest,
    query: web::Query<GetFileReq>,
    data: web::Data<AppState>,
) -> impl Responder {
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

    let file = match NamedFile::open_async(path).await {
        Ok(i) => i,
        _ => {
            return HttpResponse::BadRequest().json(ErrRes::from_static(
                "fileOpenErr",
                "Failed to open the file or invalid path",
            ))
        }
    };

    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

    let file = file.set_content_disposition(ContentDisposition {
        disposition: if *view_file {
            DispositionType::Inline
        } else {
            DispositionType::Attachment
        },
        parameters: vec![DispositionParam::Filename(file_name)],
    });

    file.into_response(&req)
}
