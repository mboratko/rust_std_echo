# A simple echo server in Rust

An example of an echo server using only `std`, demonstrating `{TcpListener, TcpStream}` and `{BufReader, BufWriter}`. The goal was to build a simple example which would read lines instead of getting into details such as frame size.

Start the server:
`cargo run --bin server`

Then start the client:
`cargo run --bin server`

Type something into the client window.