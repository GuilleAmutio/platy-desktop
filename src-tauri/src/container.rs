use bollard::{container::{StartContainerOptions, StopContainerOptions, RestartContainerOptions, RemoveContainerOptions, InspectContainerOptions}, Docker, exec::{CreateExecOptions, StartExecOptions}};
use futures::TryFutureExt;

use std::default::Default;
use std::fmt::Debug;

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

#[tauri::command(async)]
pub async fn start_container() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let response = docker_lin.start_container("nginx", None::<StartContainerOptions<String>>);

  print_type_of(&response);
  println!("-> {:?}", response.await.unwrap());
}

#[tauri::command(async)]
pub async fn stop_container() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let response = docker_lin.stop_container("nginx", Some(StopContainerOptions{ t: 30, }));

  print_type_of(&response);
  println!("-> {:?}", response.await.unwrap());
}

#[tauri::command(async)]
pub async fn restart_container() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let response = docker_lin.restart_container("nginx", Some(RestartContainerOptions{ t: 30, }));

  print_type_of(&response);
  println!("-> {:?}", response.await.unwrap());
}

#[tauri::command(async)]
pub async fn remove_container() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let response = docker_lin.remove_container("nginx", Some(RemoveContainerOptions{ force: true, ..Default::default() }));

  print_type_of(&response);
  println!("-> {:?}", response.await.unwrap());
}

#[tauri::command(async)]
pub async fn open_in_browser() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let response = docker_lin.inspect_container("nginx", Some(InspectContainerOptions{ size: false }));

  print_type_of(&response);
  let putamierda1 = response.await.unwrap();
  let putamierda2 = putamierda1.host_config.unwrap();
  let putamierda3 = putamierda2.port_bindings.as_ref().unwrap();
  let putamierda4 = putamierda3.get("80/tcp").unwrap();
  let putamierda5 = putamierda4;
  
  print_type_of(&putamierda5);
  println!("-> {:?}", putamierda4);
}

#[tauri::command(async)]
pub async fn open_terminal() {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let config = CreateExecOptions {
    cmd: Some(vec!["/bin/bash"]),
    attach_stdout: Some(true),
    tty: Some(true),
    ..Default::default()
  };

  let message = docker_lin.create_exec("nginx", config).await.unwrap();
  let response = docker_lin.start_exec(&message.id, None::<StartExecOptions>);
  print_type_of(&response);
  println!("-> {:?}", response.await.unwrap());
}

