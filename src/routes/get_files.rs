use std::path::Path;

use actix_web::{post, web, HttpResponse, Responder};

use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

use crate::{
    models::{AppState, ErrRes, File, FileType, GetFilesReq, GetFilesRes},
    util,
};

#[post("/get_files")]
pub async fn get_files(req: web::Json<GetFilesReq>, data: web::Data<AppState>) -> impl Responder {
    let path = Path::new(&req.path);
    if !util::check_path(&path, &data) {
        return HttpResponse::BadRequest()
            .json(ErrRes::from_static("readDirErr", "Error reading directory"));
    }
    let mut dir_stream = ReadDirStream::new(match fs::read_dir(path).await {
        Ok(x) => x,
        _ => {
            return HttpResponse::BadRequest()
                .json(ErrRes::from_static("readDirErr", "Error reading directory"))
        }
    });

    let mut files: Vec<File> = Vec::new();

    while let Some(i) = dir_stream.next().await {
        let entry = match i {
            Ok(i) => i,
            _ => continue,
        };
        let file_type = entry.file_type().await.expect("Error getting type of file"); // TODO: might not be able to get metadata
        let file_name = entry
            .file_name()
            .to_str()
            .expect("Error converting file name to str")
            .to_string();
        let path = entry
            .path()
            .to_str()
            .expect("Error converting path to str")
            .to_string();
        let metadata = entry.metadata().await;

        let modified: Option<u64>;
        let created: Option<u64>;
        let size: u64;

        if metadata.is_ok() {
            let metadata = metadata.unwrap();

            modified = match metadata.modified() {
                Ok(i) => match util::system_time_to_unix_timestamp(i) {
                    Ok(i) => Some(i),
                    _ => None,
                },
                _ => None,
            };

            created = match metadata.created() {
                Ok(i) => match util::system_time_to_unix_timestamp(i) {
                    Ok(i) => Some(i),
                    _ => None,
                },
                _ => None,
            };

            size = metadata.len();
        } else {
            modified = None;
            created = None;
            size = 0;
        }

        if file_type.is_dir() || file_type.is_file() {
            files.push(File {
                name: file_name,
                modified,
                created,
                path,
                size: if file_type.is_dir() { None } else { Some(size) },
                file_type: if file_type.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                },
            });
        }
    }

    return HttpResponse::Ok().json(GetFilesRes { files });
}
