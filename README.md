rust-lime
----------------

### Rust impl of the LIME Protocol
The goal of this project ultimately is to have a succesful implementation of the [LIME Protocol](http://limeprotocol.org/),

This library is implemented using the [tokio](https://tokio.rs/) library, which is the up-and-coming library for all
asynchronous I/O needs. The purpose of this is to provide an example of how someone could use this library as well as for
myself, to gain experience with Rust as a language, as well as networking applications in general.

### Goals
- one-on-one messaging
- group messaging
- user registry (using [mysql_async](https://github.com/blackbeam/mysql_async)

### Design

TcpConnection -> Handshake -> Authentication -> Session -> EndOfSession

