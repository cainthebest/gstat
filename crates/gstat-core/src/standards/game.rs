use crate::prelude::{Error, Protocol};

use std::net::SocketAddr;

use async_trait::async_trait;

/// The `Game` trait represents a specific game that can interact with a game server.
///
/// It provides an associated type for the specific `Protocol` to be used for network operations.
/// It also provides associated constants for the game's name and the year it was released,
/// and a method to fetch data from a game server.
#[async_trait]
pub trait Game<'a, P>
where
    P: Protocol<'a>,
{
    /// The name of the game.
    const GAME_NAME: &'static str;

    /// The year the game was released.
    const RELEASE_YEAR: u32;

    /// Provides a new instance of the protocol.
    ///
    /// This internal method is intended to allow the use of the protocol in the `fetch`
    /// method without causing lifetime issues or requiring cloning.
    fn _protocol(&self) -> P;

    /// Fetches data from the game server.
    ///
    /// This asynchronous method performs several operations. First, it connects to the game
    /// server using the provided protocol. Then, it sends a query to the server. The specifics
    /// of this query are determined by the `query` parameter. After sending the query, it
    /// waits for a response from the server.
    ///
    /// The method returns the response from the server, parsed into the appropriate type
    /// determined by the protocol. If any errors occur during these operations, it returns
    /// an `Error` variant instead.
    ///
    /// # Parameters
    ///
    /// * `query`: The query to send to the server.
    /// * `address`: The address of the server.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed server response or an `Error`.
    async fn fetch(&'a self, query: P::Q, address: SocketAddr) -> Result<P::R, Error<P::E>> {
        let protocol = self._protocol();

        protocol.connect(address).await?;
        protocol.send_query(query).await?;

        let response = protocol.receive_response().await?;

        protocol.disconnect().await?;
        Ok(response)
    }
}
