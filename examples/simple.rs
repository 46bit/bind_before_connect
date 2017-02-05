extern crate bind_before_connect;

use bind_before_connect::bind_before_connect;
use std::io::prelude::*;

fn main() {
    // Bind to a randomly selected local port. This is equivalent to creating a
    // std::net::TcpStream normally.
    // let source_addr = "0.0.0.0:0";

    // Bind to a specified local port 4646.
    let source_addr = "0.0.0.0:4646";

    // Bind to a specified local IP and port.
    // let source_addr = "10.240.34.166:0";

    // This is just the same as the normal std::net::TcpStream destination address.
    let dest_addr = "46b.it:22";

    let mut stream = bind_before_connect(source_addr, dest_addr).unwrap();

    // Read 10 bytes from the 46b.it SSH server.
    let mut buffer = [0; 10];
    println!("{}", stream.read(&mut buffer).unwrap());
    println!("{:?}", String::from_utf8_lossy(&buffer));
}
