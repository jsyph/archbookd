use crate::error::{ArchbookDError, ArchbookDResult};
use crate::utils::systemctl::{systemctl_disable, systemctl_enable_now};
use files::{
    BLACKLIST_CONTENT, BLACKLIST_PATH, MODESET_PATH, MODESET_RTD3, NVIDIA_XORG_CONFIG_PATH,
    NVIDIA_XRANDR_SCRIPT, SDDM_XSETUP_BACKUP_PATH, SDDM_XSETUP_CONTENT, SDDM_XSETUP_PATH,
    UDEV_INTEGRATED, UDEV_INTEGRATED_PATH, UDEV_PM_CONTENT, UDEV_PM_PATH, XORG_INTEL, XORG_PATH,
};
use radix_fmt::radix;
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use std::str;
use tokio::{fs, process::Command};

mod files;

const CURRENT_ENVYCONTROL_VERSION_NAME: &str = "v3.3.1";

#[cfg(not(test))]
const GITHUB_API_ENDPOINT: &str =
    "https://api.github.com/repos/bayasdev/envycontrol/releases/latest";
#[cfg(test)]
const GITHUB_API_ENDPOINT: &str = "http://localhost/8080";

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
        fs::write(
            SDDM_XSETUP_PATH,
            fs::read_to_string(SDDM_XSETUP_BACKUP_PATH).await?,
        )
        .await?;
        fs::remove_file(SDDM_XSETUP_BACKUP_PATH).await?;
    }

    Ok(())
}

/// Switch graphics mode to use integrated
pub async fn switch_to_integrated() -> ArchbookDResult<()> {
    // disable nvidia persistence service
    systemctl_disable("nvidia-persistenced.service").await?;

    // cleanup residue files
    cleanup().await?;

    // blacklist all nvidia open source and closed source drivers
    fs::write(BLACKLIST_PATH, BLACKLIST_CONTENT).await?;

    // power off nvidia gpu with udev rules
    fs::write(UDEV_INTEGRATED_PATH, UDEV_INTEGRATED).await?;

    Ok(())
}

/// Switch graphics to hybrid, run apps on dedicated graphics card using prime-run
pub async fn switch_to_hybrid() -> ArchbookDResult<()> {
    cleanup().await?;

    systemctl_enable_now("nvidia-persistenced.service").await?;

    fs::write(MODESET_PATH, MODESET_RTD3).await?;
    fs::write(UDEV_PM_PATH, UDEV_PM_CONTENT).await?;

    Ok(())
}

async fn get_nvidia_pci_bus() -> ArchbookDResult<String> {
    // get nvidia oci bus address
    let lspci_output = Command::new("lspci").output().await?;
    let lspci_output_stdout = String::from_utf8_lossy(&lspci_output.stdout);

    for line in lspci_output_stdout.lines() {
        if line.contains("NVIDIA")
            && (line.contains("VGA compatible controller") || line.contains("3D controller"))
        {
            let split_line: Vec<&str> = line.split(" ").collect();
            let pci_bus_id = split_line.first().unwrap().replace("0000:", "");
            let bus_id_split: Vec<&str> = pci_bus_id.split(":").collect();
            let (bus, device_function) =
                (bus_id_split.first().unwrap(), bus_id_split.last().unwrap());
            let device_function_split: Vec<&str> = device_function.split(".").collect();

            let (device, function) = (
                device_function_split.first().unwrap(),
                device_function_split.last().unwrap(),
            );

            return Ok(format!(
                "PCI:{}:{}:{}",
                radix(bus.parse::<i32>().unwrap(), 16),
                radix(device.parse::<i32>().unwrap(), 16),
                radix(function.parse::<i32>().unwrap(), 16)
            ));
        }
    }

    Err(ArchbookDError::InvalidPCIBusId)
}

/// Switch to use nvidia gpu
pub async fn switch_to_nvidia() -> ArchbookDResult<()> {
    systemctl_enable_now("nvidia-persistenced.service").await?;

    cleanup().await?;

    let nvidia_gpu_pci = get_nvidia_pci_bus().await?;

    fs::write(XORG_PATH, XORG_INTEL.replace("[BUS_ID]", &nvidia_gpu_pci)).await?;

    if fs::metadata(SDDM_XSETUP_PATH).await.is_ok() {
        fs::write(SDDM_XSETUP_BACKUP_PATH, fs::read(SDDM_XSETUP_PATH).await?).await?;
        fs::write(SDDM_XSETUP_PATH, NVIDIA_XRANDR_SCRIPT).await?;
    }

    Ok(())
}

/// Restores previous sddm configuration and removes files created
pub async fn reset_all() -> ArchbookDResult<()> {
    fs::write(SDDM_XSETUP_PATH, SDDM_XSETUP_CONTENT).await?;

    cleanup().await?;

    Ok(())
}

/// Checks via github api if envycontrol has been updated
pub async fn library_up_to_date() -> ArchbookDResult<bool> {
    let client = reqwest::Client::new();
    let response = client
        .get(GITHUB_API_ENDPOINT)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/118.0",
        )
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(ArchbookDError::FailedToCheckForUpdates(String::from(
            "hybrid_graphics",
        )));
    }

    let response_content = response.text().await?;

    let parsed_response = json::parse(&response_content)?;

    if parsed_response["name"] != CURRENT_ENVYCONTROL_VERSION_NAME {
        return Ok(false);
    }

    Ok(true)
}
