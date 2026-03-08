use std::fs;
use std::path::Path;

use crate::infra::errors::{AppError, AppResult};
use crate::infra::paths;
use crate::infra::persistence::Persistence;

const DEFAULT_MGS_LIST: &str = include_str!("../../assets/default-mgs-list.txt");

pub fn ensure_data_files(persistence: &Persistence) -> AppResult<()> {
    copy_legacy_if_missing(persistence.memory_path(), "memory.txt")?;
    copy_legacy_if_missing(persistence.mgs_list_path(), "MGSList.txt")?;

    if !persistence.memory_path().exists() {
        persistence.save_memory_text("")?;
    }

    if !persistence.mgs_list_path().exists() {
        persistence.save_mgs_list_text(DEFAULT_MGS_LIST)?;
    }

    Ok(())
}

fn copy_legacy_if_missing(target: &Path, file_name: &str) -> AppResult<()> {
    if target.exists() {
        return Ok(());
    }

    for dir in paths::legacy_search_dirs() {
        let candidate = dir.join(file_name);
        if candidate.exists() {
            fs::copy(&candidate, target).map_err(|error| AppError::Io(error.to_string()))?;
            return Ok(());
        }
    }

    Ok(())
}