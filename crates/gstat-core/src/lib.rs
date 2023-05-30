pub mod error;
pub mod standards;
pub mod prelude {
    pub use crate::error::{Error, ErrorDetail};
    pub use crate::standards::game::Game;
    pub use crate::standards::parser::Parser;
    pub use crate::standards::protocol::Protocol;
    pub use crate::standards::query::Query;
    pub use crate::standards::response::Response;
}
