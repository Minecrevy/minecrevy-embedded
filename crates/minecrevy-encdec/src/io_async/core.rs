use embedded_byteorder::{
    AsyncRead, AsyncReadBytesExt, AsyncWrite, AsyncWriteBytesExt, BigEndian, ReadExactError,
};
use minecrevy_bytes::{AsyncReadMinecraftExt, AsyncWriteMinecraftExt, ReadMinecraftError};

use crate::{AsyncDecode, AsyncEncode, options::IntOptions};

macro_rules! impl_primitive {
    ($($ty:ty: $dec:expr, $enc:expr;)*) => {
        $(
            impl AsyncDecode for $ty {
                type Options = ();
                type Error<E> = ReadExactError<E>;

                #[inline]
                async fn decode<R: AsyncRead>(reader: &mut R, (): ()) -> Result<Self, Self::Error<R::Error>>
                {
                    $dec(reader).await
                }
            }

            impl AsyncEncode for $ty {
                type Options = ();
                type Error<E> = E;

                #[inline]
                async fn encode<W: AsyncWrite>(&self, writer: &mut W, (): ()) -> Result<(), Self::Error<W::Error>>
                {
                    $enc(writer, *self).await
                }
            }
        )*
    };
}

impl_primitive!(
    u8: AsyncReadBytesExt::read_u8, AsyncWriteBytesExt::write_u8;
    i8: AsyncReadBytesExt::read_i8, AsyncWriteBytesExt::write_i8;
    u16: AsyncReadBytesExt::read_u16::<BigEndian>, AsyncWriteBytesExt::write_u16::<BigEndian>;
    i16: AsyncReadBytesExt::read_i16::<BigEndian>, AsyncWriteBytesExt::write_i16::<BigEndian>;
    u32: AsyncReadBytesExt::read_u32::<BigEndian>, AsyncWriteBytesExt::write_u32::<BigEndian>;
    u64: AsyncReadBytesExt::read_u64::<BigEndian>, AsyncWriteBytesExt::write_u64::<BigEndian>;
    i64: AsyncReadBytesExt::read_i64::<BigEndian>, AsyncWriteBytesExt::write_i64::<BigEndian>;
    f32: AsyncReadBytesExt::read_f32::<BigEndian>, AsyncWriteBytesExt::write_f32::<BigEndian>;
    f64: AsyncReadBytesExt::read_f64::<BigEndian>, AsyncWriteBytesExt::write_f64::<BigEndian>;
);

impl AsyncDecode for i32 {
    type Options = IntOptions;
    type Error<E> = ReadMinecraftError<E>;

    #[inline]
    async fn decode<R: AsyncRead>(
        reader: &mut R,
        IntOptions { varint }: IntOptions,
    ) -> Result<Self, Self::Error<R::Error>> {
        if varint {
            return reader.read_var_i32().await;
        } else {
            return Ok(reader.read_i32::<BigEndian>().await?);
        }
    }
}

impl AsyncEncode for i32 {
    type Options = IntOptions;
    type Error<E> = E;

    #[inline]
    async fn encode<W: AsyncWrite>(
        &self,
        writer: &mut W,
        IntOptions { varint }: IntOptions,
    ) -> Result<(), Self::Error<W::Error>> {
        if varint {
            writer.write_var_i32(*self).await
        } else {
            writer.write_i32::<BigEndian>(*self).await
        }
    }
}
