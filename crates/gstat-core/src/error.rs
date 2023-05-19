use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

/// `ErrorDetail` is a structure that encapsulates an error message and its associated data.
///
/// `E` is the type of the error data that can be associated with the error message.
pub struct ErrorDetail<E> {
    /// The error message.
    message: String,
    /// The optional data associated with the error.
    inner: Option<E>,
}

impl<E> ErrorDetail<E> {
    /// Creates a new `ErrorDetail` instance.
    ///
    /// # Parameters
    ///
    /// * `message`: The error message.
    /// * `inner`: The optional data to associate with the error.
    pub fn new(message: &str, inner: Option<E>) -> Self {
        ErrorDetail {
            message: message.to_string(),
            inner,
        }
    }

    /// Formats the error message and its associated category for display.
    ///
    /// # Parameters
    ///
    /// * `f`: The formatter.
    /// * `category`: The category of the error.
    fn display(&self, f: &mut Formatter<'_>, category: &str) -> FmtResult {
        write!(f, "[GSTAT ERROR ({}): {}", category, self.message)
    }
}

/// Represents various kinds of errors that can occur in GSTAT.
///
/// `E` is the type of the error data that can be associated with the error message.
pub enum Error<E> {
    /// An error that occurred within the game logic.
    GameError(ErrorDetail<E>),
    /// An error that occurred while parsing.
    ParserError(ErrorDetail<E>),
    /// An error that occurred within the communication protocol.
    ProtocolError(ErrorDetail<E>),
    /// An error that occurred while creating or using a query.
    QueryError(ErrorDetail<E>),
    /// An error that occurred while receiving or processing a response.
    ResponseError(ErrorDetail<E>),
}

impl<E: Debug> Display for Error<E> {
    /// Formats the error for display.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GameError(detail) => detail.display(f, "Game"),
            Self::ParserError(detail) => detail.display(f, "Parser"),
            Self::ProtocolError(detail) => detail.display(f, "Protocol"),
            Self::QueryError(detail) => detail.display(f, "Query"),
            Self::ResponseError(detail) => detail.display(f, "Response"),
        }
    }
}

impl<E: Debug> Debug for Error<E> {
    /// Formats the error for debugging purposes.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GameError(detail) => f
                .debug_struct("GameError")
                .field("message", &detail.message)
                .field("inner", &detail.inner)
                .finish(),

            Self::ParserError(detail) => f
                .debug_struct("ParserError")
                .field("message", &detail.message)
                .field("inner", &detail.inner)
                .finish(),

            Self::ProtocolError(detail) => f
                .debug_struct("ProtocolError")
                .field("message", &detail.message)
                .field("inner", &detail.inner)
                .finish(),

            Self::QueryError(detail) => f
                .debug_struct("QueryError")
                .field("message", &detail.message)
                .field("inner", &detail.inner)
                .finish(),

            Self::ResponseError(detail) => f
                .debug_struct("ResponseError")
                .field("message", &detail.message)
                .field("inner", &detail.inner)
                .finish(),
        }
    }
}

/// Allows `Error` to be treated like a standard library error.
impl<E: Debug + 'static> StdError for Error<E> {}
