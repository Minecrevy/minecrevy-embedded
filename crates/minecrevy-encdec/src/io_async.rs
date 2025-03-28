#![expect(async_fn_in_trait)]

use embedded_byteorder::{AsyncRead, AsyncWrite};

mod core;
mod heapless;

pub trait AsyncDecode: Sized {
    type Options: Clone + Default;
    type Error<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        options: Self::Options,
    ) -> Result<Self, Self::Error<R::Error>>;
}

pub trait AsyncEncode {
    type Options: Clone + Default;
    type Error<E>;

    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        options: Self::Options,
    ) -> Result<(), Self::Error<W::Error>>;
}
