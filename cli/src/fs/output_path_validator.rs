use std::path::PathBuf;

pub fn validate_output_file(user_input: impl AsRef<str>) -> Result<PathBuf, String> {
    let path = PathBuf::from(user_input.as_ref());

    if path.is_dir() {
        return Err(format!(
            "`{}`is directory, It should be file",
            path.to_str().unwrap()
        ));
    }

    let parent = path.parent().ok_or_else(|| {
        "Invalid output path: no parent directory (e.g. root or malformed)".to_string()
    })?;

    // create parent directories
    if let Err(e) = std::fs::create_dir_all(parent) {
        return Err(format!(
            "Failed to create parent directories for '{}': {}",
            parent.display(),
            e
        ));
    }

    // prevents override already exists burn.json
    if path.exists() {
        return Err(format!(
            "Output file already exists: '{}'. Refusing to overwrite.",
            path.display()
        ));
    }

    return Ok(path);
}
