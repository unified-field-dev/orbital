/// Whether a MIME type represents an inline image preview.
pub fn is_image_mime(mime: Option<&str>) -> bool {
    mime.map(|m| m.starts_with("image/")).unwrap_or(false)
}

/// Human-readable file size for download links.
pub fn format_file_size(bytes: Option<u64>) -> Option<String> {
    let bytes = bytes?;
    if bytes < 1024 {
        Some(format!("{bytes} B"))
    } else if bytes < 1024 * 1024 {
        Some(format!("{:.1} KB", bytes as f64 / 1024.0))
    } else {
        Some(format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::{format_file_size, is_image_mime};

    #[test]
    fn detects_image_mime() {
        assert!(is_image_mime(Some("image/png")));
        assert!(!is_image_mime(Some("application/pdf")));
    }

    #[test]
    fn formats_sizes() {
        assert_eq!(format_file_size(Some(512)), Some("512 B".to_string()));
        assert_eq!(format_file_size(Some(2048)), Some("2.0 KB".to_string()));
    }
}
