use crate::prelude::Error;

use std::error::Error as StdError;

/// The `Response` trait represents a type that encapsulates the data received from a protocol.
///
/// This trait is generic over the type of Response Error `E`.
pub trait Response 
where
    Self: Send + Sync + Sized,
{
    /// The type for response errors.
    type E: StdError + 'static;

    /// Creates a new instance of the Response.
    ///
    /// This method is expected to return a `Result` containing the newly created
    /// `Response` or an `Error` if the instantiation fails.
    ///
    /// The Response instance can then be used to represent the received data from a protocol.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a new instance of the Response or an `Error`.
    fn new() -> Result<Self, Error<Self::E>>;

    // Add more response specific methods
    // Keep in mind this is about managing response data, not its serialization or deserialization
}
