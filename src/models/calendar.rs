use crate::{
    common::timestamp,
    traits::{HasIdPath, TimestampId, Validatable},
    validation::{is_valid_hex_color, is_valid_timezone},
    EVENTKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// Validation constants
const MIN_NAME_LENGTH: usize = 1;
const MAX_NAME_LENGTH: usize = 100;
const MAX_DESCRIPTION_LENGTH: usize = 10_000;
const MAX_AUTHORS: usize = 20;

/// Represents styled description with metadata for rich content
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct StyledDescription {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub content: String,     // Raw content
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub format: String,      // Format type: "markdown", "html", "plain"
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub attachments: Option<Vec<String>>, // URI references to attached files
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl StyledDescription {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(content: String, format: String, attachments: Option<Vec<String>>) -> Self {
        StyledDescription {
            content,
            format,
            attachments,
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn content(&self) -> String {
        self.content.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn attachments(&self) -> Option<Vec<String>> {
        self.attachments.clone()
    }
}

/// Calendar container - collection of events
/// URI: /pub/eventky.app/calendars/:calendar_id
/// Where calendar_id is a timestamp-based ID (like events)
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct PubkyAppCalendar {
    // RFC 7986 - Calendar Properties
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: String,                      // REQUIRED - calendar display name
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub color: Option<String>,             // CSS color value (hex format #RRGGBB)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub image_uri: Option<String>,         // Calendar image/logo URI (pubky:// or https)

    // RFC 5545 - Calendar Metadata
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub timezone: String,                  // REQUIRED - IANA timezone ID (e.g., "Europe/Zurich")
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,       // Calendar description
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub url: Option<String>,               // Calendar homepage/details URL
    pub created: Option<i64>,              // Creation timestamp (Unix microseconds)

    // Versioning fields (like events) for edit tracking
    pub sequence: Option<i32>,             // Version number, incremented on each edit
    pub last_modified: Option<i64>,        // Last modification timestamp (Unix microseconds)

    // Pubky Extensions (all custom fields use x_pubky_ prefix)
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub x_pubky_authors: Option<Vec<String>>,  // Pubky URIs of users who can add events to this calendar (only owner can edit calendar itself)
}

impl PubkyAppCalendar {
    /// Creates a new `PubkyAppCalendar` instance with required fields and sensible defaults.
    pub fn new(name: String, timezone: String) -> Self {
        let created = Some(timestamp());
        let last_modified = Some(timestamp());
        Self {
            name,
            timezone,
            // Sensible defaults for optional fields
            color: None,
            image_uri: None,
            description: None,
            url: None,
            created,
            x_pubky_authors: None,
            // Versioning fields (like events)
            sequence: Some(0),
            last_modified,
        }
        .sanitize()
    }

    /// Helper methods for setting optional fields
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self.sanitize()
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.sanitize()
    }

    pub fn with_image_uri(mut self, image_uri: String) -> Self {
        self.image_uri = Some(image_uri);
        self.sanitize()
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self.sanitize()
    }

    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        self.x_pubky_authors = Some(authors);
        self.sanitize()
    }

    /// Set sequence number (for versioning)
    pub fn with_sequence(mut self, sequence: i32) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Set last_modified timestamp (for versioning)
    pub fn with_last_modified(mut self, last_modified: i64) -> Self {
        self.last_modified = Some(last_modified);
        self
    }

    /// Increment sequence and update last_modified (call on edit)
    pub fn increment_version(mut self) -> Self {
        self.sequence = Some(self.sequence.unwrap_or(0) + 1);
        self.last_modified = Some(timestamp());
        self
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PubkyAppCalendar {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn color(&self) -> Option<String> {
        self.color.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn image_uri(&self) -> Option<String> {
        self.image_uri.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn timezone(&self) -> String {
        self.timezone.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = "getCreated"))]
    pub fn created(&self) -> Option<i64> {
        self.created
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn x_pubky_authors(&self) -> Option<Vec<String>> {
        self.x_pubky_authors.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = "getSequence"))]
    pub fn sequence(&self) -> Option<i32> {
        self.sequence
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = "getLastModified"))]
    pub fn last_modified(&self) -> Option<i64> {
        self.last_modified
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
impl Json for PubkyAppCalendar {}

// Use TimestampId (like events) instead of HashId
// This allows calendars to be edited while keeping the same ID
impl TimestampId for PubkyAppCalendar {}

impl HasIdPath for PubkyAppCalendar {
    const PATH_SEGMENT: &'static str = "calendars/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, EVENTKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for PubkyAppCalendar {
    fn sanitize(self) -> Self {
        // Sanitize name
        let name = self.name.trim().chars().take(MAX_NAME_LENGTH).collect();
        
        // Sanitize timezone
        let timezone = self.timezone.trim().to_string();
        
        // Sanitize description
        let description = self.description.map(|desc| {
            desc.trim().chars().take(MAX_DESCRIPTION_LENGTH).collect()
        });
        
        // Sanitize color (keep only valid hex colors)
        let color = self.color.and_then(|c| {
            let c = c.trim().to_lowercase();
            if is_valid_hex_color(&c) {
                Some(c)
            } else {
                None
            }
        });
        
        // Sanitize image_uri and url
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
        
        // Sanitize author identifiers - accepts both full pubky URIs and plain public keys
        let x_pubky_authors = self.x_pubky_authors.map(|authors| {
            authors.into_iter()
                .take(MAX_AUTHORS)
                .filter_map(|author_value| {
                    let trimmed = author_value.trim();
                    // If it's a valid URL (full pubky URI), use it
                    if let Ok(url) = Url::parse(trimmed) {
                        return Some(url.to_string());
                    }
                    // If it looks like a public key (alphanumeric, reasonable length), accept it as-is
                    // Public keys are typically 52 characters (z-base-32 encoded)
                    if trimmed.len() >= 32 && trimmed.len() <= 64 && trimmed.chars().all(|c| c.is_alphanumeric()) {
                        return Some(trimmed.to_string());
                    }
                    None
                })
                .collect::<Vec<_>>()
        }).filter(|authors| !authors.is_empty());
        
        Self {
            name,
            color,
            image_uri,
            timezone,
            description,
            url,
            created: self.created,
            x_pubky_authors,
            sequence: self.sequence,
            last_modified: self.last_modified,
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        // Validate the calendar ID
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate name
        let name_length = self.name.chars().count();
        if !(MIN_NAME_LENGTH..=MAX_NAME_LENGTH).contains(&name_length) {
            return Err("Validation Error: Calendar name length must be between 1 and 100 characters".into());
        }

        // Validate timezone
        if self.timezone.trim().is_empty() {
            return Err("Validation Error: Timezone is required".into());
        }
        
        if !is_valid_timezone(&self.timezone) {
            return Err("Validation Error: Invalid timezone format. Must be a valid IANA timezone ID".into());
        }

        // Validate description length
        if let Some(desc) = &self.description {
            if desc.chars().count() > MAX_DESCRIPTION_LENGTH {
                return Err("Validation Error: Description exceeds maximum length".into());
            }
        }

        // Validate color format
        if let Some(color) = &self.color {
            if !is_valid_hex_color(color) {
                return Err("Validation Error: Color must be a valid hex color (#RRGGBB)".into());
            }
        }

        // Validate author count
        if let Some(authors) = &self.x_pubky_authors {
            if authors.len() > MAX_AUTHORS {
                return Err("Validation Error: Too many calendar authors".into());
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
    fn test_new() {
        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        )
        .with_color("#3498db".to_string())
        .with_description("My work events".to_string());

        assert_eq!(calendar.name, "Work Calendar");
        assert_eq!(calendar.timezone, "Europe/Zurich");
        assert_eq!(calendar.color, Some("#3498db".to_string()));
        assert_eq!(calendar.description, Some("My work events".to_string()));
        assert!(calendar.created.is_some());
        
        // Check that created timestamp is recent
        let now = timestamp();
        assert!(calendar.created.unwrap() <= now && calendar.created.unwrap() >= now - 1_000_000);
    }

    #[test]
    fn test_create_id() {
        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        )
        .with_color("#3498db".to_string())
        .with_description("My work events".to_string());

        let calendar_id = calendar.create_id();
        println!("Generated Calendar ID: {}", calendar_id);

        // The ID should not be empty
        assert!(!calendar_id.is_empty());
    }

    #[test]
    fn test_create_path() {
        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        );

        let calendar_id = calendar.create_id();
        let path = PubkyAppCalendar::create_path(&calendar_id);

        // Check if the path starts with the expected prefix
        let prefix = format!("{}{}calendars/", PUBLIC_PATH, EVENTKY_PATH);
        assert!(path.starts_with(&prefix));

        let expected_path_len = prefix.len() + calendar_id.len();
        assert_eq!(path.len(), expected_path_len);
    }

    #[test]
    fn test_validate_valid() {
        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        )
        .with_color("#3498db".to_string())
        .with_description("My work events".to_string());

        let id = calendar.create_id();
        let result = calendar.validate(Some(&id));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_name() {
        let calendar = PubkyAppCalendar::new(
            "".to_string(), // Empty name
            "Europe/Zurich".to_string(),
        );

        let id = calendar.create_id();
        let result = calendar.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Calendar name length"));
    }

    #[test]
    fn test_validate_invalid_timezone() {
        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "invalid_timezone".to_string(), // Invalid timezone
        );

        let id = calendar.create_id();
        let result = calendar.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid timezone format"));
    }

    #[test]
    fn test_validate_invalid_color() {
        let mut calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        );
        
        // Manually set invalid color after creation to bypass sanitization
        calendar.color = Some("invalid_color".to_string());

        let id = calendar.create_id();
        let result = calendar.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("valid hex color"));
    }

    #[test]
    fn test_sanitize() {
        let calendar = PubkyAppCalendar::new(
            "  Work Calendar  ".to_string(),
            " Europe/Zurich ".to_string(),
        )
        .with_color("  #3498DB  ".to_string())
        .with_image_uri("invalid_url".to_string())
        .with_description("  Description  ".to_string())
        .with_authors(vec!["invalid_uri".to_string()]);

        assert_eq!(calendar.name, "Work Calendar");
        assert_eq!(calendar.timezone, "Europe/Zurich");
        assert_eq!(calendar.color, Some("#3498db".to_string()));
        assert!(calendar.image_uri.is_none()); // Invalid URL sanitized out
        assert_eq!(calendar.description, Some("Description".to_string()));
        assert!(calendar.x_pubky_authors.is_none()); // Invalid URIs filtered out
    }

    #[test]
    fn test_try_from_valid() {
        let calendar_json = r##"
        {
            "name": "Work Calendar",
            "color": "#3498db",
            "image_uri": null,
            "timezone": "Europe/Zurich",
            "description": "My work events",
            "url": null,
            "created": 1700000000,
            "x_pubky_authors": null
        }
        "##;

        let calendar = PubkyAppCalendar::new(
            "Work Calendar".to_string(),
            "Europe/Zurich".to_string(),
        )
        .with_color("#3498db".to_string())
        .with_description("My work events".to_string());
        let id = calendar.create_id();

        let blob = calendar_json.as_bytes();
        let calendar_parsed = <PubkyAppCalendar as Validatable>::try_from(blob, &id).unwrap();

        assert_eq!(calendar_parsed.name, "Work Calendar");
        assert_eq!(calendar_parsed.timezone, "Europe/Zurich");
        assert_eq!(calendar_parsed.color, Some("#3498db".to_string()));
        assert_eq!(calendar_parsed.description, Some("My work events".to_string()));
    }
}