// Hide the console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    // Spotify only accepts loopback redirect URIs like http://127.0.0.1:PORT/,
    // so the app serves its own page from a tiny local server instead of tauri://
    server::start();

    tauri::Builder::default()
        .setup(|app| {
            let url = format!("http://127.0.0.1:{}/", server::PORT).parse().unwrap();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Refract")
                .inner_size(400.0, 780.0)
                .min_inner_size(340.0, 620.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Refract");
}
