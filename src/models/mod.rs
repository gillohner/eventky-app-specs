pub mod attendee;
pub mod calendar;
pub mod event;
pub mod location;

use crate::traits::Validatable;
use crate::{PubkyAppAttendee, PubkyAppCalendar, PubkyAppEvent};

/// A unified enum wrapping all EventkyApp objects dispatched from the watcher.
#[derive(Debug, Clone)]
pub enum EventkyAppObject {
    Event(PubkyAppEvent),
    Calendar(PubkyAppCalendar),
    Attendee(PubkyAppAttendee),
}

impl EventkyAppObject {
    /// Parse a blob into an EventkyAppObject based on the path segment.
    /// path_segment should be e.g. "events", "calendars", "attendees".
    pub fn from_path(path_segment: &str, blob: &[u8], id: &str) -> Result<Self, String> {
        match path_segment {
            "events" => {
                let obj = <PubkyAppEvent as Validatable>::try_from(blob, id)?;
                Ok(EventkyAppObject::Event(obj))
            }
            "calendars" => {
                let obj = <PubkyAppCalendar as Validatable>::try_from(blob, id)?;
                Ok(EventkyAppObject::Calendar(obj))
            }
            "attendees" => {
                let obj = <PubkyAppAttendee as Validatable>::try_from(blob, id)?;
                Ok(EventkyAppObject::Attendee(obj))
            }
            _ => Err(format!(
                "Unrecognized eventky.app resource: {}",
                path_segment
            )),
        }
    }
}
