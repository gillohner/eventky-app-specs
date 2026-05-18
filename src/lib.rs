mod common;
pub mod constants;
mod models;
pub mod traits;
mod utils;
mod validation;

// Re-export constants
pub use constants::{
    EVENTKY_PATH, MAX_ATTENDEE_URI_LENGTH, MAX_CALENDAR_AUTHORS, MAX_CALENDAR_DESCRIPTION_LENGTH,
    MAX_CALENDAR_NAME_LENGTH, MAX_CALENDAR_URIS, MAX_EVENT_DESCRIPTION_LENGTH, MAX_EVENT_LOCATIONS,
    MAX_EVENT_SUMMARY_LENGTH, MAX_EVENT_UID_LENGTH, MAX_LOCATION_DESCRIPTION_LENGTH,
    MAX_LOCATION_LABEL_LENGTH, MAX_LOCATION_URI_LENGTH, MIN_EVENT_SUMMARY_LENGTH,
    MIN_EVENT_UID_LENGTH, PROTOCOL, PUBLIC_PATH,
};

// Re-export domain types
pub use models::attendee::PubkyAppAttendee;
pub use models::calendar::{PubkyAppCalendar, StyledDescription};
pub use models::event::PubkyAppEvent;
pub use models::location::{EventLocation, EventLocationKind};
pub use models::EventkyAppObject;

// Re-export from pubky-app-specs base crate
pub use pubky_app_specs::PubkyId;

// Re-export utils & validation for consumers
pub use utils::*;
pub use validation::*;

// WASM module (only compiled on wasm32)
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
