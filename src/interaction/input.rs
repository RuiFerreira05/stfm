use crate::app::App;

use super::action::Action;

impl App {
    /// Execute an action that was resolved from the keymap.
    /// This is pure application logic — no keybind awareness needed.
    pub fn execute(&mut self, action: &Action) {
        match action {
            Action::Exit => {
                self.logger.log_info("Exit triggered");
                self.exit = true;
            }

            Action::NavigateUp => {
                self.logger.log_info("Navigate up");
                self.ui.dir_table_state.select_previous();
            }

            Action::NavigateDown => {
                self.logger.log_info("Navigate down");
                self.ui.dir_table_state.select_next();
            }

            Action::Select => {
                if let Some(item_index) = self.ui.dir_table_state.selected() {
                    let item = &self.navigator.dir_items[item_index];
                    if let Ok(item_type) = item.file_type() {
                        if item_type.is_dir() {
                            let path = item.path();
                            self.logger.log_info(
                                format!("Selected directory: {}", path.display()).as_str(),
                            );
                            self.traverse(path, true);
                        } else if item_type.is_file() {
                            self.logger.log_info(
                                format!("Selected file: {}", item.path().display()).as_str(),
                            );
                            todo!("Handle interaction with files");
                        }
                    }
                }
            }

            Action::CtrlSelect => {
                self.logger
                    .log_info("Ctrl+Select: outputting current directory");
                let output = self.navigator.root_dir.to_str().unwrap_or("").to_string();
                self.logger
                    .log_info(format!("Outputting directory: {}", output).as_str());
                self.output = output;
                self.exit = true;
            }

            Action::BackDir => {
                if let Some(dir) = self.navigator.root_dir.parent() {
                    self.logger.log_info(
                        format!("Back dir: navigating to parent {}", dir.display()).as_str(),
                    );
                    self.traverse(dir.to_path_buf(), true);
                }
            }

            Action::BackHistory => {
                if let Some(previous_entry) = self.navigator.history.pop() {
                    self.logger.log_info(
                        format!("Back history: returning to {}", previous_entry.display()).as_str(),
                    );
                    self.traverse(previous_entry, false);
                } else {
                    self.logger
                        .log_info("Back history: no history left, exiting");
                    self.exit = true;
                }
            }

            Action::ToggleLogs => {
                self.logger.log_info("Toggle log screen");
                self.toggle_logs();
            }

            Action::ToggleScroll => {
                self.logger.log_error("Toggle scroll - TODO");
            }
        }
    }
}
