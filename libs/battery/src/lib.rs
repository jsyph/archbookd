use archbookd_error::ArchbookDResult;
use service_utils::{enable_service_now, nuke_active_service, create_service_in_systemd_directory};
use tokio::fs;

const BATTERY_SERVICE_TEMPLATE: &str = include_str!("./service.template");
const BATTERY_SERVICE_EVENTS: [&str; 5] = [
    "hibernate",
    "hybrid-sleep",
    "multi-user",
    "suspend",
    "suspend-then-hibernate",
];

const BATTERY_CHARGE_THRESHOLD_FILE: &str =
    "/sys/class/power_supply/BAT0/charge_control_end_threshold";

pub async fn get_charge_threshold() -> ArchbookDResult<i16> {
    let unparsed_brightness = fs::read_to_string(BATTERY_CHARGE_THRESHOLD_FILE).await?;

    Ok(unparsed_brightness.trim().parse::<i16>()?)
}

/// Temporary set charge threshold until the next power cycle
pub async fn set_charge_threshold(value: i16) -> ArchbookDResult<()> {
    fs::write(BATTERY_CHARGE_THRESHOLD_FILE, value.to_string()).await?;
    Ok(())
}

async fn persist_charge_threshold(value: i16) -> ArchbookDResult<()> {
    for event in BATTERY_SERVICE_EVENTS {
        let mut service_content = BATTERY_SERVICE_TEMPLATE.replace("EVENT", event);
        service_content = service_content.replace("THRESHOLD", &value.to_string());

        let service_name = format!("archbookd-{}-charge-maximum-persistence.service", event);

        create_service_in_systemd_directory(&service_name, &service_content).await?;
        enable_service_now(&service_name).await?;
    }

    Ok(())
}

/// Set and persist the charge threshold
pub async fn set_and_persist_charge_threshold(value: i16) -> ArchbookDResult<()> {
    set_charge_threshold(value).await?;
    persist_charge_threshold(value).await?;

    Ok(())
}

/// Resets the charge threshold to 100 and
/// removing any service files created by `set_and_persist_charge_threshold`
pub async fn full_reset() -> ArchbookDResult<()> {
    set_charge_threshold(100).await?;

    for event in BATTERY_SERVICE_EVENTS {
        let service_name = format!("archbookd-{}-charge-maximum-persistence.service", event);

        nuke_active_service(&service_name).await?;
    }
    
    Ok(())
}
