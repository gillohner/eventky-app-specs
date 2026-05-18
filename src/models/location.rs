use crate::constants::{
    MAX_LOCATION_DESCRIPTION_LENGTH, MAX_LOCATION_LABEL_LENGTH, MAX_LOCATION_URI_LENGTH,
};
use crate::traits::Validatable;
use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Kind of event location. PHYSICAL places typically have an OSM URL as `uri`;
/// VIRTUAL locations carry a meeting URL (Zoom, Jitsi, Google Meet, etc.).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "UPPERCASE")]
pub enum EventLocationKind {
    #[default]
    Physical,
    Virtual,
}

/// Slim event location — URI-first, with human-readable label and optional
/// description. Designed to co-exist with the universal-tags convention:
/// a `PubkyAppTag` targeting the same `uri` creates a cross-app edge in nexus.
///
/// Field shape:
/// - `label` — required display name ("Main Hall", "Weekly Standup").
/// - `uri` — optional canonical reference: OSM URL for physical places,
///   meeting URL for virtual, or future `pubky://.../places/...` for custom.
/// - `kind` — PHYSICAL | VIRTUAL; indicates intent when `uri` is absent.
/// - `description` — optional extra detail ("Building 5, 3rd floor").
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct EventLocation {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub label: String,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub uri: Option<String>,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub kind: EventLocationKind,

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,
}

impl Default for EventLocation {
    fn default() -> Self {
        Self {
            label: String::new(),
            uri: None,
            kind: EventLocationKind::Physical,
            description: None,
        }
    }
}

impl EventLocation {
    pub fn physical(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            kind: EventLocationKind::Physical,
            ..Default::default()
        }
    }

    pub fn physical_with_uri(label: impl Into<String>, osm_url: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            uri: Some(osm_url.into()),
            kind: EventLocationKind::Physical,
            description: None,
        }
    }

    pub fn virtual_meeting(label: impl Into<String>, meeting_url: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            uri: Some(meeting_url.into()),
            kind: EventLocationKind::Virtual,
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for EventLocation {}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EventLocation {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn label(&self) -> String {
        self.label.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn uri(&self) -> Option<String> {
        self.uri.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

impl Validatable for EventLocation {
    fn sanitize(self) -> Self {
        let label = self
            .label
            .trim()
            .chars()
            .take(MAX_LOCATION_LABEL_LENGTH)
            .collect::<String>();

        let description = self.description.and_then(|d| {
            let trimmed = d
                .trim()
                .chars()
                .take(MAX_LOCATION_DESCRIPTION_LENGTH)
                .collect::<String>();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });

        let uri = self.uri.and_then(|u| {
            let trimmed = u.trim();
            if trimmed.is_empty() || trimmed.len() > MAX_LOCATION_URI_LENGTH {
                None
            } else {
                Url::parse(trimmed)
                    .ok()
                    .map(|parsed| parsed.to_string())
                    .or_else(|| Some(trimmed.to_string()))
            }
        });

        Self {
            label,
            uri,
            kind: self.kind,
            description,
        }
    }

    fn validate(&self, _id: Option<&str>) -> Result<(), String> {
        if self.label.is_empty() {
            return Err("Validation Error: EventLocation label must not be empty".into());
        }
        if self.label.chars().count() > MAX_LOCATION_LABEL_LENGTH {
            return Err(format!(
                "Validation Error: EventLocation label exceeds {} characters",
                MAX_LOCATION_LABEL_LENGTH
            ));
        }
        if let Some(ref uri) = self.uri {
            if uri.len() > MAX_LOCATION_URI_LENGTH {
                return Err(format!(
                    "Validation Error: EventLocation uri exceeds {} characters",
                    MAX_LOCATION_URI_LENGTH
                ));
            }
            Url::parse(uri).map_err(|_| {
                "Validation Error: EventLocation uri must be a valid URL".to_string()
            })?;
        }
        if let Some(ref desc) = self.description {
            if desc.chars().count() > MAX_LOCATION_DESCRIPTION_LENGTH {
                return Err(format!(
                    "Validation Error: EventLocation description exceeds {} characters",
                    MAX_LOCATION_DESCRIPTION_LENGTH
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_without_uri() {
        let loc = EventLocation::physical("My backyard");
        assert_eq!(loc.label, "My backyard");
        assert_eq!(loc.kind, EventLocationKind::Physical);
        assert!(loc.uri.is_none());
        assert!(loc.validate(None).is_ok());
    }

    #[test]
    fn physical_with_osm_url() {
        let loc = EventLocation::physical_with_uri(
            "Big Ben",
            "https://www.openstreetmap.org/node/10176085",
        );
        assert_eq!(loc.label, "Big Ben");
        assert_eq!(
            loc.uri.as_deref(),
            Some("https://www.openstreetmap.org/node/10176085")
        );
        assert!(loc.validate(None).is_ok());
    }

    #[test]
    fn virtual_meeting() {
        let loc = EventLocation::virtual_meeting("Weekly Standup", "https://meet.google.com/abc");
        assert_eq!(loc.kind, EventLocationKind::Virtual);
        assert!(loc.validate(None).is_ok());
    }

    #[test]
    fn sanitize_trims_and_truncates() {
        let loc = EventLocation {
            label: "  Hi  ".to_string(),
            uri: None,
            kind: EventLocationKind::Physical,
            description: Some("  detail  ".to_string()),
        }
        .sanitize();
        assert_eq!(loc.label, "Hi");
        assert_eq!(loc.description.as_deref(), Some("detail"));
    }

    #[test]
    fn invalid_uri_rejected() {
        let loc = EventLocation {
            label: "x".to_string(),
            uri: Some("not a url".to_string()),
            kind: EventLocationKind::Physical,
            description: None,
        };
        assert!(loc.validate(None).is_err());
    }
}
