use tauri::command;
use zettel_engine::{create_note, list_notes, read_note, extract_links};

#[tauri::command]
fn create_new_note(title: String, content: String) -> Result<String, String> {
    create_note(&title, &content).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_all_notes() -> Result<Vec<String>, String> {
    list_notes().map_err(|e| e.to_string())
}

#[tauri::command]
fn read_note_by_id(id: String) -> Result<String, String> {
    read_note(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn extract_links_from_note(content: String) -> Vec<String> {
    extract_links(&content)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_new_note,
            list_all_notes,
            read_note_by_id,
            extract_links_from_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
