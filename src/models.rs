use std::sync::Mutex;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub path: String,
    pub version: String,
    pub password: Option<String>,
    pub auth_keys: Mutex<Vec<String>>,
    pub lock_path: bool,
}

#[derive(Deserialize)]
pub struct AuthReq {
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthRes {
    pub key: Option<String>,
}

#[derive(Deserialize)]
pub struct GetFilesReq {
    pub path: String,
}

#[derive(Deserialize)]
pub struct GetFileReq {
    pub path: String,
    pub view: Option<String>,
}

#[derive(Serialize)]
pub struct GetFilesRes {
    pub files: Vec<File>,
}

#[derive(Serialize)]
pub enum FileType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "dir")]
    Directory,
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub modified: Option<u64>,
    pub created: Option<u64>,
    pub size: Option<u64>,
    pub file_type: FileType,
    pub path: String,
}

#[derive(Serialize)]
pub struct GetInfoRes {
    pub version: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct ErrRes {
    pub code: String,
    pub error: String,
}

impl ErrRes {
    pub fn from_static(code: &'static str, error: &'static str) -> Self {
        ErrRes {
            code: String::from(code),
            error: String::from(error),
        }
    }
}

#[derive(Parser)]
#[clap(version, author = "sexnine")]
pub struct Args {
    #[clap(long, short = 'P', help = "Sets a password (highly recommended)")]
    pub password: Option<String>,

    // #[clap(long, help = "Enables SSL")]
    // pub ssl: bool,
    #[clap(long, short, help = "(default 3030)")]
    pub port: Option<u16>,

    #[clap(long = "bind", short = 'b', help = "IP to bind to (default local IP)")]
    pub bind: Option<String>,
    // #[clap(
    //     long = "any",
    //     short = 'a',
    //     help = "Downcat will serve any files it has permissions to"
    // )]
    // pub not_lock_dir: bool,
}
