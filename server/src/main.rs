use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;

    println!("UDP server running on port 8080");

    let mut buffer = [0u8; 1024];

    loop {
        let (size, client) = socket.recv_from(&mut buffer)?;

        let message = String::from_utf8_lossy(&buffer[..size]);

        println!("Received from {}: {}", client, message);

        socket.send_to(
            b"Hello from Rust UDP server!",
            client
        )?;
    }
}