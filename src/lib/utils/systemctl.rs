use crate::error::{ArchbookDError, ArchbookDResult};
use tokio::{fs, process::Command};

#[cfg(not(debug_assertions))]
const SYSTEMD_SERVICE_DIRECTORY: &str = "/etc/systemd/system";

#[cfg(debug_assertions)]
const SYSTEMD_SERVICE_DIRECTORY: &str = "./lib_test/etc/systemd/system";

fn systemd_service_path(name: &str) -> String {
    format!("{}/{}", SYSTEMD_SERVICE_DIRECTORY, name)
}

/// Runs `systemctl  enable SERVICE --now`
pub async fn systemctl_enable_now(name: &str) -> ArchbookDResult<()> {
    if !Command::new("systemctl")
        .arg("enable")
        .arg(name.to_string())
        .arg("--now")
        .status()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlEnable(name.to_string()));
    }

    Ok(())
}

/// Runs `systemctl daemon-reload`
pub async fn systemctl_daemon_reload() -> ArchbookDResult<()> {
    if !Command::new("systemctl")
        .arg("daemon-reload")
        .status()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlDaemonReload);
    }
    Ok(())
}

/// Runs `systemctl disable SERVICE`
pub async fn systemctl_disable(name: &str) -> ArchbookDResult<()> {
    if !Command::new("systemctl")
        .arg("disable")
        .arg(name.to_string())
        .status()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlDisable(name.to_string()));
    }

    Ok(())
}

/// Create file with `name` and `content` in `/etc/systemd/system` directory
pub async fn create_in_systemd_directory(name: &str, content: &str) -> ArchbookDResult<()> {
    fs::write(systemd_service_path(name), content).await?;
    Ok(())
}

/// Deletes service from `/etc/systemd/system` directory
pub async fn remove_from_systemd_directory(name: &str) -> ArchbookDResult<()> {
    fs::remove_file(systemd_service_path(name)).await?;

    Ok(())
}
