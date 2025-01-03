use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
 use rand::RngCore;

/// Create a test file with specific content and size
pub fn create_test_file(path: &Path, size: usize) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut buffer = vec![0u8; 1024]; // 1KB chunks
    let mut remaining = size;

    while remaining > 0 {
        rand::thread_rng().fill_bytes(&mut buffer);
        let write_size = std::cmp::min(remaining, buffer.len());
        file.write_all(&buffer[..write_size])?;
        remaining -= write_size;
    }

    file.sync_all()?;
    Ok(())
}

/// Try to read file content after deletion
/// Returns true if any content was readable
pub fn try_read_deleted_file(path: &Path) -> io::Result<bool> {
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(!buffer.is_empty())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// Helper function to verify pattern overwrite
pub fn verify_pattern_overwrite(path: &Path, pattern: &[u8]) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; pattern.len()];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        if &buffer[..n] != &pattern[..n] {
            return Ok(false);
        }
    }

    Ok(true)
}
