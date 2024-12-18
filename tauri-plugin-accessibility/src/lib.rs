use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Accessibility;
#[cfg(mobile)]
use mobile::Accessibility;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the accessibility APIs.
pub trait AccessibilityExt<R: Runtime> {
  fn accessibility(&self) -> &Accessibility<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AccessibilityExt<R> for T {
  fn accessibility(&self) -> &Accessibility<R> {
    self.state::<Accessibility<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("accessibility")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let accessibility = mobile::init(app, api)?;
      #[cfg(desktop)]
      let accessibility = desktop::init(app, api)?;
      app.manage(accessibility);
      Ok(())
    })
    .build()
}
