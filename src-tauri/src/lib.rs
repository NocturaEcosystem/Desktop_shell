mod utils;
use utils::AppInfo;
use std::collections::HashMap;
use tauri::utils::config::WebviewUrl;
use tauri::WebviewWindowBuilder;

#[tauri::command]
fn get_apps() -> HashMap<String, Vec<AppInfo>> {
    utils::get_apps()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let frontend = std::env::var("LEPUS_FRONTEND").unwrap_or_else(|_| "OSD".to_string());

            let label = match frontend.as_str() {
                "Widgets" => "widgets",
                "MiddleGround" => "middleground",
                "Background" => "background",
                _ => "osd",
            };

            // Read the dynamic port directly from the environment set by dev.mjs
            #[cfg(debug_assertions)]
            let url = {
                let port = std::env::var("VITE_PORT").unwrap_or_else(|_| "1421".to_string());
                WebviewUrl::External(format!("http://localhost:{}", port).parse().unwrap())
            };

            #[cfg(not(debug_assertions))]
            let url = WebviewUrl::App(format!("../frontends/{}/index.html", frontend).into());

            let window = WebviewWindowBuilder::new(app, label, url)
                .title("Lepus")
                .transparent(true)
                .decorations(false)
                .resizable(false)
                .maximized(true)
                .inner_size(1920.0, 1080.0)
                .build()?;
                
            window.open_devtools();
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("app-icon", |_app, request, responder| {
            let path = request.uri().path();
            let decoded_path = urlencoding::decode(path)
                .map(|p| p.into_owned())
                .unwrap_or_else(|_| path.to_string());

            match std::fs::read(&decoded_path) {
                Ok(bytes) => {
                    let mime_type = if decoded_path.ends_with(".svg") {
                        "image/svg+xml"
                    } else {
                        "image/png"
                    };

                    responder.respond(
                        tauri::http::Response::builder()
                            .header("Content-Type", mime_type)
                            .header("Access-Control-Allow-Origin", "*")
                            .body(bytes)
                            .unwrap(),
                    );
                }
                Err(_) => {
                    responder.respond(
                        tauri::http::Response::builder()
                            .status(404)
                            .body(Vec::new())
                            .unwrap(),
                    );
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_apps])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}