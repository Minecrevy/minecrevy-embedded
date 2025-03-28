#![no_std]

pub use byteorder::*;
pub use embedded_io::*;
pub use embedded_io_async::{
    BufRead as AsyncBufRead, Read as AsyncRead, Seek as AsyncSeek, Write as AsyncWrite,
};

mod io_async;
mod io_blocking;

pub use self::io_async::*;
pub use self::io_blocking::*;

pub struct Limit<T> {
    inner: T,
    limit: usize,
}

impl<T> Limit<T> {
    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: ErrorType> ErrorType for Limit<T> {
    type Error = T::Error;
}

impl<T: Read> Read for Limit<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = core::cmp::min(buf.len(), self.limit);
        let n = self.inner.read(&mut buf[..max])?;
        assert!(n <= self.limit, "number of read bytes exceeds limit");
        self.limit -= n;
        Ok(n)
    }
}

impl<T: AsyncRead> AsyncRead for Limit<T> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = core::cmp::min(buf.len(), self.limit);
        let n = self.inner.read(&mut buf[..max]).await?;
        assert!(n <= self.limit, "number of read bytes exceeds limit");
        self.limit -= n;
        Ok(n)
    }
}

impl<T: Write> Write for Limit<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = core::cmp::min(buf.len(), self.limit);
        let n = self.inner.write(&buf[..max])?;
        assert!(n <= self.limit, "number of written bytes exceeds limit");
        self.limit -= n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}

impl<T: AsyncWrite> AsyncWrite for Limit<T> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if self.limit == 0 {
            return Ok(0);
        }

        let max = core::cmp::min(buf.len(), self.limit);
        let n = self.inner.write(&buf[..max]).await?;
        assert!(n <= self.limit, "number of written bytes exceeds limit");
        self.limit -= n;
        Ok(n)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush().await
    }
}
