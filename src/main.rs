use std::env;
use std::io::Write;
use std::net::{SocketAddr, TcpListener};

fn main() {
    const DEFAULT_PORT: u16 = 3000;
    const DEFAULT_REDIRECT: &str = "https://hack.gt";

    let port: u16 = match env::var("PORT") {
        Ok(value) => value.parse().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT
    };
    let redirect = match env::var("REDIRECT_URL") {
        Ok(value) => value,
        Err(_) => DEFAULT_REDIRECT.to_string()
    };

    let mut socket_address: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    socket_address.set_port(port);
    println!("Redirector redirecting to {} listening on {}", redirect, socket_address);

    let listener = TcpListener::bind(socket_address).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let _ = stream.write(
            format!("HTTP/1.1 308 Permanent Redirect\r\nLocation: {}\r\n\r\n", redirect).as_bytes()
        );
        let _ = stream.flush();
    }
}
