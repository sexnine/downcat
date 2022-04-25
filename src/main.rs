use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    middleware::{Condition, DefaultHeaders},
    web, App, HttpResponse, HttpServer,
};
use actix_web_static_files::ResourceFiles;
use clap::StructOpt;
use colored::*;
use local_ip_address::local_ip;

use crate::models::AppState;

mod codec;
mod info;
mod middleware;
mod models;
mod routes;
mod util;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn on_ready(host: String, port: u16) {
    println!(
        "{}\n{} {}\n",
        format!("✅ Downcat v{} running!", info::version()).bright_green(),
        "✨ Listening on".bright_yellow(),
        format!("http://{host}:{port}/").bright_blue()
    );

    #[cfg(debug_assertions)]
    println!("{}\n", "🛠 CORS enabled for development".red());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = models::Args::parse();
    let port = args.port.unwrap_or(3030);
    let password = args.password;
    let host = args.bind.unwrap_or(match local_ip() {
        Ok(x) => x.to_string(),
        _ => String::from("0.0.0.0"),
    });

    println!("\n{}\n", "🐈 Starting downcat...".dimmed());

    let cwd_buf = std::env::current_dir().unwrap();
    let cwd = String::from(cwd_buf.as_path().to_str().unwrap());

    let state = web::Data::new(AppState {
        path: cwd,
        version: String::from(info::version()),
        password,
        auth_keys: Mutex::new(Vec::new()),
        // lock_path: !args.not_lock_dir,
        lock_path: true,
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .wrap(middleware::auth::Authentication)
                    .wrap(DefaultHeaders::new().add(("Connection", "Keep-Alive")))
                    .wrap(Condition::new(cfg!(debug_assertions), Cors::permissive()))
                    .route(
                        "/ping",
                        web::to(|| async { HttpResponse::Ok().body("pong!") }),
                    )
                    .service(routes::get_info)
                    .service(routes::get_files)
                    .service(routes::get_file)
                    .service(routes::auth),
            )
            .service(ResourceFiles::new("/", generate()))
    });

    match tokio::join!(
        server.bind(format!("{host}:{port}"))?.run(),
        on_ready(host, port)
    ) {
        (Err(e), _) => println!("An error occoured: {e}"),
        _ => {}
    }

    println!("{}", "\n👋 Downcat server closed\n".dimmed());

    Ok(())
}
