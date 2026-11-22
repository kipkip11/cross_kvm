use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info, warn};

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub tag_name: String,
    pub html_url: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Deserialize, Debug)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct AutoUpdater {
    repo: &'static str,
    current_version: &'static str,
}

impl AutoUpdater {
    pub fn new() -> Self {
        Self {
            repo: "kipkip11/cross_kvm",
            current_version: env!("CARGO_PKG_VERSION"),
        }
    }

    /// Kiểm tra phiên bản mới từ GitHub Releases
    pub async fn check_for_updates(&self) -> anyhow::Result<Option<GithubRelease>> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.repo);

        info!(
            "🔍 Kiểm tra cập nhật từ GitHub: {} (Hiện tại: v{})",
            self.repo, self.current_version
        );

        let output = std::process::Command::new("curl")
            .arg("-s")
            .arg("-H")
            .arg("User-Agent: cross_kvm-updater")
            .arg(&url)
            .output()?;

        if !output.status.success() {
            warn!("Không thể kết nối đến GitHub Releases API.");
            return Ok(None);
        }

        let body = String::from_utf8_lossy(&output.stdout);
        if let Ok(release) = serde_json::from_str::<GithubRelease>(&body) {
            let latest_ver = release.tag_name.trim_start_matches('v');
            if latest_ver != self.current_version {
                info!(
                    "🎉 Phát hiện phiên bản mới: {} -> {}",
                    self.current_version, release.tag_name
                );
                return Ok(Some(release));
            } else {
                info!(
                    "✅ Ứng dụng đang ở phiên bản mới nhất (v{}).",
                    self.current_version
                );
            }
        }

        Ok(None)
    }

    /// Tải về và thực hiện atomic swap binary
    pub async fn apply_update(&self, release: &GithubRelease) -> anyhow::Result<()> {
        let target_os = env::consts::OS;
        let expected_asset = match target_os {
            "macos" => "cross_kvm-macos",
            "windows" => "cross_kvm-windows.exe",
            "linux" => "cross_kvm-linux",
            _ => {
                return Err(anyhow::anyhow!(
                    "Hệ điều hành không hỗ trợ tự động cập nhật"
                ))
            }
        };

        let asset = release
            .assets
            .iter()
            .find(|a| a.name.contains(expected_asset))
            .ok_or_else(|| {
                anyhow::anyhow!("Không tìm thấy gói cài đặt tương ứng: {}", expected_asset)
            })?;

        info!(
            "⬇️ Đang tải bản cập nhật từ: {}",
            asset.browser_download_url
        );

        let current_exe = env::current_exe()?;
        let temp_exe = current_exe.with_extension("download");

        let status = std::process::Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&temp_exe)
            .arg(&asset.browser_download_url)
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Tải file thất bại."));
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&temp_exe, fs::Permissions::from_mode(0o755))?;
        }

        // Atomic replace
        let backup_exe = current_exe.with_extension("old");
        let _ = fs::remove_file(&backup_exe);
        fs::rename(&current_exe, &backup_exe)?;
        fs::rename(&temp_exe, &current_exe)?;

        info!(
            "✅ Cập nhật thành công lên phiên bản {}! Vui lòng khởi động lại ứng dụng.",
            release.tag_name
        );
        Ok(())
    }
}
