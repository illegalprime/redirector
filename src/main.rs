use std::env;
use std::io::Write;
use std::net::TcpListener;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_REDIRECT: &str = "https://hack.gt";
const LISTENING_ADDRESS: &str = "0.0.0.0";

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    let redirect = env::var("REDIRECT_URL").unwrap_or_else(|_| DEFAULT_REDIRECT.to_string());

    let header = format!("HTTP/1.1 308 Permanent Redirect\r\nLocation: {}\r\n\r\n", redirect);
    let header = header.as_bytes();

    let listener = TcpListener::bind((LISTENING_ADDRESS, port)).expect("Failed to bind to address! Are you trying to listen to ports <= 1024 without root access?");

    println!("Redirector redirecting to {} listening on {}:{}", redirect, LISTENING_ADDRESS, port);

    for mut stream in listener.incoming().filter_map(Result::ok) {
        stream.write(header).ok();
        stream.flush().ok();
    }
}
