# tokio-either
[![Docs.rs](https://docs.rs/tokio-either/badge.svg)](https://crates.io/crates/tokio-either)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.4-orange.svg)](https://crates.io/crates/tokio-either)

Either for Tokio

```rust
type MaybeTls = Either<TcpStream, TlsStream<TcpStream>>;
```
