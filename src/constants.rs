pub static PUBLIC_PATH: &str = "/pub/";
pub static EVENTKY_PATH: &str = "eventky.app/";
pub static PROTOCOL: &str = "pubky://";

// Event limits
pub const MIN_EVENT_UID_LENGTH: usize = 1;
pub const MAX_EVENT_UID_LENGTH: usize = 255;
pub const MIN_EVENT_SUMMARY_LENGTH: usize = 1;
pub const MAX_EVENT_SUMMARY_LENGTH: usize = 500;
pub const MAX_EVENT_DESCRIPTION_LENGTH: usize = 10_000;
pub const MAX_CALENDAR_URIS: usize = 10;
pub const MAX_EVENT_LOCATIONS: usize = 5;

// Calendar limits
pub const MAX_CALENDAR_NAME_LENGTH: usize = 100;
pub const MAX_CALENDAR_DESCRIPTION_LENGTH: usize = 10_000;
pub const MAX_CALENDAR_AUTHORS: usize = 20;

// Attendee limits
pub const MAX_ATTENDEE_URI_LENGTH: usize = 2048;

// EventLocation limits
pub const MAX_LOCATION_LABEL_LENGTH: usize = 500;
pub const MAX_LOCATION_DESCRIPTION_LENGTH: usize = 2000;
pub const MAX_LOCATION_URI_LENGTH: usize = 2048;
