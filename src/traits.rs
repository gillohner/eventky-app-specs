pub use pubky_app_specs::traits::{HasIdPath, HasPath, HashId, TimestampId, Validatable};

#[cfg(target_arch = "wasm32")]
pub use pubky_app_specs::traits::Json;
