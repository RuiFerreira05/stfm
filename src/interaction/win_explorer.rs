#[cfg(target_os = "windows")]
use crate::app::App;
#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(target_os = "windows")]
impl App {
    pub fn open_file(&mut self, path: &str) {
        if let Err(e) = Command::new("explorer.exe").arg(path).spawn() {
            self.ui.display_error("Failed to open explorer");
            self.logger
                .log_error(format!("Failed to open explorer: {}", e.to_string()).as_str());
        }
    }
}
