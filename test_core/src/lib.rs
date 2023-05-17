pub mod answers;
pub mod questions;

pub mod json {
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{from_slice, from_str, from_value, json, Deserializer, Serializer, Value};
    pub use serde_json::{Error as JsonError, Result as JsonResult};

    #[cfg(debug_assertions)]
    pub use serde_json::ser::{to_string_pretty as to_str, PrettyFormatter as Formatter};

    #[cfg(not(debug_assertions))]
    pub use serde_json::ser::{to_string as to_str, CompactFormatter as Formatter};
}
