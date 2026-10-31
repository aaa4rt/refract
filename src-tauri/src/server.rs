//! Minimal HTTP server bound to 127.0.0.1.
//! - "/"            → the bundled player page (index.html)
//! - "/lrclib/..."  → fetches lyrics from lrclib.net on the page's behalf
//!                    (the web view isn't allowed to call lrclib.net directly)

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

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
    let path = req.split_whitespace().nth(1).unwrap_or("/").to_string();
    let route = path.split('?').next().unwrap_or("/");

    let (status, ctype, body, proxied): (&str, &str, Vec<u8>, bool) =
        if route == "/" || route == "/index.html" {
            ("200 OK", "text/html; charset=utf-8", PAGE.as_bytes().to_vec(), false)
        } else if route == "/lrclib/api/get" || route == "/lrclib/api/search" {
            let (s, b) = lyrics(&path["/lrclib".len()..]);
            (s, "application/json", b, true)
        } else {
            ("404 Not Found", "text/plain", b"not found".to_vec(), false)
        };

    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\n{}Connection: close\r\n\r\n",
        body.len(),
        if proxied { "X-Refract-Proxy: 1\r\n" } else { "" }
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&body);
}

/// Forward a lyrics request (only /api/get and /api/search) to lrclib.net.
fn lyrics(api_path: &str) -> (&'static str, Vec<u8>) {
    let url = format!("https://lrclib.net{api_path}");
    let res = ureq::get(&url)
        .set("User-Agent", "Refract (https://github.com/aaa4rt/refract)")
        .timeout(Duration::from_secs(10))
        .call();
    match res {
        Ok(r) => ("200 OK", r.into_string().unwrap_or_default().into_bytes()),
        Err(ureq::Error::Status(404, _)) => ("404 Not Found", b"{}".to_vec()),
        Err(_) => ("502 Bad Gateway", b"{}".to_vec()),
    }
}
