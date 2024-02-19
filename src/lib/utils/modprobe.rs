use std::path::{Path, PathBuf};

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

use crate::error::{ArchbookDError, ArchbookDResult};

/// run `modprobe [name]`
pub async fn modprobe_enable(name: &str) -> ArchbookDResult<()> {
    if !Command::new("modprobe").arg(name).status().await?.success() {
        return Err(ArchbookDError::ModprobeEnable(name.to_string()));
    }
    Ok(())
}

/// run `modprobe -r [name]`
pub async fn modprobe_disable(name: &str) -> ArchbookDResult<()> {
    if !Command::new("modprobe -r")
        .arg(name)
        .status()
        .await?
        .success()
    {
        return Err(ArchbookDError::ModprobeDisable(name.to_string()));
    }
    Ok(())
}

// #[cfg(not(debug_assertions))]
// const MODPROBE_CONFIG_DIR_PATH: &str = "/etc/modprobe.d/";
// #[cfg(debug_assertions)]
// const MODPROBE_CONFIG_DIR_PATH: &str = "./etc/modprobe.d/";
//
// pub struct BlacklistFile {
//     file_name: String,
//     file: fs::File,
// }
//
// impl BlacklistFile {
//
//     fn path(file_name: String) -> PathBuf {
//         PathBuf::from(format!(
//             "{}{}.conf",
//             MODPROBE_CONFIG_DIR_PATH, file_name
//         ))
//     }
//
//
//     async fn open(&self) -> ArchbookDResult<fs::File> {
//         Ok(fs::File::create(self.path()).await?)
//     }
//
//     pub async fn add(&self, module: &str) -> ArchbookDResult<()> {
//         
//         Ok(())
//     }
// }
