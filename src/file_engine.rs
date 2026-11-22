use anyhow::{anyhow, Result};
use directories::UserDirs;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct FileEngine {
    drop_dir: PathBuf,
}

impl FileEngine {
    pub fn new() -> Result<Self> {
        let user_dirs = UserDirs::new().ok_or_else(|| anyhow!("Cannot locate user directories"))?;
        let drop_dir = user_dirs
            .download_dir()
            .unwrap_or(&user_dirs.home_dir().join("Downloads"))
            .join("LanDrop");

        if !drop_dir.exists() {
            fs::create_dir_all(&drop_dir)?;
        }

        Ok(Self { drop_dir })
    }

    pub fn sanitize_file_name(raw_name: &str) -> String {
        let file_name = Path::new(raw_name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("received_file.bin");

        // Loại trừ ký tự lạ và path traversal
        file_name
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
            .collect()
    }

    pub fn save_chunk(&self, file_name: &str, chunk: &[u8], is_first: bool) -> Result<PathBuf> {
        let clean_name = Self::sanitize_file_name(file_name);
        let target_path = self.drop_dir.join(&clean_name);

        let mut file = if is_first {
            File::create(&target_path)?
        } else {
            fs::OpenOptions::new().append(true).open(&target_path)?
        };

        file.write_all(chunk)?;
        Ok(target_path)
    }

    pub fn verify_checksum(path: &Path, expected_hash: &str) -> Result<bool> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let calculated = format!("{:x}", hasher.finalize());
        Ok(calculated.eq_ignore_ascii_case(expected_hash))
    }
}
