use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
    pub size: u64,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub is_dir: bool,
}

#[derive(Debug)]
pub struct Cache {
    entries: HashMap<PathBuf, FileEntry>,
}

impl Cache {
    pub fn new() -> Cache {
        let entries = HashMap::new();

        Cache { entries }
    }

    pub fn build(&mut self, root: &Path, depth: usize) -> std::io::Result<(usize)> {
        let walker = WalkDir::new(root).min_depth(1).max_depth(depth);

        for entry in walker {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let file_entry = FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: entry.path().to_path_buf(),
                extension: entry.path().extension().map(|e| e.to_string_lossy().to_string()),
                size: metadata.len(),
                modified: metadata.modified()?,
                created: metadata.created()?,
                is_dir: entry.file_type().is_dir(),
            };

            self.entries.insert(entry.path().to_path_buf(), file_entry);
        };

        Ok(self.entries.len())
    }

    pub fn iter(&self) -> impl Iterator<Item = &FileEntry> {
        self.entries.values()
    }
}