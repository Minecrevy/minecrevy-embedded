use embedded_byteorder::{Read, Write};

pub trait Decode: Sized {
    type Options: Clone + Default;
    type Error<E>;

    fn decode<R: Read>(
        reader: &mut R,
        options: Self::Options,
    ) -> Result<Self, Self::Error<R::Error>>;
}

pub trait Encode {
    type Options: Clone + Default;
    type Error<E>;

    fn encode<W: Write>(
        &self,
        writer: &mut W,
        options: Self::Options,
    ) -> Result<(), Self::Error<W::Error>>;
}
