#![expect(async_fn_in_trait)]

use embedded_byteorder::{AsyncRead, AsyncWrite};

mod core;
mod heapless;

pub trait AsyncDecode: Sized {
    type Options: Default;
    type Error<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        options: Self::Options,
    ) -> Result<Self, Self::Error<R::Error>>;

    async fn decode_default<R: AsyncRead>(reader: &mut R) -> Result<Self, Self::Error<R::Error>> {
        Self::decode(reader, Default::default()).await
    }
}

pub trait AsyncEncode {
    type Options: Default;
    type Error<E>;

    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        options: Self::Options,
    ) -> Result<(), Self::Error<W::Error>>;

    async fn encode_default<W: AsyncWrite>(
        &self,
        writer: &mut W,
    ) -> Result<(), Self::Error<W::Error>> {
        self.encode(writer, Default::default()).await
    }
}
