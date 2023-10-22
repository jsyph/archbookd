use archbookd_error::ArchbookDResult;
use files::{
    BLACKLIST_PATH, MODESET_PATH, NVIDIA_XORG_CONFIG_PATH, SDDM_XSETUP_BACKUP_PATH,
    SDDM_XSETUP_CONTENT, SDDM_XSETUP_PATH, UDEV_INTEGRATED_PATH, UDEV_PM_PATH, XORG_PATH,
};
use service_utils::disable_active_service;
use tokio::fs;

mod files;

const FILES_TO_CLEANUP: [&str; 6] = [
    BLACKLIST_PATH,
    UDEV_INTEGRATED_PATH,
    UDEV_PM_PATH,
    XORG_PATH,
    NVIDIA_XORG_CONFIG_PATH,
    MODESET_PATH,
];

async fn cleanup() -> ArchbookDResult<()> {
    for path in FILES_TO_CLEANUP {
        if fs::remove_file(path).await.is_err() {
            println!("Failed to remove {}", path);
        }
    }

    if fs::metadata(SDDM_XSETUP_BACKUP_PATH).await.is_ok() {
        
    }

    Ok(())
}

pub async fn switch_to_integrated() -> ArchbookDResult<()> {
    disable_active_service("nvidia-persistenced.service").await?;

    Ok(())
}
