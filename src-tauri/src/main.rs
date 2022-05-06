use tauri::{Manager};


fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.windows().iter().for_each(|(_, window)| {
        window_shadows::set_shadow(&window, true).unwrap_or(());
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
