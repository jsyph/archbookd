mod interface_trait;

pub use interface_trait::ArchbookdInterface;

mod battery;
mod hybrid_graphics;
mod screenpad;
mod peripherals;

pub use screenpad::ScreenpadInterface;
pub use battery::Battery;