
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Create UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Server address
    let server = "127.0.0.1:8080";

    // Send message
    socket.send_to(b"Hello UDP Server!", server)?;

    println!("Message sent!");

    Ok(())
}
