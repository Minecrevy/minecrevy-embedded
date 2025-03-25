#![expect(async_fn_in_trait)]

use byteorder::ByteOrder;
use embedded_io_async::{Read, ReadExactError, Write};

use crate::Take;

/// Extends [`Read`] with methods for reading numbers.
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
pub trait AsyncReadBytesExt: Read {
    /// Creates a new [`Take`] instance that reads at most `limit` bytes from the
    /// underlying reader.
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take { inner: self, limit }
    }

    /// Asynchronously reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u8(&mut self) -> Result<u8, ReadExactError<Self::Error>> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    /// Asynchronously reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i8(&mut self) -> Result<i8, ReadExactError<Self::Error>> {
        Ok(self.read_u8().await? as i8)
    }

    /// Asynchronously reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u16<T: ByteOrder>(&mut self) -> Result<u16, ReadExactError<Self::Error>> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u16(&buf))
    }

    /// Asynchronously reads a signed 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i16<T: ByteOrder>(&mut self) -> Result<i16, ReadExactError<Self::Error>> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i16(&buf))
    }

    /// Asynchronously reads an unsigned 24 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u24<T: ByteOrder>(&mut self) -> Result<u32, ReadExactError<Self::Error>> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u24(&buf))
    }

    /// Asynchronously reads a signed 24 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i24<T: ByteOrder>(&mut self) -> Result<i32, ReadExactError<Self::Error>> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i24(&buf))
    }

    /// Asynchronously reads an unsigned 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u32<T: ByteOrder>(&mut self) -> Result<u32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u32(&buf))
    }

    /// Asynchronously reads a signed 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i32<T: ByteOrder>(&mut self) -> Result<i32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i32(&buf))
    }

    /// Asynchronously reads an unsigned 48 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u48<T: ByteOrder>(&mut self) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 6];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u48(&buf))
    }

    /// Asynchronously reads a signed 48 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i48<T: ByteOrder>(&mut self) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 6];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i48(&buf))
    }

    /// Asynchronously reads an unsigned 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u64<T: ByteOrder>(&mut self) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u64(&buf))
    }

    /// Asynchronously reads a signed 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i64<T: ByteOrder>(&mut self) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i64(&buf))
    }

    /// Asynchronously reads an unsigned 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_u128<T: ByteOrder>(&mut self) -> Result<u128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(T::read_u128(&buf))
    }

    /// Asynchronously reads a signed 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_i128<T: ByteOrder>(&mut self) -> Result<i128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(T::read_i128(&buf))
    }

    /// Asynchronously reads an unsigned n-bytes integer as a `u64` from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_uint<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<u64, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_uint(&buf[..nbytes], nbytes))
    }

    /// Asynchronously reads a signed n-bytes integer as an `i64` from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_int<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<i64, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_int(&buf[..nbytes], nbytes))
    }

    /// Asynchronously reads an unsigned n-bytes integer as a `u128` from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_uint128<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<u128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_uint128(&buf[..nbytes], nbytes))
    }

    /// Asynchronously reads a signed n-bytes integer as an `i128` from the
    /// underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_int128<T: ByteOrder>(
        &mut self,
        nbytes: usize,
    ) -> Result<i128, ReadExactError<Self::Error>> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf[..nbytes]).await?;
        Ok(T::read_int128(&buf[..nbytes], nbytes))
    }

    /// Asynchronously reads a IEEE754 single-precision (4 bytes) floating point
    /// number from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_f32<T: ByteOrder>(&mut self) -> Result<f32, ReadExactError<Self::Error>> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(T::read_f32(&buf))
    }

    /// Asynchronously reads a IEEE754 double-precision (8 bytes) floating point
    /// number from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    async fn read_f64<T: ByteOrder>(&mut self) -> Result<f64, ReadExactError<Self::Error>> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(T::read_f64(&buf))
    }
}

impl<R: Read + ?Sized> AsyncReadBytesExt for R {}

/// Extends [`Write`] with methods for writing numbers.
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
pub trait AsyncWriteBytesExt: Write {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u8(&mut self, n: u8) -> Result<(), Self::Error> {
        self.write_all(&[n]).await
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i8(&mut self, n: i8) -> Result<(), Self::Error> {
        self.write_all(&[n as u8]).await
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u16<T: ByteOrder>(&mut self, n: u16) -> Result<(), Self::Error> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i16<T: ByteOrder>(&mut self, n: i16) -> Result<(), Self::Error> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 24 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u24<T: ByteOrder>(&mut self, n: u32) -> Result<(), Self::Error> {
        let mut buf = [0; 3];
        T::write_u24(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 24 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i24<T: ByteOrder>(&mut self, n: i32) -> Result<(), Self::Error> {
        let mut buf = [0; 3];
        T::write_i24(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u32<T: ByteOrder>(&mut self, n: u32) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i32<T: ByteOrder>(&mut self, n: i32) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 48 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u48<T: ByteOrder>(&mut self, n: u64) -> Result<(), Self::Error> {
        let mut buf = [0; 6];
        T::write_u48(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 48 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i48<T: ByteOrder>(&mut self, n: i64) -> Result<(), Self::Error> {
        let mut buf = [0; 6];
        T::write_i48(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u64<T: ByteOrder>(&mut self, n: u64) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i64<T: ByteOrder>(&mut self, n: i64) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_u128<T: ByteOrder>(&mut self, n: u128) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_u128(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_i128<T: ByteOrder>(&mut self, n: i128) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_i128(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes an unsigned n-bytes integer as a `u64` to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_uint<T: ByteOrder>(&mut self, n: u64, nbytes: usize) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_uint(&mut buf[..nbytes], n, nbytes);
        self.write_all(&buf[..nbytes]).await
    }

    /// Writes a signed n-bytes integer as an `i64` to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_int<T: ByteOrder>(&mut self, n: i64, nbytes: usize) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_int(&mut buf[..nbytes], n, nbytes);
        self.write_all(&buf[..nbytes]).await
    }

    /// Writes an unsigned n-bytes integer as a `u128` to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_uint128<T: ByteOrder>(
        &mut self,
        n: u128,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_uint128(&mut buf[..nbytes], n, nbytes);
        self.write_all(&buf[..nbytes]).await
    }

    /// Writes a signed n-bytes integer as an `i128` to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_int128<T: ByteOrder>(
        &mut self,
        n: i128,
        nbytes: usize,
    ) -> Result<(), Self::Error> {
        let mut buf = [0; 16];
        T::write_int128(&mut buf[..nbytes], n, nbytes);
        self.write_all(&buf[..nbytes]).await
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number to the
    /// underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_f32<T: ByteOrder>(&mut self, n: f32) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all(&buf).await
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number to the
    /// underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    async fn write_f64<T: ByteOrder>(&mut self, n: f64) -> Result<(), Self::Error> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf).await
    }
}

impl<W: Write + ?Sized> AsyncWriteBytesExt for W {}
