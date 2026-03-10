use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, interaction::InteractState};

pub fn handle_interaction(key: KeyEvent, app: &mut App) {
    match app.input.interact_state {
        InteractState::Normal => match key.code {
            //EXIT
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.logger.log_info("Exit triggered via Ctrl+C");
                app.exit = true;
            }
            code if app.input.keybinds_normal.exit.contains(&code) => {
                app.logger.log_info("Exit triggered via quit keybind");
                app.exit = true;
            }

            //NAVIGATE UP
            code if app.input.keybinds_normal.navigate_up.contains(&code) => {
                if key.is_press() {
                    app.logger.log_info("Navigate up");
                    app.ui.dir_table_state.select_previous();
                }
            }

            //NAVIGATE DOWN
            code if app.input.keybinds_normal.navigate_down.contains(&code) => {
                if key.is_press() {
                    app.logger.log_info("Navigate down");
                    app.ui.dir_table_state.select_next();
                }
            }

            //SELECT
            code if app.input.keybinds_normal.select.contains(&code) => {
                if key.is_press() {
                    // 1. Check for CONTROL using .contains() (Robust)
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.logger
                            .log_info("Ctrl+Select: outputting current directory");
                        output_dir(app);
                    }
                    // 2. Check for NONE (or handle else for default behavior)
                    // strict equality is fine for NONE, or use key.modifiers.is_empty()
                    else if let Some(item_index) = app.ui.dir_table_state.selected() {
                        let item = &app.navigator.dir_items[item_index];
                        if let Ok(item_type) = item.file_type() {
                            match item_type {
                                _ if item_type.is_dir() => {
                                    let path = item.path();
                                    app.logger.log_info(
                                        format!("Selected directory: {}", path.display()).as_str(),
                                    );
                                    app.traverse(path, true);
                                }
                                _ if item_type.is_file() => {
                                    app.logger.log_info(
                                        format!("Selected file: {}", item.path().display())
                                            .as_str(),
                                    );
                                    todo!("Handle interaction with files")
                                }
                                _ => {}
                            }
                        }
                    };
                }
            }

            //BACK DIR
            code if app.input.keybinds_normal.back_dir.contains(&code) => {
                if key.is_press() {
                    if let Some(dir) = app.navigator.root_dir.parent() {
                        app.logger.log_info(
                            format!("Back dir: navigating to parent {}", dir.display()).as_str(),
                        );
                        app.traverse(dir.to_path_buf(), true);
                    }
                }
            }

            //BACK HISTORY
            code if app.input.keybinds_normal.back_history.contains(&code) => {
                if key.is_press() {
                    if let Some(previous_entry) = app.navigator.history.pop() {
                        app.logger.log_info(
                            format!("Back history: returning to {}", previous_entry.display())
                                .as_str(),
                        );
                        app.traverse(previous_entry, false);
                    } else {
                        app.logger
                            .log_info("Back history: no history left, exiting");
                        app.exit = true;
                    }
                }
            }

            // TOGGLE LOG SCREEN
            code if app.input.keybinds_normal.toggle_log.contains(&code) => {
                if key.is_press() {
                    app.logger.log_info("Toggle log screen");
                    app.toggle_logs();
                }
            }

            _ => {}
        },

        InteractState::Log => match key.code {
            // TOGGLE SCROLL
            code if app.input.keybinds_log.toggle_scroll.contains(&code) => {
                if key.is_press() {
                    app.logger.log_error("Toggle scroll - TODO");
                }
            }

            // TOGGLE LOG SCREEN
            code if app.input.keybinds_log.toggle_log.contains(&code) => {
                if key.is_press() {
                    app.logger.log_info("Toggle log screen");
                    app.toggle_logs();
                }
            }

            _ => {}
        },
    }
}

fn output_dir(app: &mut App) {
    let output = app.navigator.root_dir.to_str().unwrap_or("").to_string();
    app.logger
        .log_info(format!("Outputting directory: {}", output).as_str());
    app.output = output;
    app.exit = true;
}

//TODO: #[cfg(target_os = "windows")] for explorer opening on windows
