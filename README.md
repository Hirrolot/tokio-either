# tokio-either
Either for Tokio

```rust
type MaybeTls = Either<TcpStream, TlsStream<TcpStream>>;
```
