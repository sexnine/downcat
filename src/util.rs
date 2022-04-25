use std::path::Component;
use std::path::Path;
use std::time::SystemTime;
use std::time::SystemTimeError;

use uuid::Uuid;

use crate::models::AppState;

pub fn get_uuid() -> String {
    blob_uuid::to_blob(&Uuid::new_v4()).to_string()
}

pub fn system_time_to_unix_timestamp(time: SystemTime) -> Result<u64, SystemTimeError> {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(i) => Ok(i.as_secs()),
        Err(e) => Err(e),
    }
}

pub fn check_path(path: &Path, state: &AppState) -> bool {
    if path
        .components()
        .into_iter()
        .any(|x| x == Component::ParentDir)
    {
        return false;
    }
    let parent = Path::new(&state.path);
    if state.lock_path {
        let mut sus = path.ancestors();
        while let Some(i) = sus.next() {
            if i == parent {
                return true;
            }
        }
        return false;
    }
    return false;
}
