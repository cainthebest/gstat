use crate::prelude::{Error, Parser, Query, Response};

use std::{error::Error as StdError, net::SocketAddr};

use async_trait::async_trait;

/// A trait defining the standard behavior of a network protocol.
///
/// `Protocol` is an asynchronous trait that provides a common interface for various network protocols.
/// It allows for sending and receiving queries and responses, as well as establishing and disconnecting connections.
/// Each operation is asynchronous and returns a `Result` to facilitate error handling.
///
/// This trait uses associated types for Query `Q`, Response `R`, Parser `P` and Error `E` allowing flexibility for various network protocols.
#[async_trait]
pub trait Protocol<'a> 
where
    Self: Send + Sync + Sized,
{
    /// The type of query that can be sent over this protocol.
    /// It must be thread-safe and have a statically known size.
    type Q: Query + 'a;

    /// The type of response that can be received over this protocol.
    /// It must be thread-safe.
    type R: Response + 'a;

    /// The type of parser that can parse a Query into a specific type and a response into a Response type.
    type P: Parser<'a, Self::Q, Self::R>;

    /// The type of error that can occur when using this protocol.
    type E: StdError;

    /// Connect to a specific IP address asynchronously.
    ///
    /// This method attempts to establish a network connection with a server or network device at the specified IP address.
    ///
    /// # Parameters
    ///
    /// * `address`: The target IP address for connection establishment.
    async fn connect(&self, address: SocketAddr) -> Result<(), Error<Self::E>>;

    /// Send a query to the connected server or device asynchronously.
    ///
    /// The query is processed through the associated Parser type before being sent across the network.
    ///
    /// # Parameters
    ///
    /// * `query`: The query object to be sent.
    async fn send_query(&self, query: Self::Q) -> Result<(), Error<Self::E>>;

    /// Receive a response from the connected server or device asynchronously.
    ///
    /// The received response is parsed using the associated Parser into the Response type.
    async fn receive_response(&self) -> Result<Self::R, Error<Self::E>>;

    /// Disconnect from the connected server or device asynchronously.
    ///
    /// This method closes the active network connection or session.
    async fn disconnect(&self) -> Result<(), Error<Self::E>>;

    /// Send a data packet over the network asynchronously.
    ///
    /// This method is intended to send raw bytes and does not involve the associated Query or Response types.
    ///
    /// # Parameters
    ///
    /// * `data`: The raw data to be sent across the network.

    // This should be classed as a unsafe function as it is not bound by the library
    async fn send(&self, data: &[u8]) -> Result<(), Error<Self::E>>;

    /// Receive a data packet from the network asynchronously.
    ///
    /// This method retrieves raw data from the network and does not involve the associated Query or Response types.

    // This should be classed as a unsafe function as it is not bound by the library
    async fn receive(&self) -> Result<Vec<u8>, Error<Self::E>>;
}
