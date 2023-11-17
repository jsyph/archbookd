use std::path::Path;

use crate::error::{ArchbookDResult, ArchbookDError};
use tokio::fs;

const BRIGHTNESS_FILE: &str = "/sys/class/leds/asus::screenpad/brightness";
const BACKUP_BRIGHTNESS_FILE: &str = "/var/lib/archbookd/brightness_backup";
const MIN_WORKING_BRIGHTNESS: i16 = 1;

/// Gets the current brightness
pub async fn get_brightness() -> ArchbookDResult<i16> {
    let unparsed_brightness = fs::read_to_string(BRIGHTNESS_FILE).await?;

    Ok(unparsed_brightness.trim().parse::<i16>()?)
}

/// Sets the current brightness
pub async fn set_brightness(value: i16) -> ArchbookDResult<()> {
    if value > 255 || value < 0 {
        return Err(ArchbookDError::ScreenpadBrightnessOutOfRange);
    }

    fs::write(BRIGHTNESS_FILE, value.to_string()).await?;

    Ok(())
}

/// returns true of backup exits
async fn brightness_backup_exists() -> bool {
    fs::metadata(BACKUP_BRIGHTNESS_FILE).await.is_ok()
}

/// creates file and parent directories
async fn create_file_and_parents<S: AsRef<[u8]>>(
    file: &str,
    file_content: S,
) -> ArchbookDResult<()> {
    let path = Path::new(file);
    let parents = path.parent().expect(
        "This should never happen, cannot determine parent directories of brightness backup file",
    );

    fs::create_dir_all(parents).await?;

    fs::write(path, file_content).await?;

    Ok(())
}

/// get the backup brightness
pub async fn get_backup_brightness() -> ArchbookDResult<i16> {
    // check if backup brightness file exists
    if !brightness_backup_exists().await {
        let current_brightness = get_brightness().await?;
        create_file_and_parents(BACKUP_BRIGHTNESS_FILE, current_brightness.to_string()).await?;
        return Ok(current_brightness);
    }

    let backup_brightness = fs::read_to_string(BACKUP_BRIGHTNESS_FILE).await?;

    Ok(backup_brightness.parse::<i16>()?)
}

/// sets the backup brightness
pub async fn set_backup_brightness(value: i16) -> ArchbookDResult<()> {
    if !brightness_backup_exists().await {
        create_file_and_parents(BACKUP_BRIGHTNESS_FILE, value.to_string()).await?;
    }

    fs::write(BACKUP_BRIGHTNESS_FILE, value.to_string()).await?;

    Ok(())
}

/// increments the brightness by value
pub async fn increment_brightness(value: i16) -> ArchbookDResult<i16> {
    let current_brightness = get_brightness().await?;

    let mut new_brightness = current_brightness + value;
    // if the new brightness is more than 255, then set the brightness to 255
    if new_brightness > 255 {
        set_brightness(255).await?;
        new_brightness = 255;
    // if the new brightness is less than 0, then set the brightness to MIN_WORKING_BRIGHTNESS
    // (i don't want the screen to turn off, just stay barely on)
    } else if new_brightness < 0 {
        set_brightness(MIN_WORKING_BRIGHTNESS).await?;
        new_brightness = MIN_WORKING_BRIGHTNESS;
    } else {
        set_brightness(new_brightness).await?;
    }

    Ok(new_brightness)
}

/// Turn the screen on
pub async fn turn_on() -> ArchbookDResult<()> {
    let previous_brightness = get_backup_brightness().await?;
    set_brightness(previous_brightness).await?;
    Ok(())
}

/// Turn the screen off
pub async fn turn_off() -> ArchbookDResult<()> {
    let current_brightness = get_brightness().await?;
    set_backup_brightness(current_brightness).await?;
    set_brightness(0).await?;
    Ok(())
}

//? To restore brightness at start, use busctl to invoke
//? busctl call joe.archbookd /screenpad joe.archbookd.Screenpad SetBrightness n 255