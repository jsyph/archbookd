use interfaces::{ArchbookdInterface, Battery, ScreenpadInterface};
use std::future::pending;
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
        .serve_at(Battery::zbus_path(), Battery::default())?
        .build()
        .await?;

    pending::<()>().await;
    
    Ok(())
}
