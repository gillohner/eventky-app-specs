use crate::{
    constants::{EVENTKY_PATH, PROTOCOL, PUBLIC_PATH},
    traits::HasIdPath,
    PubkyAppAttendee, PubkyAppCalendar, PubkyAppEvent,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Builds an Eventky base URI: "pubky://<user_id>/pub/eventky.app/"
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = eventkyBaseUriBuilder))]
pub fn eventky_base_uri_builder(user_id: String) -> String {
    format!("{}{}{}{}", PROTOCOL, user_id, PUBLIC_PATH, EVENTKY_PATH)
}

/// Builds an Event URI: "pubky://<author_id>/pub/eventky.app/events/<event_id>"
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = eventkyEventUriBuilder))]
pub fn eventky_event_uri_builder(author_id: String, event_id: String) -> String {
    let path = PubkyAppEvent::create_path(&event_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds a Calendar URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = eventkyCalendarUriBuilder))]
pub fn eventky_calendar_uri_builder(author_id: String, calendar_id: String) -> String {
    let path = PubkyAppCalendar::create_path(&calendar_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds an Attendee URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = eventkyAttendeeUriBuilder))]
pub fn eventky_attendee_uri_builder(author_id: String, attendee_id: String) -> String {
    let path = PubkyAppAttendee::create_path(&attendee_id);
    [PROTOCOL, &author_id, &path].concat()
}
