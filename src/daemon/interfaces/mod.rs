mod interface_trait;

mod battery;
mod hybrid_graphics;
mod screenpad;
mod peripherals;

pub use interface_trait::ArchbookdInterface;
pub use battery::BatteryInterface;
pub use screenpad::ScreenpadInterface;
