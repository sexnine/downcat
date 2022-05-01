use std::{fmt::Display, path::Path, vec};

use actix_multipart::{Field, Multipart};
use actix_web::{post, web, HttpResponse, Responder};
use serde_json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

use crate::{
    models::{AppState, ErrRes, UploadOptions},
    util::check_path,
};

async fn get_data(mut field: Field) -> Result<UploadOptions, ()> {
    let data: UploadOptions;
    let mut data_bytes: Vec<u8> = vec![];

    while let Some(chunk) = field.next().await {
        data_bytes.extend_from_slice(&chunk.unwrap()[..])
    }

    data = match serde_json::from_slice(&data_bytes[..]) {
        Ok(x) => x,
        _ => return Err(()),
    };

    Ok(data)
}

fn upload_error(message: impl Display) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrRes {
        code: "fileUploadError".to_string(),
        error: message.to_string(),
    })
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart, state: web::Data<AppState>) -> impl Responder {
    let mut field_keys: Vec<String> = vec![];
    let mut data: Option<UploadOptions> = None;

    'field: while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(x) => x,
            Err(e) => return upload_error(e),
        };

        // field_keys.push(field.name().to_string());
        // println!("name: {}, all: {:?}", field.name(), field_keys);
        // println!("{}", field.content_disposition());

        if field.name() == "options" {
            if data.is_some() {
                continue;
            }

            match get_data(field).await {
                Ok(x) => data = Some(x),
                _ => return upload_error("Failed to get/parse options"),
            }

            continue;
        }

        let data = match data.clone() {
            Some(x) => x,
            _ => return upload_error("No options"),
        };

        let file_name = match field.content_disposition().get_filename() {
            Some(x) => x.to_owned(),
            _ => return upload_error("No filename"),
        };

        let full_path_str = data.path + "/" + &file_name;
        // println!("full path: {}", full_path_str);
        let full_path = Path::new(&full_path_str);

        // println!("IS PATH valid: {}", check_path(full_path, &state));

        if !check_path(full_path, &state) {
            return upload_error("Invalid Path");
        }

        let mut file = match File::create(full_path).await {
            Ok(x) => x,
            Err(_) => return upload_error("Failed to create file"),
        };

        while let Some(chunk) = field.next().await {
            // println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk.unwrap()));
            match file.write_all(&chunk.unwrap()).await {
                Err(e) => {
                    println!(
                        "Aborting write.  Failed to write file '{}': {}",
                        file_name, e
                    );
                    continue 'field;
                }
                _ => {}
            }
        }
    }

    HttpResponse::Ok().body("OK")
}
