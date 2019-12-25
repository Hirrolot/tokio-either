use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
};

#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> AsyncRead for Either<L, R>
where
    L: AsyncRead,
    R: AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<tokio::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::Left(l) => Pin::new_unchecked(l).poll_read(cx, buf),
                Self::Right(r) => Pin::new_unchecked(r).poll_read(cx, buf),
            }
        }
    }
}

impl<L, R> AsyncWrite for Either<L, R>
where
    L: AsyncWrite,
    R: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<tokio::io::Result<usize>> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::Left(l) => Pin::new_unchecked(l).poll_write(cx, buf),
                Self::Right(r) => Pin::new_unchecked(r).poll_write(cx, buf),
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<tokio::io::Result<()>> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::Left(l) => Pin::new_unchecked(l).poll_flush(cx),
                Self::Right(r) => Pin::new_unchecked(r).poll_flush(cx),
            }
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<tokio::io::Result<()>> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::Left(l) => Pin::new_unchecked(l).poll_shutdown(cx),
                Self::Right(r) => Pin::new_unchecked(r).poll_shutdown(cx),
            }
        }
    }
}
