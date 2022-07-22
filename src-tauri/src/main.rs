#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::{error, info, LevelFilter, SetLoggerError};
use bollard::container::ListContainersOptions;
use bollard::Docker;
use log4rs::{append::{file::FileAppender, console::{ConsoleAppender}}, encode::pattern::PatternEncoder, init_config, Config, config::{Appender, Root, Logger}};
use tauri::{Manager};
use std::default::Default;
use dotenvy::dotenv;

mod container;

/*
  Wait until the window is ready
*/
#[tauri::command(async)]
async fn app_ready(app_handle: tauri::AppHandle) {
	let window = app_handle.get_window("main").unwrap();

	window.show().unwrap();
}

/*
  Start log4rs
*/
fn init_logging() -> Result<(), SetLoggerError> {
  let logfile_path = "logs/outputs.log";
  let logfile_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

  let stdout = ConsoleAppender::builder().build();
  let logfile = FileAppender::builder()
      .append(false)
      .encoder(Box::new(PatternEncoder::new(logfile_line_pattern)))
      .build(logfile_path)
      .unwrap();

  let config = Config::builder()
      .appender(Appender::builder().build("consoleAppender", Box::new(stdout)))
      .appender(Appender::builder().build("containerAppender", Box::new(logfile)))
      .logger(Logger::builder()
            .appender("containerAppender")
            .additive(true)
            .build("app", LevelFilter::Info))
      .build(Root::builder().appender("consoleAppender").build(LevelFilter::Debug))
      .unwrap();

  let _handle = init_config(config);
  let _result = match _handle
  {
    Ok(_result) => { 
      info!("Logger started") 
    }
    Err(error) => { 
      error!("Logger could not be started: {}", error) 
    }
  };

  Ok(())
}

#[tokio::main]
async fn main() {
  // Load .env file variables
  dotenv().ok();

  // Initialize logger
  let _log_result = init_logging();
  let _content = match _log_result
  {
    Ok(()) => { 
      info!("Logger initialized successfuly") 
    }
    Err(error) => { 
      error!("Logger failed to start: {:?}", error)
    }
  };

  tauri::Builder::default()
    .setup(|app| {
      let _app = app.handle();

      #[cfg(debug_assertions)] // Only include this code on debug builds
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
      }
    
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      container::list_containers,
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
