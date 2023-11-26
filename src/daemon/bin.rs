use std::future::pending;

use interfaces::{ScreenpadInterface, BatteryInterface, ArchbookdInterface};
use zbus::{fdo, ConnectionBuilder};

mod interfaces;

#[tokio::main]
async fn main() -> fdo::Result<()> {
    let _connection = ConnectionBuilder::system()?
        .name("joe.archbookd")?
        .serve_at(
            ScreenpadInterface::zbus_path(),
            ScreenpadInterface::default(),
        )?
        .serve_at(BatteryInterface::zbus_path(), BatteryInterface::default())?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
