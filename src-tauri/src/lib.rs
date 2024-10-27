// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn tauri_file_server(
    _app: &AppHandle,
    request: tauri::http::Request<Vec<u8>>,
    extension_folder_path: PathBuf,
) -> tauri::http::Response<Vec<u8>> {
    let path = &request.uri().path()[1..]; // skip the first /
    let path = urlencoding::decode(path).unwrap().to_string();
    let mut url_file_path = extension_folder_path;

    url_file_path = url_file_path.join(path);
    println!("url_file_path: {:?}", url_file_path);
    // check if it's file or directory, if file and exist, return file, if directory, return index.html, if neither, check .html
    if url_file_path.is_file() {
        // println!("1st case url_file_path: {:?}", url_file_path);
        let mime_type = match url_file_path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("js") => "application/javascript",
            Some("html") => "text/html",
            Some("css") => "text/css",
            _ => "application/octet-stream",
        };
        return tauri::http::Response::builder()
            .status(tauri::http::StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Type", mime_type)
            .body(std::fs::read(url_file_path).unwrap())
            .unwrap();
    } else if url_file_path.is_dir() {
        /*
         * there are two cases:
         * 1. directory conntains a index.html, then return index.html
         * 2. directory has a sibling file with .html extension, return that file
         */
        let index_html_path = url_file_path.join("index.html");
        if index_html_path.is_file() {
            // println!("2nd case index_html_path: {:?}", index_html_path);
            return tauri::http::Response::builder()
                .status(tauri::http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "no-cache")
                .body(std::fs::read(index_html_path).unwrap())
                .unwrap();
        }
        // check if path has a sibling file with .html extension
        // get folder name
        match url_file_path.file_name() {
            Some(folder_name) => {
                let parent_path = url_file_path.parent().unwrap();
                let html_file_path =
                    parent_path.join(format!("{}.html", folder_name.to_str().unwrap()));
                if html_file_path.is_file() {
                    // println!("3rd case html_file_path: {:?}", html_file_path);
                    return tauri::http::Response::builder()
                        .status(tauri::http::StatusCode::OK)
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Cache-Control", "no-cache")
                        .body(std::fs::read(html_file_path).unwrap())
                        .unwrap();
                }
            }
            None => {}
        }

        // check if url_file_path's parent has a file with name folder_name.html
    } else {
        // file not found, check if end with .html works. if path ends with /, remove the / and check if adding .html makes a file
        let mut path_str = url_file_path.to_str().unwrap().to_string();
        if path_str.ends_with("/") {
            path_str.pop();
        }
        path_str.push_str(".html");
        let path_str = PathBuf::from(path_str);
        if path_str.is_file() {
            // println!("4rd case path_str: {:?}", path_str);
            return tauri::http::Response::builder()
                .status(tauri::http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "no-cache")
                .body(std::fs::read(path_str).unwrap())
                .unwrap();
        }
    }
    // println!("5th case file not found");
    return tauri::http::Response::builder()
        .status(tauri::http::StatusCode::NOT_FOUND)
        .header("Access-Control-Allow-Origin", "*")
        .body("file not found".as_bytes().to_vec())
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .register_uri_scheme_protocol("ext", |app, request| {
            let cwd = std::env::current_dir().unwrap();
            let extension_folder_path = cwd.parent().unwrap().join("iframe");
            println!("extension_folder_path: {:?}", extension_folder_path);
            let app_handle = app.app_handle();
            tauri_file_server(app_handle, request, extension_folder_path)
        })
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
