
use std::{net::UdpSocket, thread, time::Duration};
use vigem_rust::{Client, X360Button, X360Report};

fn server() {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    println!("UDP server running on port 8080");

    let mut buffer = [0u8; 1024];

    loop {
        let (size, addr) = socket.recv_from(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
        socket.send_to(b"Hello!", addr).unwrap();
    }
}

fn main() {
    let client = Client::connect().unwrap();
    let pad = client.new_x360_target().plug().unwrap().wait_for_ready().unwrap();

    println!("Controller connected!");

    thread::spawn(server);

    let mut report = X360Report::default();
    report.buttons.insert(X360Button::A);
    pad.update(&report).unwrap();

    println!("A pressed!");
    thread::sleep(Duration::from_secs(5));

    pad.update(&X360Report::default()).unwrap();
    println!("A released!");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
