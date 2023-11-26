use archbookd_lib::screenpad as lib; 
use zbus::{dbus_interface, fdo};

use super::interface_trait::ArchbookdInterface;

#[derive(Default)]
pub struct ScreenpadInterface;
impl ArchbookdInterface for ScreenpadInterface {
    fn zbus_path() -> String {
        "/screenpad".to_string()
    }
}

#[dbus_interface(name = "joe.archbookd.Screenpad")]
impl ScreenpadInterface {
    async fn get_brightness(&self) -> fdo::Result<i16> {
        Ok(lib::get_brightness().await?)
    }

    async fn set_brightness(&self, value: i16) -> fdo::Result<()> {
        Ok(lib::set_brightness(value).await?)
    }

    async fn increment_brightness(&self, value: i16) -> fdo::Result<i16> {
        Ok(lib::increment_brightness(value).await?)
    }

    async fn power_on(&self) -> fdo::Result<()> {
        Ok(lib::turn_on().await?)
    }

    async fn power_off(&self) -> fdo::Result<()> {
        Ok(lib::turn_off().await?)
    }
}
