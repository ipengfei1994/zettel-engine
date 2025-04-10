pub mod models;
pub mod io;
pub mod link;

pub use io::{create_note, list_notes, read_note};
pub use link::extract_links;

