use crate::{Error, Parser, Query, Response};

use std::{error::Error as StdError, net::Ipv4Addr};

use async_trait::async_trait;

/// `Protocol` trait defines the contract for a network protocol.
///
/// This trait provides a generic way to interact with network protocols. It includes
/// connecting to an address, disconnecting, sending queries and receiving responses. These
/// operations are asynchronous and return a `Result` for error handling.
///
/// The trait is generic over the types of Query `Q`, Response `R`, Parser `P` and Error `E`.
#[async_trait]
pub trait Protocol
where
    <Self as Protocol>::Q: Send,
{
    /// The type of the query sent over the protocol.
    type Q: Query;
    /// The type of the response received from the protocol.
    type R: Response;
    /// The type of the parser that parses a Query into a query and a response into a Response.
    type P: Parser<Self::Q, Self::R>;
    /// The type of the error.
    type E: StdError + 'static;

    /// Asynchronously connects to the provided IP address.
    ///
    /// Establishes a connection or a session with the server or the network device.
    ///
    /// # Parameters
    ///
    /// * `address`: The IP address to connect to.
    async fn connect(&self, address: &Ipv4Addr) -> Result<(), Error<Self::E>>;

    /// Asynchronously sends a query to the connected device or server.
    ///
    /// The query is parsed using the defined parser before being sent over the network.
    ///
    /// # Parameters
    ///
    /// * `query`: The query to send.
    async fn send_query(&self, query: Self::Q) -> Result<(), Error<Self::E>>;

    /// Asynchronously receives a response from the connected device or server.
    ///
    /// The response is parsed using the defined parser into a predefined structure.
    async fn receive_response(&self) -> Result<Self::R, Error<Self::E>>;

    /// Asynchronously disconnects from the connected device or server.
    ///
    /// Closes the connection or session with the device or server.
    async fn disconnect(&self) -> Result<(), Error<Self::E>>;

    /// Asynchronously sends a data packet over the network.
    ///
    /// The data to send can be of any type that can be converted into bytes.
    ///
    /// # Parameters
    ///
    /// * `data`: The data to send.
    async fn send(&self, data: &[u8]) -> Result<(), Error<Self::E>>;

    /// Asynchronously receives a data packet from the network.
    ///
    /// Returns the received data as bytes.
    async fn receive(&self) -> Result<Vec<u8>, Error<Self::E>>;
}
