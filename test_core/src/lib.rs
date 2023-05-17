pub mod questions;
pub mod answers;


pub mod json {
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{
        Serializer,
        Deserializer,
        Value,
        json,
        from_str,
        from_slice,
        from_value,
    };

    #[cfg(debug_assertions)]
    pub use serde_json::ser::{ PrettyFormatter as Formatter, to_string_pretty as to_str };

    #[cfg(not(debug_assertions))]
    pub use serde_json::ser::{ CompactFormatter as Formatter, to_string as to_str };
}

