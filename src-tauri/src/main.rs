#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use bollard::container::ListContainersOptions;
use bollard::Docker;
use std::default::Default;

mod container;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let _app = app.handle();
      let docker_lin = Docker::connect_with_socket_defaults().unwrap(); // TODO Connect with Docker Linux daemon using TCP
      // let docker_win = Docker::connect_with_socket_defaults().unwrap; //  TODO Connect with Docker Windows daemon
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
      container::create_container
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
