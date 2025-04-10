pub fn extract_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut remaining = content;

    while let Some(start) = remaining.find("[[") {
        let rest = &remaining[start + 2..];
        if let Some(end) = rest.find("]]") {
            let id = &rest[..end];
            links.push(id.to_string());
            remaining = &rest[end + 2..];
        } else {
            break;
        }
    }

    links
}
