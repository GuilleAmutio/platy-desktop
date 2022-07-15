#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::{debug, error, log_enabled, info, Level};
use bollard::container::ListContainersOptions;
use bollard::Docker;
use tauri::{Manager};
use std::default::Default;

mod container;

#[tauri::command(async)]
async fn app_ready(app_handle: tauri::AppHandle) {
	let window = app_handle.get_window("main").unwrap();

	window.show().unwrap();
}


#[tokio::main]
async fn main() {
  env_logger::init();

  // TODO Connect with Docker Linux daemon using TCP
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();
  // let docker_win = Docker::connect_with_socket_defaults().unwrap; //  TODO Connect with Docker Windows daemon

  tauri::Builder::default()
    .setup(|app| {
      let _app = app.handle();
      
      /*
        4 different threads:
          - Events for Linux containers
          - Events for Windows containers
          - Events for Linux images
          - Events for Windows images
      */
      tokio::spawn(async move {        
        let containers = &docker_lin.list_containers(Some(ListContainersOptions::<String> {
          all: true,
          ..Default::default()
        })).await.unwrap();

        for container in containers {
          println!("-> {:?}", container);
        }
        
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      container::start_container,
      container::stop_container,
      container::restart_container,
      container::remove_container,
      container::open_in_browser,
      container::open_terminal,
      app_ready
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
