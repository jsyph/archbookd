use archbookd_error::{ArchbookDError, ArchbookDResult};
use tokio::{fs, process::Command};

const SYSTEMD_SERVICE_DIRECTORY: &str = "/etc/systemd/system";

fn systemd_service_path(name: &str) -> String {
    format!("{}/{}", SYSTEMD_SERVICE_DIRECTORY, name)
}

pub async fn create_activate_service(name: &str, content: &str) -> ArchbookDResult<()> {
    fs::write(systemd_service_path(name), content).await?;

    if !Command::new("systemctl")
        .arg("enable")
        .arg(name.to_string())
        .arg("--now")
        .spawn()?
        .wait()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlEnable(name.to_string()));
    }

    Ok(())
}

pub async fn nuke_active_service(name: &str) -> ArchbookDResult<()> {
    fs::remove_file(systemd_service_path(name)).await?;

    if !Command::new("systemctl")
        .arg("daemon-reload")
        .spawn()?
        .wait()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlDaemonReload);
    }

    Ok(())
}

pub async fn disable_active_service(name: &str) -> ArchbookDResult<()> {
    if !Command::new("systemctl")
        .arg("disable")
        .arg(name.to_string())
        .spawn()?
        .wait()
        .await?
        .success()
    {
        return Err(ArchbookDError::SystemCtlDisable(name.to_string()));
    }

    Ok(())
}