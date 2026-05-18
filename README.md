> **Warning:** This project is in early development. APIs and data models may change without notice.

# eventky-app-specs

Rust/WASM type library defining Eventky data models and builders.
It extends and reuses upstream [`pubky-app-specs`](https://github.com/pubky/pubky-app-specs).

## Models

| Model | Path | ID Type | Description |
|---|---|---|---|
| `PubkyAppCalendar` | `/pub/eventky.app/calendars/<id>` | TimestampId | Calendar metadata and sharing config |
| `PubkyAppEvent` | `/pub/eventky.app/events/<id>` | TimestampId | Event details (RFC 5545/7986 style fields) |
| `PubkyAppAttendee` | `/pub/eventky.app/attendees/<id>` | HashId | RSVP/attendance relation for events |
| `PubkyAppTag` (eventky namespace) | `/pub/eventky.app/tags/<id>` | HashId | Universal tag shape stored in Eventky namespace |

## Notes

- Eventky keeps calendar/event/attendee records in the `eventky.app` namespace.
- Tag objects use the universal `PubkyAppTag` structure, but are written under `/pub/eventky.app/tags/*`.
- This crate is used by Eventky frontend and Eventky Nexus plugin for consistent IDs and validation.

## Build

```sh
# Run tests
cargo test

# Build WASM package
wasm-pack build --target bundler --release
```

## Publishing

- Crate: `eventky-app-specs` (crates.io)
- npm package: `eventky-app-specs`

Tag format must match `Cargo.toml` version:

```sh
git tag vX.Y.Z
git push origin vX.Y.Z
```
