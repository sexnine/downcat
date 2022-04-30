use actix_multipart::{Multipart, MultipartError};
use actix_web::{post, web, HttpResponse, Responder};
use tokio_stream::StreamExt;

async fn bruh(mut payload: Multipart) -> Result<(), MultipartError> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
        }
    }

    Ok(())
}

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> &'static str {
    match bruh(payload).await {
        Ok(_) => "OK",
        Err(e) => {
            println!("{}", e);
            "Err"
        }
    }
}
