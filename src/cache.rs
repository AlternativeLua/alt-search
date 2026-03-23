use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use lz4_flex;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub modified: u64,
    pub created: u64,
    pub is_dir: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    entries: HashMap<String, FileEntry>
}

impl Cache {
    pub fn new() -> Cache {
        let entries = HashMap::new();
        Cache { entries }
    }

    pub fn build(&mut self, root: &Path, depth: usize) -> std::io::Result<usize> {
        let walker = WalkDir::new(root).min_depth(1).max_depth(depth);

        for entry in walker {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let file_entry = FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                extension: entry.path().extension().map(|e| e.to_string_lossy().to_string()),
                size: metadata.len(),
                modified: metadata.modified()
                    .ok()
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
                created: metadata.created()
                    .ok()
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
                is_dir: entry.file_type().is_dir(),
            };

            self.entries.insert(entry.path().to_string_lossy().to_string(), file_entry);
        }

        Ok(self.entries.len())
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let bytes = postcard::to_allocvec(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let compressed = lz4_flex::compress(&bytes);
        std::fs::write(path, compressed)
    }

    pub fn load(path: &Path) -> std::io::Result<Cache> {
        let bytes = std::fs::read(path)?;
        let decompressed = lz4_flex::decompress(&bytes, 0).unwrap();
        postcard::from_bytes::<Cache>(&decompressed)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn iter(&self) -> impl Iterator<Item = &FileEntry> {
        self.entries.values()
    }
}
