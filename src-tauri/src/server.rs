//! Minimal HTTP server bound to 127.0.0.1 that serves the single-page player.
//! It only ever answers with the bundled index.html (or 404), so there is nothing to attack.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Fixed so the Spotify Redirect URI never changes: http://127.0.0.1:43821/
pub const PORT: u16 = 43821;
const PAGE: &str = include_str!("../../src/index.html");

pub fn start() {
    let listener = match TcpListener::bind(("127.0.0.1", PORT)) {
        Ok(l) => l,
        Err(e) => {
            // Usually means another copy of the app is already running and serving the page.
            eprintln!("Refract: could not bind 127.0.0.1:{PORT}: {e}");
            return;
        }
    };
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            thread::spawn(move || handle(stream));
        }
    });
}

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req.split_whitespace().nth(1).unwrap_or("/");
    let route = path.split('?').next().unwrap_or("/");

    let (status, body) = if route == "/" || route == "/index.html" {
        ("200 OK", PAGE)
    } else {
        ("404 Not Found", "not found")
    };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}
