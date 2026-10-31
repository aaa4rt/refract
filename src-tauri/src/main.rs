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
            let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Refract")
                .inner_size(960.0, 620.0)
                .min_inner_size(420.0, 560.0);

            // macOS: keep the real traffic lights, but blend the title bar into the app
            #[cfg(target_os = "macos")]
            let builder = builder
                .title_bar_style(tauri::TitleBarStyle::Overlay)
                .hidden_title(true);

            // Windows / Linux: remove the system title bar; the page draws its own buttons
            #[cfg(not(target_os = "macos"))]
            let builder = builder.decorations(false).shadow(true);

            builder.build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Refract");
}
