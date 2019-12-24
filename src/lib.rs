use std::{
    fmt,
    fmt::{Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> AsyncRead for Either<L, R>
where
    L: AsyncRead + Unpin,
    R: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<tokio::io::Result<usize>> {
        match self.get_mut() {
            Self::Left(l) => Pin::new(l).poll_read(cx, buf),
            Self::Right(r) => Pin::new(r).poll_read(cx, buf),
        }
    }
}

impl<L, R> AsyncWrite for Either<L, R>
where
    L: AsyncWrite + Unpin,
    R: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<tokio::io::Result<usize>> {
        match self.get_mut() {
            Self::Left(l) => Pin::new(l).poll_write(cx, buf),
            Self::Right(r) => Pin::new(r).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<tokio::io::Result<()>> {
        match self.get_mut() {
            Self::Left(l) => Pin::new(l).poll_flush(cx),
            Self::Right(r) => Pin::new(r).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<tokio::io::Result<()>> {
        match self.get_mut() {
            Self::Left(l) => Pin::new(l).poll_shutdown(cx),
            Self::Right(r) => Pin::new(r).poll_shutdown(cx),
        }
    }
}

impl<L, R> Debug for Either<L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left(l) => l.fmt(f),
            Self::Right(r) => r.fmt(f),
        }
    }
}
