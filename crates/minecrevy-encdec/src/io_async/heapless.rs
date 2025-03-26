use embedded_byteorder::{AsyncRead, AsyncWrite};
use heapless::String;
use minecrevy_bytes::{AsyncReadMinecraftExt, AsyncWriteMinecraftExt, ReadMinecraftError};

use crate::{AsyncDecode, AsyncEncode};

impl<const N: usize> AsyncDecode for String<N> {
    type Options<'a> = ();
    type Error<E> = ReadMinecraftError<E>;

    async fn decode<R: AsyncRead>(
        reader: &mut R,
        (): Self::Options<'_>,
    ) -> Result<Self, Self::Error<R::Error>> {
        reader.read_string().await
    }
}

impl<const N: usize> AsyncEncode for String<N> {
    type Options<'a> = ();
    type Error<E> = E;

    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        (): Self::Options<'_>,
    ) -> Result<(), Self::Error<W::Error>> {
        writer.write_string(self).await
    }
}
