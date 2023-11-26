use zbus::{dbus_interface, fdo};
use archbookd_lib::battery as lib;

use super::interface_trait::ArchbookdInterface;

#[derive(Default)]
pub struct BatteryInterface;
impl ArchbookdInterface for BatteryInterface {
    fn zbus_path() -> String {
        "/power".to_string()
    }
}

#[dbus_interface(name = "joe.archbookd.Battery")]
impl BatteryInterface {
    async fn get_charge_threshold(&self) -> fdo::Result<i16> {
        Ok(lib::get_charge_threshold().await?)
    }

    async fn temporary_set_charge_threshold(&self, value: i16) -> fdo::Result<()> {
        lib::set_charge_threshold(value).await?;
        Ok(())
    }

    async fn persist_set_charge_threshold(&self, value: i16) -> fdo::Result<()> {
        lib::set_charge_threshold(value).await?;
        lib::persist_charge_threshold(value).await?;
        
        Ok(())
    }

    async fn reset(&self) -> fdo::Result<()> {
        lib::full_reset().await?;
        Ok(())
    }
}
