use embedded_byteorder::{AsyncRead, ReadExactError};
use heapless::String;
use minecrevy_bytes::ReadMinecraftError;
use minecrevy_encdec::{AsyncDecode, options::IntOptions};
use thiserror::Error;

#[derive(Clone, PartialEq, Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String<255>,
    pub server_port: u16,
    pub next_state: NextState,
}

#[derive(Error, Debug)]
#[error("failed to read handshake: {0}")]
pub enum DecodeHandshakeError<E> {
    InvalidNextState(#[from] InvalidNextStateError),
    Io(#[from] ReadMinecraftError<E>),
}

impl<E> From<ReadExactError<E>> for DecodeHandshakeError<E> {
    fn from(err: ReadExactError<E>) -> Self {
        Self::Io(err.into())
    }
}

impl AsyncDecode for Handshake {
    type Options<'a> = ();
    type Error<E> = DecodeHandshakeError<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        (): Self::Options<'_>,
    ) -> Result<Self, Self::Error<R::Error>> {
        Ok(Self {
            protocol_version: i32::decode(reader, IntOptions { varint: true }).await?,
            server_address: String::decode(reader, ()).await?,
            server_port: u16::decode(reader, ()).await?,
            next_state: i32::decode(reader, IntOptions { varint: true })
                .await?
                .try_into()?,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NextState {
    Status,
    Login,
    Transfer,
}

#[derive(Error, Debug)]
#[error("Invalid next state ID: {0}")]
pub struct InvalidNextStateError(pub i32);

impl TryFrom<i32> for NextState {
    type Error = InvalidNextStateError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            3 => Ok(NextState::Transfer),
            _ => Err(InvalidNextStateError(value)),
        }
    }
}
