use std::{fs, path::Path, io};
use crate::models::NoteMeta;
use chrono::Local;

pub fn create_note(title: &str, content: &str) -> io::Result<String> {
    let id = Local::now().format("%Y%m%d%H%M").to_string();
    let meta = NoteMeta {
        id: id.clone(),
        title: title.to_string(),
        created: Local::now().format("%Y-%m-%d").to_string(),
        tags: Some(Vec::new()),
    };

    let yaml = serde_yaml::to_string(&meta).unwrap();
    let note_dir = Path::new("zettel");
    fs::create_dir_all(&note_dir)?;
    let file_path = note_dir.join(format!("{}.md", id));
    let full = format!("---\n{}---\n\n{}", yaml, content);
    fs::write(file_path, full)?;
    Ok(id)
}

pub fn list_notes() -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let dir = Path::new("zettel");
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                result.push(stem.to_string());
            }
        }
    }
    Ok(result)
}

pub fn read_note(id: &str) -> io::Result<String> {
    let path = Path::new("zettel").join(format!("{}.md", id));
    fs::read_to_string(path)
}
pub fn search_notes(query: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let dir = Path::new("zettel");
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if let Some(content) = fs::read_to_string(&path).ok() {
            if content.contains(query) {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    result.push(stem.to_string());
                }
            }
        }
    }
    Ok(result)
}

