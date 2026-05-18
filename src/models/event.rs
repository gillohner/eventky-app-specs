use crate::{
    common::timestamp,
    models::calendar::StyledDescription,
    models::location::EventLocation,
    traits::{HasIdPath, TimestampId, Validatable},
    validation::{is_valid_datetime, is_valid_duration, is_valid_timezone},
    EVENTKY_PATH, MAX_CALENDAR_URIS, MAX_EVENT_DESCRIPTION_LENGTH, MAX_EVENT_LOCATIONS,
    MAX_EVENT_SUMMARY_LENGTH, MAX_EVENT_UID_LENGTH, MIN_EVENT_SUMMARY_LENGTH, MIN_EVENT_UID_LENGTH,
    PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// Local aliases for centrally-defined limits (see constants.rs).
const MIN_UID_LENGTH: usize = MIN_EVENT_UID_LENGTH;
const MAX_UID_LENGTH: usize = MAX_EVENT_UID_LENGTH;
const MIN_SUMMARY_LENGTH: usize = MIN_EVENT_SUMMARY_LENGTH;
const MAX_SUMMARY_LENGTH: usize = MAX_EVENT_SUMMARY_LENGTH;
const MAX_DESCRIPTION_LENGTH: usize = MAX_EVENT_DESCRIPTION_LENGTH;
const MAX_LOCATIONS: usize = MAX_EVENT_LOCATIONS;

/// Valid event status values (RFC 5545).
const VALID_STATUS: &[&str] = &["CONFIRMED", "TENTATIVE", "CANCELLED"];

/// Event - a scheduled activity or occasion
/// URI: /pub/eventky.app/events/:event_id
/// Where event_id is a timestamp-based ID for chronological ordering
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct PubkyAppEvent {
    // RFC 5545 - Core Event Properties (REQUIRED)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub uid: String,                    // Globally unique identifier
    pub dtstamp: i64,                   // Creation/last-modified timestamp (Unix microseconds)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub dtstart: String,                // Start date-time in ISO 8601 format (YYYY-MM-DDTHH:MM:SS)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub summary: String,                // Event title/subject

    // RFC 5545 - Time & Duration
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub dtend: Option<String>,          // End date-time in ISO 8601 format (mutually exclusive with duration)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub duration: Option<String>,       // RFC 5545 duration format (mutually exclusive with dtend)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub dtstart_tzid: Option<String>,   // IANA timezone for dtstart (e.g., "Europe/Zurich")
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub dtend_tzid: Option<String>,     // IANA timezone for dtend

    // RFC 5545 - Event Details
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,    // Plain text description
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub status: Option<String>,         // CONFIRMED | TENTATIVE | CANCELLED
    
    // RFC 9073 - Structured Locations
    /// Structured locations for this event (RFC 9073 VLOCATION)
    /// First location is considered primary. Supports multiple for hybrid events.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub locations: Option<Vec<EventLocation>>,
    
    // RFC 7986 - Event Publishing Extensions
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub image_uri: Option<String>,      // Event image/banner URI
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub url: Option<String>,            // Event homepage/details link

    // RFC 5545 - Change Management
    pub sequence: Option<i32>,          // Version number (increment on modifications)
    pub last_modified: Option<i64>,     // Last modification timestamp
    pub created: Option<i64>,           // Creation timestamp

    // RFC 5545 - Recurrence
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub rrule: Option<String>,          // Recurrence rule (RFC 5545 format)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub rdate: Option<Vec<String>>,     // Additional recurrence dates
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub exdate: Option<Vec<String>>,    // Excluded recurrence dates
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub recurrence_id: Option<String>,  // ISO 8601 datetime of specific recurrence instance

    // RFC 9073 - Rich Content
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub styled_description: Option<StyledDescription>, // Formatted description with metadata

    // Pubky Extensions (all custom fields use x_pubky_ prefix)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub x_pubky_calendar_uris: Option<Vec<String>>, // URIs of calendars containing this event
}

impl PubkyAppEvent {
    /// Creates a new `PubkyAppEvent` instance with required fields and sensible defaults.
    pub fn new(uid: String, dtstart: String, summary: String) -> Self {
        let now = timestamp();
        Self {
            uid,
            dtstamp: now,
            dtstart,
            summary,
            // Sensible defaults for all optional fields
            dtend: None,
            duration: None,
            dtstart_tzid: None,
            dtend_tzid: None,
            description: None,
            status: Some("CONFIRMED".to_string()),
            locations: None,
            image_uri: None,
            url: None,
            sequence: Some(0),
            last_modified: Some(now),
            created: Some(now),
            rrule: None,
            rdate: None,
            exdate: None,
            recurrence_id: None,
            styled_description: None,
            x_pubky_calendar_uris: None,
        }
        .sanitize()
    }

    /// Helper method to create an event with an end time
    pub fn with_end_time(mut self, dtend: String) -> Self {
        self.dtend = Some(dtend);
        self.sanitize()
    }

    /// Helper method to add a description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.sanitize()
    }

    /// Helper method to add structured locations (RFC 9073)
    pub fn with_locations(mut self, locations: Vec<EventLocation>) -> Self {
        self.locations = Some(locations);
        self.sanitize()
    }

    /// Helper method to add a single location
    pub fn with_location_item(mut self, location: EventLocation) -> Self {
        self.locations = Some(vec![location]);
        self.sanitize()
    }

    /// Helper method to set the event status
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self.sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PubkyAppEvent {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn uid(&self) -> String {
        self.uid.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn summary(&self) -> String {
        self.summary.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn dtstart(&self) -> String {
        self.dtstart.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn dtend(&self) -> Option<String> {
        self.dtend.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn duration(&self) -> Option<String> {
        self.duration.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn dtstart_tzid(&self) -> Option<String> {
        self.dtstart_tzid.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn dtend_tzid(&self) -> Option<String> {
        self.dtend_tzid.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn status(&self) -> Option<String> {
        self.status.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn locations(&self) -> Option<Vec<EventLocation>> {
        self.locations.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn image_uri(&self) -> Option<String> {
        self.image_uri.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn rrule(&self) -> Option<String> {
        self.rrule.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn rdate(&self) -> Option<Vec<String>> {
        self.rdate.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn exdate(&self) -> Option<Vec<String>> {
        self.exdate.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn styled_description(&self) -> Option<StyledDescription> {
        self.styled_description.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn x_pubky_calendar_uris(&self) -> Option<Vec<String>> {
        self.x_pubky_calendar_uris.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for PubkyAppEvent {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PubkyAppEvent {
    /// Creates a new `PubkyAppEvent` instance for WASM.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new_wasm(uid: String, dtstart: String, summary: String) -> Self {
        Self::new(uid, dtstart, summary)
    }

    /// Generates a unique timestamp-based ID for the event
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = createId))]
    pub fn create_id_wasm(&self) -> String {
        self.create_id()
    }
}

impl TimestampId for PubkyAppEvent {}

impl HasIdPath for PubkyAppEvent {
    const PATH_SEGMENT: &'static str = "events/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, EVENTKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for PubkyAppEvent {
    fn sanitize(self) -> Self {
        // Sanitize UID
        let uid = self.uid.trim().chars().take(MAX_UID_LENGTH).collect();
        
        // Sanitize summary
        let summary = self.summary.trim().chars().take(MAX_SUMMARY_LENGTH).collect();
        
        // Sanitize dtstart (trim whitespace, validate format)
        let dtstart = self.dtstart.trim().to_string();
        
        // Sanitize dtend (trim whitespace, validate format)
        let dtend = self.dtend.map(|dt| dt.trim().to_string());
        
        // Sanitize description
        let description = self.description.map(|desc| {
            desc.trim().chars().take(MAX_DESCRIPTION_LENGTH).collect()
        });
        
        // Sanitize status (normalize to uppercase)
        let status = self.status.map(|s| {
            let s = s.trim().to_uppercase();
            if VALID_STATUS.contains(&s.as_str()) {
                s
            } else {
                "CONFIRMED".to_string() // Default to CONFIRMED for invalid status
            }
        });
        
        // Sanitize URIs (image_uri, url, calendar_uris)
        let image_uri = self.image_uri.and_then(|uri| {
            match Url::parse(&uri.trim()) {
                Ok(url) => Some(url.to_string()),
                Err(_) => None,
            }
        });
        
        let url = self.url.and_then(|uri| {
            match Url::parse(&uri.trim()) {
                Ok(url) => Some(url.to_string()),
                Err(_) => None,
            }
        });
        
        let x_pubky_calendar_uris = self.x_pubky_calendar_uris.map(|uris| {
            uris.into_iter()
                .take(MAX_CALENDAR_URIS)
                .filter_map(|uri| {
                    match Url::parse(&uri.trim()) {
                        Ok(url) => Some(url.to_string()),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>()
        }).filter(|uris| !uris.is_empty());
        
        // Sanitize timezones
        let dtstart_tzid = self.dtstart_tzid.and_then(|tz| {
            if is_valid_timezone(&tz.trim()) {
                Some(tz.trim().to_string())
            } else {
                None
            }
        });
        
        let dtend_tzid = self.dtend_tzid.and_then(|tz| {
            if is_valid_timezone(&tz.trim()) {
                Some(tz.trim().to_string())
            } else {
                None
            }
        });
        
        // Sanitize duration
        let duration = self.duration.and_then(|dur| {
            if is_valid_duration(&dur.trim()) {
                Some(dur.trim().to_string())
            } else {
                None
            }
        });
        
        // Sanitize locations (limit count and sanitize each)
        let locations = self.locations.map(|locs| {
            locs.into_iter()
                .take(MAX_LOCATIONS)
                .map(|loc| loc.sanitize())
                .collect::<Vec<_>>()
        }).filter(|locs| !locs.is_empty());
        
        Self {
            uid,
            dtstamp: self.dtstamp,
            dtstart,
            summary,
            dtend,
            duration,
            dtstart_tzid,
            dtend_tzid,
            description,
            status,
            locations,
            image_uri,
            url,
            sequence: self.sequence,
            last_modified: self.last_modified,
            created: self.created,
            rrule: self.rrule,
            rdate: self.rdate,
            exdate: self.exdate,
            recurrence_id: self.recurrence_id,
            styled_description: self.styled_description,
            x_pubky_calendar_uris,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        // Validate the event ID
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate UID
        let uid_length = self.uid.chars().count();
        if !(MIN_UID_LENGTH..=MAX_UID_LENGTH).contains(&uid_length) {
            return Err("Validation Error: Event UID length must be between 1 and 255 characters".into());
        }

        // Validate summary
        let summary_length = self.summary.chars().count();
        if !(MIN_SUMMARY_LENGTH..=MAX_SUMMARY_LENGTH).contains(&summary_length) {
            return Err("Validation Error: Event summary length must be between 1 and 500 characters".into());
        }

        // Validate dtstart format
        if !is_valid_datetime(&self.dtstart) {
            return Err("Validation Error: Invalid start date-time format. Must be ISO 8601 (YYYY-MM-DDTHH:MM:SS)".into());
        }

        // Validate dtend format if present
        if let Some(ref dtend) = self.dtend {
            if !is_valid_datetime(dtend) {
                return Err("Validation Error: Invalid end date-time format. Must be ISO 8601 (YYYY-MM-DDTHH:MM:SS)".into());
            }
            
            // Validate that dtend is after dtstart
            if dtend <= &self.dtstart {
                return Err("Validation Error: Event end time must be after start time".into());
            }
        }

        // Validate that only one of dtend or duration is present
        if self.dtend.is_some() && self.duration.is_some() {
            return Err("Validation Error: Event cannot have both dtend and duration".into());
        }

        // Validate status
        if let Some(status) = &self.status {
            if !VALID_STATUS.contains(&status.as_str()) {
                return Err("Validation Error: Invalid event status".into());
            }
        }

        // Validate description length
        if let Some(desc) = &self.description {
            if desc.chars().count() > MAX_DESCRIPTION_LENGTH {
                return Err("Validation Error: Event description exceeds maximum length".into());
            }
        }

        // Validate structured locations
        if let Some(locations) = &self.locations {
            if locations.len() > MAX_LOCATIONS {
                return Err(format!(
                    "Validation Error: Too many locations (max {})",
                    MAX_LOCATIONS
                ));
            }
            for (i, loc) in locations.iter().enumerate() {
                if let Err(e) = loc.validate(None) {
                    return Err(format!("Validation Error: Location {}: {}", i + 1, e));
                }
            }
        }

        // Validate calendar URIs count
        if let Some(cal_uris) = &self.x_pubky_calendar_uris {
            if cal_uris.len() > MAX_CALENDAR_URIS {
                return Err("Validation Error: Too many calendar URIs".into());
            }
        }

        // Additional validations can be added here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Validatable;

    #[test]
    fn test_new_simple() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        );

        assert_eq!(event.uid, "event-123");
        assert_eq!(event.dtstart, "2025-12-01T10:00:00");
        assert_eq!(event.summary, "Team Meeting");
        assert_eq!(event.status, Some("CONFIRMED".to_string()));
        assert!(event.created.is_some());
        
        // Check that timestamps are recent
        let now = timestamp();
        assert!(event.dtstamp <= now && event.dtstamp >= now - 1_000_000);
    }

    #[test]
    fn test_new_complex() {
        use crate::EventLocation;

        let event = PubkyAppEvent::new(
            "complex-event-456".to_string(),
            "2025-12-01T14:00:00".to_string(),
            "Annual Conference".to_string(),
        )
        .with_end_time("2025-12-01T18:00:00".to_string())
        .with_description("Annual company conference with presentations".to_string())
        .with_location_item(EventLocation::physical("Convention Center"));

        assert_eq!(event.uid, "complex-event-456");
        assert_eq!(event.dtstart, "2025-12-01T14:00:00");
        assert_eq!(event.dtend, Some("2025-12-01T18:00:00".to_string()));
        assert_eq!(event.summary, "Annual Conference");
        assert!(event.locations.is_some());
        assert_eq!(event.locations.as_ref().unwrap()[0].label, "Convention Center");
    }

    #[test]
    fn test_create_id() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        );

        let event_id = event.create_id();
        println!("Generated Event ID: {}", event_id);

        // Assert that the event ID is 13 characters long (timestamp-based)
        assert_eq!(event_id.len(), 13);
    }

    #[test]
    fn test_create_path() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        );

        let event_id = event.create_id();
        let path = PubkyAppEvent::create_path(&event_id);

        // Check if the path starts with the expected prefix
        let prefix = format!("{}{}events/", PUBLIC_PATH, EVENTKY_PATH);
        assert!(path.starts_with(&prefix));

        let expected_path_len = prefix.len() + event_id.len();
        assert_eq!(path.len(), expected_path_len);
    }

    #[test]
    fn test_validate_valid() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        );

        let id = event.create_id();
        let result = event.validate(Some(&id));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_uid() {
        let event = PubkyAppEvent::new(
            "".to_string(), // Empty UID
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        );

        let id = event.create_id();
        let result = event.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("UID length"));
    }

    #[test]
    fn test_validate_invalid_summary() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "".to_string(), // Empty summary
        );

        let id = event.create_id();
        let result = event.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("summary length"));
    }

    #[test]
    fn test_validate_invalid_time_order() {
        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        ).with_end_time("2025-12-01T09:00:00".to_string()); // End before start

        let id = event.create_id();
        let result = event.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("end time must be after start time"));
    }

    #[test]
    fn test_validate_both_dtend_and_duration() {
        let mut event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        ).with_end_time("2025-12-01T11:00:00".to_string());
        
        // Set both dtend and duration (invalid)
        event.duration = Some("PT1H".to_string());

        let id = event.create_id();
        let result = event.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot have both dtend and duration"));
    }

    #[test]
    fn test_sanitize() {
        let event = PubkyAppEvent::new(
            "  event-123  ".to_string(), // uid
            "  2025-12-01T10:00:00  ".to_string(), // dtstart
            "  Team Meeting  ".to_string(), // summary
        )
        .with_description("  Meeting description  ".to_string())
        .with_status("  confirmed  ".to_string()); // lowercase

        assert_eq!(event.uid, "event-123");
        assert_eq!(event.dtstart, "2025-12-01T10:00:00");
        assert_eq!(event.summary, "Team Meeting");
        assert_eq!(event.description, Some("Meeting description".to_string()));
        assert_eq!(event.status, Some("CONFIRMED".to_string()));
    }

    #[test]
    fn test_timezone_validation() {
        assert!(is_valid_timezone("Europe/Zurich"));
        assert!(is_valid_timezone("America/New_York"));
        assert!(is_valid_timezone("Asia/Tokyo"));
        assert!(!is_valid_timezone(""));           // Empty
        assert!(!is_valid_timezone("Invalid"));    // No slash
        assert!(!is_valid_timezone("Europe@Zurich")); // Invalid char
    }

    #[test]
    fn test_duration_validation() {
        assert!(is_valid_duration("PT1H"));       // 1 hour
        assert!(is_valid_duration("PT30M"));      // 30 minutes
        assert!(is_valid_duration("P1D"));        // 1 day
        assert!(is_valid_duration("P1DT2H30M"));  // 1 day, 2 hours, 30 minutes
        assert!(!is_valid_duration("1H"));        // Missing P
        assert!(!is_valid_duration(""));          // Empty
        assert!(!is_valid_duration("PT@H"));      // Invalid character
    }

    #[test]
    fn test_try_from_valid() {
        let event_json = r##"
        {
            "uid": "event-123",
            "dtstamp": 1700000000000,
            "dtstart": "2025-12-01T10:00:00",
            "summary": "Team Meeting",
            "dtend": "2025-12-01T11:00:00",
            "duration": null,
            "dtstart_tzid": "Europe/Zurich",
            "dtend_tzid": "Europe/Zurich",
            "description": "Weekly team sync meeting",
            "status": "CONFIRMED",
            "locations": [{"label": "Conference Room A", "kind": "PHYSICAL"}],
            "image_uri": null,
            "url": "https://example.com/meeting",
            "sequence": 0,
            "last_modified": 1700000000000,
            "created": 1700000000000,
            "rrule": null,
            "rdate": null,
            "exdate": null,
            "recurrence_id": null,
            "styled_description": null,
            "x_pubky_calendar_uris": null
        }
        "##;

        let event = PubkyAppEvent::new(
            "event-123".to_string(),
            "2025-12-01T10:00:00".to_string(),
            "Team Meeting".to_string(),
        ).with_end_time("2025-12-01T11:00:00".to_string());
        let id = event.create_id();

        let blob = event_json.as_bytes();
        let event_parsed = <PubkyAppEvent as Validatable>::try_from(blob, &id).unwrap();

        assert_eq!(event_parsed.uid, "event-123");
        assert_eq!(event_parsed.dtstart, "2025-12-01T10:00:00");
        assert_eq!(event_parsed.summary, "Team Meeting");
        assert!(event_parsed.locations.is_some());
        assert_eq!(event_parsed.locations.as_ref().unwrap()[0].label, "Conference Room A");
    }
}