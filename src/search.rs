use std::time::SystemTime;
use crate::cache::{Cache, FileEntry};

pub struct Query {
    pub name_contains: Option<String>,
    pub extension: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub modified_after: Option<u64>,
    pub is_dir: Option<bool>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            name_contains: None,
            extension: None,
            min_size: None,
            max_size: None,
            modified_after: None,
            is_dir: None,
        }
    }
}

pub fn search<'a>(cache: &'a Cache, query: &Query) -> Vec<&'a FileEntry> {
    cache.iter()
        .filter(|entry| matches_name(entry, query))
        .filter(|entry| matches_extension(entry, query))
        .filter(|entry| matches_size(entry, query))
        .filter(|entry| matches_modified(entry, query))
        .filter(|entry| matches_is_dir(entry, query))
        .collect()
}

fn matches_name(entry: &FileEntry, query: &Query) -> bool {
    match &query.name_contains {
        Some(name) => entry.name.to_lowercase().contains(&name.to_lowercase()),
        None => true,
    }
}

fn matches_extension(entry: &FileEntry, query: &Query) -> bool {
    match (&query.extension, &entry.extension) {
        (Some(q_ext), Some(e_ext)) => e_ext.to_lowercase() == q_ext.to_lowercase(),
        (Some(_), None) => false,
        (None, _) => true,
    }
}

fn matches_size(entry: &FileEntry, query: &Query) -> bool {
    if let Some(min) = query.min_size {
        if entry.size < min { return false; }
    }
    if let Some(max) = query.max_size {
        if entry.size > max { return false; }
    }
    true
}

fn matches_modified(entry: &FileEntry, query: &Query) -> bool {
    match query.modified_after {
        Some(after) => entry.modified > after,
        None => true,
    }
}

fn matches_is_dir(entry: &FileEntry, query: &Query) -> bool {
    match query.is_dir {
        Some(is_dir) => entry.is_dir == is_dir,
        None => true,
    }
}