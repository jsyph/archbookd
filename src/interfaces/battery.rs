use super::ArchbookdInterface;
use zbus::{dbus_interface, fdo};
use archbookd_lib::battery as lib;

#[derive(Default)]
pub struct Battery;
impl ArchbookdInterface for Battery {
    fn zbus_path() -> String {
        "/power".to_string()
    }
}

#[dbus_interface(name = "joe.archbookd.Battery")]
impl Battery {
    async fn get_charge_threshold(&self) -> fdo::Result<i16> {
        Ok(lib::get_charge_threshold().await?)
    }

    async fn temporary_set_charge_threshold(&self, value: i16) -> fdo::Result<()> {
        lib::set_charge_threshold(value).await?;
        Ok(())
    }

    async fn persist_set_charge_threshold(&self, value: i16) -> fdo::Result<()> {
        lib::set_and_persist_charge_threshold(value).await?;
        Ok(())
    }

    async fn reset(&self) -> fdo::Result<()> {
        lib::full_reset().await?;
        Ok(())
    }
}
