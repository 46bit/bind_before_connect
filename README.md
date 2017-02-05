# bind_before_connect

In older network protocols or when using multiple IP Addresses, one wants to open a connection to a remote host using a specific local port and/or a specific local IP address. In other words you want to specify the local source address.

This can be done using the `bind` syscall (commonly used to specify the binding address for servers) followed by the `connect` syscall (commonly used to connect to a remote server). This sequence of syscalls is termed [Bind Before Connect](https://idea.popcount.org/2014-04-03-bind-before-connect/).

The constructor of Rust's `std::net::TcpStream` does not allow specifying a particular local address. It picks a random high port to use. This crate constructs a bound socket then converts it to a `std::net::TcpStream` for you to use as normal.

**At present only Unix-based systems are supported. An implementation for Windows/etc would be very much welcomed.**

## Usage examples

See `examples/simple.rs` and run `cargo run --example simple`.
