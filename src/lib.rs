pub use error::{Error, Result};
pub use value::Value;
pub use ser::to_bytes;
pub use de::from_bytes;

mod de;
mod error;
mod ser;
mod value;
