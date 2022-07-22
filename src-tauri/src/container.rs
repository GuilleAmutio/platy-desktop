use bollard::{container::{StartContainerOptions, StopContainerOptions, RestartContainerOptions, RemoveContainerOptions, InspectContainerOptions, ListContainersOptions}, Docker, exec::{CreateExecOptions, StartExecOptions}};
use log::{warn, error, info, debug};
use webbrowser::open;

use std::{default::Default};

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

/*
  List all containers with their configuration. Frontend is the responsible to parse the information retrieve by this command
*/
#[tauri::command(async)]
pub async fn list_containers() -> Result<(), ()> {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let list_container_options = ListContainersOptions::<String> {
    all:true,
    ..Default::default()
  };

  let result = docker_lin.list_containers(Some(list_container_options)).await;
  let _content = match result {
    Ok(_content) => { 
      info!("{:?}", _content);
    },
    Err(error) => { 
      error!("{}", error); 
    }
  };
  
  Ok(())
}

#[tauri::command(async)]
pub async fn start_container(name: &str) -> Result<(), ()> {
  let docker_lin = Docker::connect_with_socket_defaults().unwrap();

  let result = docker_lin.start_container(name, None::<StartContainerOptions<String>>).await;
  let _content = match result {
    Ok(()) => { 
      info!("Container {} started", name);
    },
    Err(error) => { 
      error!("{}", error); 
    }
  };
  
  Ok(())
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

  let host_ports = response.await.unwrap().host_config.unwrap().port_bindings.unwrap();
  let port_binding = host_ports.get("80/tcp").unwrap();
  let port_browser = &port_binding.iter().flatten().collect::<Vec<_>>()[0].host_port.as_ref().unwrap().to_string();

  let mut url = "http://localhost:".to_string();
  
  url.push_str(port_browser);
  
  let url2 = &url[..];
  if open(url2).is_ok(){
    print!("Opened");
  }
  println!("-> {:?}", port_browser);
  println!("-> {:?}", url);
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

