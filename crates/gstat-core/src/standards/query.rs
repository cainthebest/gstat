use crate::prelude::Error;

use std::error::Error as StdError;

/// A `Query` trait represents a type that can be instantiated and then sent to a protocol.
///
/// This trait is generic over the type of Query Error `E`.
pub trait Query: Sized {
    /// The type for query errors.
    type E: StdError + 'static;

    /// Creates a new instance of the Query.
    ///
    /// This method is expected to return a `Result` containing the newly created
    /// `Query` or an `Error` if the instantiation fails.
    ///
    /// This query can later be used with a protocol to send a request or a command.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a new instance of the Query or an `Error`.
    fn new() -> Result<Self, Error<Self::E>>;
}
