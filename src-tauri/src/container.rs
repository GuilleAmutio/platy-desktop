use bollard::{container::{StartContainerOptions, StopContainerOptions, RestartContainerOptions, RemoveContainerOptions}, Docker};
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