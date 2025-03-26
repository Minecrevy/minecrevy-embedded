#![expect(async_fn_in_trait)]

use embedded_byteorder::{AsyncRead, AsyncWrite};

mod core;
mod heapless;

pub trait AsyncDecode: Sized {
    type Options<'a>;
    type Error<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        options: Self::Options<'_>,
    ) -> Result<Self, Self::Error<R::Error>>;

    async fn decode_default<R: AsyncRead>(reader: &mut R) -> Result<Self, Self::Error<R::Error>>
    where
        for<'a> Self::Options<'a>: Default,
    {
        Self::decode(reader, Default::default()).await
    }
}

pub trait AsyncEncode {
    type Options<'a>;
    type Error<E>;

    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        options: Self::Options<'_>,
    ) -> Result<(), Self::Error<W::Error>>;

    async fn encode_default<W: AsyncWrite>(
        &self,
        writer: &mut W,
    ) -> Result<(), Self::Error<W::Error>>
    where
        for<'a> Self::Options<'a>: Default,
    {
        self.encode(writer, Default::default()).await
    }
}
