use std::{
    error::Error,
    fmt::{Debug, Display},
    io,
    num::ParseIntError,
};
use zbus::fdo;

pub type ArchbookDResult<T> = Result<T, ArchbookDError>;

pub enum ArchbookDError {
    IO(io::Error),
    ParseInt(ParseIntError),
    SystemCtlEnable(String),
    SystemCtlDisable(String),
    SystemCtlDaemonReload,
    ScreenpadBrightnessOutOfRange,
    InvalidPCIBusId,
    Reqwest(reqwest::Error),
    FailedToCheckForUpdates(String),
    JsonParsing(json::Error)
}

impl Error for ArchbookDError {}

impl Debug for ArchbookDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ArchbookDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchbookDError::IO(error) => write!(f, "IO error on line {}: {}", line!(), error),
            ArchbookDError::ParseInt(error) => {
                write!(f, "ParseInt error on line {}: {}", line!(), error)
            }
            ArchbookDError::SystemCtlEnable(service) => write!(
                f,
                "systemctl service activation error on line {}: {}",
                line!(),
                service
            ),
            ArchbookDError::SystemCtlDisable(service) => write!(
                f,
                "systemctl service deactivation error on line {}: {}",
                line!(),
                service
            ),
            ArchbookDError::SystemCtlDaemonReload => {
                write!(f, "systemctl daemon-reload failed on line {}", line!())
            }
            ArchbookDError::ScreenpadBrightnessOutOfRange => write!(
                f,
                "Screenpad brightness out of range. BRIGHTNESS >= 0 and BRIGHTNESS <= 255."
            ),
            ArchbookDError::InvalidPCIBusId => write!(f, "Lspci returned invalid pci address"),
            ArchbookDError::Reqwest(error) => write!(f, "{}", error),
            ArchbookDError::FailedToCheckForUpdates(module) => {
                write!(f, "Failed to check on updates for module {}", module)
            }
            ArchbookDError::JsonParsing(error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for ArchbookDError {
    fn from(value: io::Error) -> Self {
        ArchbookDError::IO(value)
    }
}

impl From<ParseIntError> for ArchbookDError {
    fn from(value: ParseIntError) -> Self {
        ArchbookDError::ParseInt(value)
    }
}

impl From<ArchbookDError> for fdo::Error {
    fn from(value: ArchbookDError) -> Self {
        Self::Failed(value.to_string())
    }
}

impl From<reqwest::Error> for ArchbookDError {
    fn from(value: reqwest::Error) -> Self {
        ArchbookDError::Reqwest(value)
    }
}

impl From<json::Error> for ArchbookDError{
    fn from(value: json::Error) -> Self {
        ArchbookDError::JsonParsing(value)
    }
}