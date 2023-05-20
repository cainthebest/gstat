pub mod error;
pub mod traits;

pub use error::{Error, ErrorDetail};
pub use traits::{Parser, Protocol, Query, Response};
