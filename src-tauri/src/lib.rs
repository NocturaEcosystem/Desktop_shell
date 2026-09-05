mod utils;
use utils::AppInfo;
use std::collections::HashMap;
use std::fs;
use tauri::utils::config::WebviewUrl;
#[tauri::command]
fn get_apps() -> HashMap<String, Vec<AppInfo>> {
    utils::get_apps()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_asynchronous_uri_scheme_protocol("app-icon", |_app, request, responder| {
            let path = request.uri().path();
            let decoded_path = urlencoding::decode(path)
                .map(|p| p.into_owned())
                .unwrap_or_else(|_| path.to_string());
            match fs::read(&decoded_path) {
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
                            .unwrap()
                    );
                }
                Err(_) => {
                    responder.respond(
                        tauri::http::Response::builder()
                            .status(404)
                            .body(Vec::new())
                            .unwrap()
                    );
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

