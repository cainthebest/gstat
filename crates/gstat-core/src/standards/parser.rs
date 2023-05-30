use crate::prelude::{Error, ErrorDetail, Query, Response};

use std::{error::Error as StdError, io::Cursor};

/// `Parser` is a trait which outlines the necessary methods for
/// serializing and deserializing data between queries and responses.
///
/// It includes methods for serializing a query into a byte vector, and for
/// deserializing a byte vector into a response.
///
/// The trait is generic over the types of `Query` `Q`, `Response` `R`,
/// Serialization Error `SE`, and Deserialization Error `DE`.
pub trait Parser<'a, Q, R>
where
    Q: Query + 'a,
    R: Response + 'a,
{
    /// The type for serialization errors.
    type SE: StdError + 'static;

    /// The type for deserialization errors.
    type DE: StdError + 'static;

    /// Serialize the provided `query` into a byte vector. If serialization
    /// fails, an Error wrapping the serialization error is returned.
    ///
    /// # Parameters
    ///
    /// * `query`: The query to serialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the serialized `query` as a byte vector or an `Error`.
    fn serialize_query(&self, query: &Q) -> Result<Vec<u8>, Error<Self::SE>> {
        self._serialize_query(query).map_err(|err| {
            Error::ParserError(ErrorDetail::new("Failed to serialize query", Some(err)))
        })
    }

    /// Internal method for serializing the provided `query` into a byte vector.
    ///
    /// # Parameters
    ///
    /// * `query`: The query to serialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the serialized `query` as a byte vector or an `Error`.
    fn _serialize_query(&self, query: &Q) -> Result<Vec<u8>, Self::SE>;

    /// Deserialize a byte stream from a provided Cursor into a `Response`.
    /// If deserialization fails, an `Error` wrapping the deserialization error is returned.
    ///
    /// # Parameters
    ///
    /// * `data`: A Cursor over the data to deserialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the deserialized `Response` or an `Error`.
    fn deserialize_response(&self, data: Cursor<Vec<u8>>) -> Result<R, Error<Self::DE>> {
        self._deserialize_response(data).map_err(|err| {
            Error::ParserError(ErrorDetail::new(
                "Failed to deserialize response",
                Some(err),
            ))
        })
    }

    /// Internal method for deserializing a byte stream from a provided Cursor into a `Response`.
    ///
    /// # Parameters
    ///
    /// * `data`: A Cursor over the data to deserialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the deserialized `Response` or an `Error`.
    fn _deserialize_response(&self, data: Cursor<Vec<u8>>) -> Result<R, Self::DE>;
}
