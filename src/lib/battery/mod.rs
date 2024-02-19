use crate::error::ArchbookDResult;
use crate::utils::systemctl::{
    create_in_systemd_directory, remove_from_systemd_directory, systemctl_daemon_reload,
    systemctl_enable_now,
};
use tokio::fs;

const BATTERY_SERVICE_TEMPLATE: &str = include_str!("./service.template");
const BATTERY_SERVICE_EVENTS: [&str; 5] = [
    "hibernate",
    "hybrid-sleep",
    "multi-user",
    "suspend",
    "suspend-then-hibernate",
];

#[cfg(not(debug_assertions))]
const BATTERY_CHARGE_THRESHOLD_FILE: &str =
    "/sys/class/power_supply/BAT0/charge_control_end_threshold";
#[cfg(debug_assertions)]
const BATTERY_CHARGE_THRESHOLD_FILE: &str = "./lib_test/charge_control_end_threshold";

/// Gets screenpad current brightness
pub async fn get_charge_threshold() -> ArchbookDResult<i16> {
    let unparsed_brightness = fs::read_to_string(BATTERY_CHARGE_THRESHOLD_FILE).await?;

    Ok(unparsed_brightness.trim().parse::<i16>()?)
}

/// Temporary set charge threshold until the next power cycle
pub async fn set_charge_threshold(value: i16) -> ArchbookDResult<()> {
    fs::write(BATTERY_CHARGE_THRESHOLD_FILE, value.to_string()).await?;
    Ok(())
}

fn service_name(event: &str) -> String {
    format!("archbookd-{}-charge-maximum-persistence.service", event)
}

/// Creates services to set the charge threshold
pub async fn persist_charge_threshold(value: i16) -> ArchbookDResult<()> {
    for event in BATTERY_SERVICE_EVENTS {
        let mut service_content = BATTERY_SERVICE_TEMPLATE.replace("EVENT", event);
        service_content = service_content.replace("THRESHOLD", &value.to_string());

        let service_name = service_name(event);

        create_in_systemd_directory(&service_name, &service_content).await?;
        systemctl_enable_now(&service_name).await?;
    }

    Ok(())
}

/// Resets the charge threshold to 100 and
/// removing any service files created by `set_and_persist_charge_threshold`
pub async fn full_reset() -> ArchbookDResult<()> {
    set_charge_threshold(100).await?;

    for event in BATTERY_SERVICE_EVENTS {
        let service_name = service_name(event);

        remove_from_systemd_directory(&service_name).await?;
        systemctl_daemon_reload().await?;
    }

    Ok(())
}
