use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use rustls::ClientConnection;

mod cert_generator;
mod connection_params;

fn main() {
    let mut client =
        ClientConnection::new(connection_params::config(), connection_params::name()).unwrap();

    let mut socket = TcpStream::connect("192.168.1.100:30025").unwrap();

    let stream = rustls::Stream::new(&mut client, &mut socket);
    let mut bufreader = BufReader::new(stream);

    println!("Listening...");

    loop {
        let mut buf = String::new();
        bufreader.read_line(&mut buf).unwrap();
        if buf.is_empty() {
            break;
        }
        println!("Received: {}", buf);
    }
}
