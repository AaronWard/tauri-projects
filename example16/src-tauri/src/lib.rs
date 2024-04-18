use tauri::{ LogicalPosition, LogicalSize, WebviewUrl };
// use std::process::Command;
use std::path::PathBuf;
use std::env;
use url::Url;
use tauri_plugin_localhost;
// use std::net::TcpListener;
use std::thread;
use tiny_http::{ Server, Response };
use std::fs::File;

const PORT: u16 = 9527;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// fn run_app_sidecar(name: String) -> Result<String, String> {
//     // Use String as the error type for simplicity
//     let output = Command::new(
//         "/Users/award40/Desktop/personal/github/tauri-projects/example15/src/example13.app"
//     ) // Adjust the path as necessary
//         .arg(name)
//         .output()
//         .expect("Failed to execute command");

//     if output.status.success() {
//         Ok(String::from_utf8_lossy(&output.stdout).into_owned()) // Convert output to String directly
//     } else {
//         Err(String::from_utf8_lossy(&output.stderr).into_owned()) // Return errors as Err
//     }
// }

// fn run_app() -> Result<Url, Box<dyn std::error::Error>> {
//     // let app_path = PathBuf::from(env::current_exe()?.parent().unwrap().parent().unwrap().parent().unwrap().join("/src/example13.app"));
//     let url = format!("http://localhost:{}", PORT);
//     let output = Command::new("/Users/award40/Desktop/personal/github/tauri-projects/example15/src/example13.app")
//         .arg(name)
//         .expect("Failed to execute command");

//     if output.status.success() {
//         Ok(url)
//     } else {
//         Err(From::from(format!("Failed to find example13.app at {:?}", app_path)))
//     }

//     // if app_path.exists() {
//     //     Command::new("open").arg(&app_path).spawn()?;
//     //     std::thread::sleep(std::time::Duration::from_secs(5));
//     //     Ok(Url::parse(&url)?)
//     // } else {
//     //     Err(From::from(format!("Failed to find example13.app at {:?}", app_path)))
//     // }
// }

// fn run_app() -> Result<Url, Box<dyn std::error::Error>> {
//     // let app_path = PathBuf::from("/Users/award40/Desktop/personal/github/tauri-projects/example16/src/example13.app");
//     let web_app_path = PathBuf::from(env::current_dir()?).join("/Users/award40/Desktop/personal/github/tauri-projects/example16/src/example13.app");
//     let server = Server::http(format!("0.0.0.0:{}", PORT)).unwrap();

//     thread::spawn(move || {
//         for request in server.incoming_requests() {
//             let file_path = web_app_path.join(request.url().trim_start_matches('/'));
//             let response = Response::from_file(File::open(file_path).unwrap());
//             request.respond(response).unwrap();
//         }
//     });

//     let url = format!("http://localhost:{}", PORT).parse()?;
//     Ok(url)
// }

fn launch_external_app() -> Result<u32, Box<dyn std::error::Error>> {
    let child = Command::new("/Users/award40/Desktop/personal/github/tauri-projects/example16/src/example13.app")
        .stdout(Stdio::piped())
        .spawn()?;
    
    Ok(child.id())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let devtools = devtools::init(); // initialize the plugin as early as possible
    let external_app_pid = launch_external_app().unwrap(); 

    tauri::Builder
        ::default()
        .plugin(devtools)
        .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_localhost::Builder::new(PORT).build())
        .setup(move |app| {
            match run_app_sidecar() {
                Ok(url) => {
                    let width = 1200.0;
                    let height = 1000.0;
                    let window = tauri::window::WindowBuilder
                        ::new(app, "main")
                        .inner_size(width, height)
                        .build()?;

                    let _webview = window.add_child(
                        tauri::webview::WebviewBuilder
                            ::new("example13", WebviewUrl::External(url))
                            .auto_resize(),
                        LogicalPosition::new(0.0, 0.0),
                        LogicalSize::new(width / 2.0, height / 2.0)
                    )?;
                }
                Err(e) => {
                    eprintln!("Failed to start external application: {:?}", e);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
