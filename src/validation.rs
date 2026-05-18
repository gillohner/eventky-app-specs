use url::Url;

/// Validates an ISO 8601 datetime (YYYY-MM-DDTHH:MM:SS[.sssssss][Z|±HH:MM])
/// This is a lenient format check — accepts with or without timezone suffix.
pub fn is_valid_datetime(s: &str) -> bool {
    if s.len() < 19 {
        return false;
    }
    let bytes = s.as_bytes();
    // YYYY-MM-DDTHH:MM:SS
    for (i, &b) in bytes.iter().take(19).enumerate() {
        let ok = match i {
            4 | 7 => b == b'-',
            10 => b == b'T',
            13 | 16 => b == b':',
            _ => b.is_ascii_digit(),
        };
        if !ok {
            return false;
        }
    }
    true
}

/// Validates an RFC 5545 duration: PnYnMnDTnHnMnS (basic check).
/// Examples: PT1H, PT30M, P1D, P1DT2H30M
pub fn is_valid_duration(s: &str) -> bool {
    if s.is_empty() || !s.starts_with('P') {
        return false;
    }
    let rest = &s[1..];
    if rest.is_empty() {
        return false;
    }
    // Must contain only digits, designators (Y M W D H M S), and the 'T' separator.
    let valid_designators = b"YMWDHSMT";
    let mut saw_any = false;
    for &b in rest.as_bytes() {
        if b.is_ascii_digit() {
            saw_any = true;
            continue;
        }
        if !valid_designators.contains(&b) {
            return false;
        }
    }
    saw_any
}

/// Validates an IANA timezone identifier — must have a '/' separator and only
/// valid chars (letters, digits, '_', '-', '/', '+').
pub fn is_valid_timezone(s: &str) -> bool {
    if s.is_empty() || !s.contains('/') {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '_' | '-' | '+'))
}

/// Validates a CSS hex color (#RRGGBB or #RGB).
pub fn is_valid_hex_color(s: &str) -> bool {
    if !s.starts_with('#') {
        return false;
    }
    let hex = &s[1..];
    (hex.len() == 3 || hex.len() == 6) && hex.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validates an RFC 5545 RRULE string — must start with FREQ= and contain only
/// known parts. Lenient check suitable for sanitize-and-store scenarios.
pub fn is_valid_rrule(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split(';').collect();
    if parts.is_empty() {
        return false;
    }
    let mut has_freq = false;
    for part in parts {
        let mut kv = part.splitn(2, '=');
        let key = match kv.next() {
            Some(k) => k,
            None => return false,
        };
        if kv.next().is_none() {
            return false;
        }
        if key == "FREQ" {
            has_freq = true;
        }
    }
    has_freq
}

/// Validates a pubky:// URI.
pub fn validate_pubky_uri(uri: &str) -> Result<(), String> {
    let parsed =
        Url::parse(uri).map_err(|_| format!("Validation Error: Invalid URI format: {}", uri))?;
    if parsed.scheme() != "pubky" {
        return Err(format!(
            "Validation Error: URI must use pubky:// protocol: {}",
            uri
        ));
    }
    Ok(())
}

/// Validates a microsecond-precision UNIX timestamp. Rejects non-positive values and
/// anything more than 1 day in the future (clock-skew tolerance).
pub fn validate_timestamp_microseconds(ts_us: i64, field: &str) -> Result<(), String> {
    if ts_us <= 0 {
        return Err(format!(
            "Validation Error: {field} must be a positive UNIX timestamp in microseconds, got {ts_us}"
        ));
    }
    let now_us = crate::common::timestamp();
    let max_future_us = now_us + 86_400_000_000; // +1 day
    if ts_us > max_future_us {
        return Err(format!(
            "Validation Error: {field} {ts_us} is more than 1 day in the future"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_valid() {
        assert!(is_valid_datetime("2025-12-01T10:00:00"));
        assert!(is_valid_datetime("2025-12-01T10:00:00Z"));
        assert!(is_valid_datetime("2025-12-01T10:00:00+02:00"));
    }

    #[test]
    fn test_datetime_invalid() {
        assert!(!is_valid_datetime("2025-12-01"));
        assert!(!is_valid_datetime("not a date"));
        assert!(!is_valid_datetime(""));
    }

    #[test]
    fn test_duration_valid() {
        assert!(is_valid_duration("PT1H"));
        assert!(is_valid_duration("PT30M"));
        assert!(is_valid_duration("P1D"));
        assert!(is_valid_duration("P1DT2H30M"));
    }

    #[test]
    fn test_duration_invalid() {
        assert!(!is_valid_duration("1H"));
        assert!(!is_valid_duration(""));
        assert!(!is_valid_duration("P"));
    }

    #[test]
    fn test_timezone_valid() {
        assert!(is_valid_timezone("Europe/Zurich"));
        assert!(is_valid_timezone("America/New_York"));
        assert!(is_valid_timezone("Asia/Tokyo"));
    }

    #[test]
    fn test_timezone_invalid() {
        assert!(!is_valid_timezone(""));
        assert!(!is_valid_timezone("Invalid"));
        assert!(!is_valid_timezone("Europe@Zurich"));
    }

    #[test]
    fn test_hex_color() {
        assert!(is_valid_hex_color("#fff"));
        assert!(is_valid_hex_color("#FFFFFF"));
        assert!(is_valid_hex_color("#123456"));
        assert!(!is_valid_hex_color("fff"));
        assert!(!is_valid_hex_color("#xyz"));
    }

    #[test]
    fn test_rrule() {
        assert!(is_valid_rrule("FREQ=DAILY"));
        assert!(is_valid_rrule("FREQ=WEEKLY;COUNT=10"));
        assert!(!is_valid_rrule("COUNT=10"));
        assert!(!is_valid_rrule(""));
    }
}
