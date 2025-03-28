use core::convert::Infallible;

use embedded_byteorder::{
    AsyncRead, AsyncReadBytesExt, AsyncWrite, AsyncWriteBytesExt, BigEndian, ReadExactError,
};
use minecrevy_encdec::{AsyncDecode, AsyncEncode, WireSize};
use serde::{Serialize, ser::SerializeMap};

pub struct StatusRequest;

impl AsyncDecode for StatusRequest {
    type Options = ();
    type Error<E> = Infallible;

    async fn decode<R: AsyncRead>(
        _reader: &mut R,
        (): Self::Options,
    ) -> Result<Self, Self::Error<R::Error>> {
        Ok(Self)
    }
}

#[derive(WireSize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct StatusPing(pub i64);

impl AsyncEncode for StatusPing {
    type Options = ();
    type Error<E> = E;

    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        (): Self::Options,
    ) -> Result<(), Self::Error<W::Error>> {
        writer.write_i64::<BigEndian>(self.0).await?;
        Ok(())
    }
}

impl AsyncDecode for StatusPing {
    type Options = ();
    type Error<E> = ReadExactError<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        (): Self::Options,
    ) -> Result<Self, Self::Error<R::Error>> {
        Ok(Self(reader.read_i64::<BigEndian>().await?))
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct StatusResponse<'a> {
    pub version: Version,
    pub players: StatusResponsePlayers,
    pub description: &'a str,
    pub enforces_secure_chat: bool,
}

// TODO: write a serde_json_core::to_writer instead
// impl AsyncEncode for StatusResponse<'_> {
//     type Options<'a> = &'a mut [u8];
//     type Error<E> = E;

//     async fn encode<W: AsyncWrite>(
//         &self,
//         writer: &mut W,
//         buf: Self::Options<'_>,
//     ) -> Result<(), Self::Error<W::Error>> {
//         let written = serde_json_core::to_slice(self, buf).unwrap();
//         writer.write_all(&buf[..written]).await?;
//         Ok(())
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub enum Version {
    V1_21_5,
}

impl Version {
    pub const fn name(&self) -> &'static str {
        match self {
            Version::V1_21_5 => "1.21.5",
        }
    }

    pub const fn protocol(&self) -> i32 {
        match self {
            Version::V1_21_5 => 770,
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("name", self.name())?;
        map.serialize_entry("protocol", &self.protocol())?;
        map.end()
    }
}

#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct StatusResponsePlayers {
    pub max: i32,
    pub online: i32,
}
