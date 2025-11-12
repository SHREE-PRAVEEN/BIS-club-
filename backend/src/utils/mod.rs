pub fn is_valid_image_type(content_type: &str) -> bool {
    matches!(
        content_type,
        "image/jpeg"
            | "image/png"
            | "image/gif"
            | "image/webp"
            | "image/svg+xml"
            | "image/x-icon"
            | "image/bmp"
    )
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    filename.rsplit('.').next()
}

pub fn validate_filename(filename: &str) -> bool {
    // Basic filename validation
    !filename.is_empty() && filename.len() <= 255 && !filename.contains('\0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_image_types() {
        assert!(is_valid_image_type("image/jpeg"));
        assert!(is_valid_image_type("image/png"));
        assert!(is_valid_image_type("image/gif"));
        assert!(is_valid_image_type("image/webp"));
        assert!(!is_valid_image_type("application/pdf"));
        assert!(!is_valid_image_type("text/plain"));
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(get_file_extension("image.jpg"), Some("jpg"));
        assert_eq!(get_file_extension("photo.png"), Some("png"));
        assert_eq!(get_file_extension("noextension"), Some("noextension"));
    }

    #[test]
    fn test_filename_validation() {
        assert!(validate_filename("image.jpg"));
        assert!(validate_filename("my_photo.png"));
        assert!(!validate_filename(""));
        assert!(!validate_filename("file\0.jpg"));
    }
}
