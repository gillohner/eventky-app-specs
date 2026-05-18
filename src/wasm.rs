use crate::constants::EVENTKY_PATH;
use crate::traits::{HasIdPath, HashId, TimestampId, Validatable};
use crate::*;
use pubky_app_specs::PubkyAppTag;
use wasm_bindgen::prelude::*;

/// Metadata returned alongside a constructed domain object — the deterministic
/// `id`, the homeserver `path`, and the fully-qualified pubky:// `url`.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct EventkyMeta {
    id: String,
    path: String,
    url: String,
}

#[wasm_bindgen]
impl EventkyMeta {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

impl EventkyMeta {
    fn new(id: &str, pubky_id: &PubkyId, path: String) -> Self {
        Self {
            id: id.to_string(),
            url: format!("{}{}{}", PROTOCOL, pubky_id, path),
            path,
        }
    }
}

macro_rules! result_struct {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        #[wasm_bindgen]
        pub struct $struct_name {
            $field_name: $field_type,
            meta: EventkyMeta,
        }

        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(getter)]
            pub fn $field_name(&self) -> $field_type {
                self.$field_name.clone()
            }

            #[wasm_bindgen(getter)]
            pub fn meta(&self) -> EventkyMeta {
                self.meta.clone()
            }
        }
    };
}

result_struct!(EventkyEventResult, event, PubkyAppEvent);
result_struct!(EventkyCalendarResult, calendar, PubkyAppCalendar);
result_struct!(EventkyAttendeeResult, attendee, PubkyAppAttendee);
result_struct!(EventkyTagResult, tag, PubkyAppTag);

/// Builder for eventky-domain objects, scoped to a single pubky author id.
/// Mirrors `MapkySpecsBuilder` — construct once per session, call `create*`
/// methods, then persist the returned object at `meta.path` on the homeserver.
#[wasm_bindgen]
pub struct EventkySpecsBuilder {
    #[wasm_bindgen(skip)]
    pubky_id: PubkyId,
}

#[wasm_bindgen]
impl EventkySpecsBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(pubky_id: String) -> Result<Self, String> {
        let pubky_id = PubkyId::try_from(&pubky_id)?;
        Ok(Self { pubky_id })
    }

    /// Create a new event. `uid` is the RFC 5545 globally-unique identifier
    /// (the caller typically uses a UUID); `dtstart` is an ISO 8601 datetime.
    #[wasm_bindgen(js_name = createEvent)]
    pub fn create_event(
        &self,
        uid: String,
        dtstart: String,
        summary: String,
    ) -> Result<EventkyEventResult, String> {
        let event = PubkyAppEvent::new(uid, dtstart, summary);
        let event_id = event.create_id();
        event.validate(Some(&event_id))?;

        let path = PubkyAppEvent::create_path(&event_id);
        let meta = EventkyMeta::new(&event_id, &self.pubky_id, path);

        Ok(EventkyEventResult { event, meta })
    }

    /// Create a new calendar. `timezone` must be a valid IANA identifier
    /// (e.g., `Europe/Zurich`).
    #[wasm_bindgen(js_name = createCalendar)]
    pub fn create_calendar(
        &self,
        name: String,
        timezone: String,
    ) -> Result<EventkyCalendarResult, String> {
        let calendar = PubkyAppCalendar::new(name, timezone);
        let calendar_id = calendar.create_id();
        calendar.validate(Some(&calendar_id))?;

        let path = PubkyAppCalendar::create_path(&calendar_id);
        let meta = EventkyMeta::new(&calendar_id, &self.pubky_id, path);

        Ok(EventkyCalendarResult { calendar, meta })
    }

    /// Create a new attendee/RSVP record. `partstat` defaults to NEEDS-ACTION
    /// when absent; when provided it must be one of NEEDS-ACTION | ACCEPTED |
    /// DECLINED | TENTATIVE. `recurrence_id` scopes the RSVP to a specific
    /// instance of a recurring event.
    #[wasm_bindgen(js_name = createAttendee)]
    pub fn create_attendee(
        &self,
        event_uri: String,
        partstat: Option<String>,
        recurrence_id: Option<String>,
    ) -> Result<EventkyAttendeeResult, String> {
        let mut attendee = match partstat {
            Some(p) => PubkyAppAttendee::with_status(event_uri, p),
            None => PubkyAppAttendee::new(event_uri),
        };
        attendee.recurrence_id = recurrence_id;
        let attendee = attendee.sanitize();
        let attendee_id = attendee.create_id();
        attendee.validate(Some(&attendee_id))?;

        let path = PubkyAppAttendee::create_path(&attendee_id);
        let meta = EventkyMeta::new(&attendee_id, &self.pubky_id, path);

        Ok(EventkyAttendeeResult { attendee, meta })
    }

    /// Create a `PubkyAppTag` stored under the eventky namespace at
    /// `/pub/eventky.app/tags/{tag_id}`. Uses the universal `PubkyAppTag`
    /// schema so nexus' universal tag handler picks it up; the app-specific
    /// path is what routes indexing to the eventky plugin.
    #[wasm_bindgen(js_name = createTag)]
    pub fn create_tag(&self, uri: String, label: String) -> Result<EventkyTagResult, String> {
        let tag = PubkyAppTag::new(uri, label);
        let tag_id = tag.create_id();
        let path = format!("{}{}tags/{}", PUBLIC_PATH, EVENTKY_PATH, tag_id);
        let meta = EventkyMeta::new(&tag_id, &self.pubky_id, path);
        Ok(EventkyTagResult { tag, meta })
    }
}
