use std::env;
use std::io::Write;
use std::net::TcpListener;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_REDIRECT: &'static str = "https://hack.gt";
const LISTENING_ADDRESS: &'static str = "0.0.0.0";

fn main() {
    let port = env::var("PORT")
        .map(|p| p.parse().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);

    let redirect = env::var("REDIRECT_URL");
    let redirect = redirect
        .as_ref().map(|s| &**s)
        .unwrap_or(DEFAULT_REDIRECT);

    let header = format!(
        "HTTP/1.1 307 Temporary Redirect\r\n\
         Location: {}\r\n\
         \r\n", redirect);
    let header = header.as_bytes();

    let listener = TcpListener::bind((LISTENING_ADDRESS, port))
        .expect("Failed to bind to address! Are you trying to listen to ports <= 1024 without root access? Or is a process already using that port?");

    println!("Redirecting to {} listening on {}:{}", redirect, LISTENING_ADDRESS, port);

    for mut stream in listener.incoming().filter_map(Result::ok) {
        stream.write(header).ok();
        stream.flush().ok();
    }
}
