use defmt::Format;
use embedded_byteorder::ReadExactError;
use thiserror::Error;

#[derive(Format, Error, Clone, PartialEq, Eq, Debug)]
pub enum ReadMinecraftError<E> {
    #[error("invalid UTF-8")]
    InvalidUtf8,
    #[error("length exceeded")]
    LengthExceeded,
    #[error("varint too big")]
    VarIntTooBig,
    #[error("varint incomplete")]
    VarIntIncomplete,
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("other error: {0}")]
    Other(#[from] E),
}

impl<E> From<ReadExactError<E>> for ReadMinecraftError<E> {
    fn from(value: ReadExactError<E>) -> Self {
        match value {
            ReadExactError::UnexpectedEof => ReadMinecraftError::UnexpectedEof,
            ReadExactError::Other(e) => ReadMinecraftError::Other(e),
        }
    }
}

/// Error type for reading a Minecraft packet.
///
/// `FE` stands for "frame error" and `DE` stands for "data error".
#[derive(Format, Error, Clone, PartialEq, Eq, Debug)]
pub enum ReadPacketError<FE, DE> {
    #[error("failed to read packet length: {0}")]
    Length(ReadMinecraftError<FE>),
    #[error("failed to read packet id: {0}")]
    Id(ReadMinecraftError<FE>),
    #[error("failed to read packet body with id {0}: {1}")]
    Body(i32, DE),
}

#[derive(Format, Error, Clone, PartialEq, Eq, Debug)]
pub enum WriteMinecraftError<E> {
    #[error("out of memory")]
    OutOfMemory,
    #[error("other error: {0}")]
    Other(#[from] E),
}

/// Error type for writing a Minecraft packet.
///
/// `FE` stands for "frame error" and `DE` stands for "data error".
#[derive(Format, Error, Clone, PartialEq, Eq, Debug)]
pub enum WritePacketError<FE, DE> {
    #[error("failed to write packet length: {0}")]
    Length(FE),
    #[error("failed to write packet id: {0}")]
    Id(FE),
    #[error("failed to write packet body with id {0}: {1}")]
    Body(i32, DE),
}
