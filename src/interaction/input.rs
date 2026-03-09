use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, interaction::InteractState};

pub fn handle_interaction(key: KeyEvent, app: &mut App) {
    match app.input.interact_state {
        InteractState::Normal => match key.code {
            //EXIT
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.exit = true,
            code if app.input.keybinds_normal.exit.contains(&code) => app.exit = true,

            //NAVIGATE UP
            code if app.input.keybinds_normal.navigate_up.contains(&code) => {
                if key.is_press() {
                    app.ui.dir_table_state.select_previous();
                }
            }

            //NAVIGATE DOWN
            code if app.input.keybinds_normal.navigate_down.contains(&code) => {
                if key.is_press() {
                    app.ui.dir_table_state.select_next();
                }
            }

            //SELECT
            code if app.input.keybinds_normal.select.contains(&code) => {
                if key.is_press() {
                    // 1. Check for CONTROL using .contains() (Robust)
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        output_dir(app);
                    }
                    // 2. Check for NONE (or handle else for default behavior)
                    // strict equality is fine for NONE, or use key.modifiers.is_empty()
                    else {
                        if let Some(item_index) = app.ui.dir_table_state.selected() {
                            let item = &app.dir.dir_items[item_index];
                            if let Ok(item_type) = item.file_type() {
                                match item_type {
                                    _ if item_type.is_dir() => {
                                        app.change_root(item.path(), true);
                                    }
                                    _ if item_type.is_file() => {
                                        todo!("Handle interaction with files")
                                    }
                                    _ => {}
                                }
                            }
                        };
                    }
                }
            }

            //BACK DIR
            code if app.input.keybinds_normal.back_dir.contains(&code) => {
                if key.is_press() {
                    if let Some(dir) = app.dir.root_dir.parent() {
                        app.change_root(dir.to_path_buf(), true);
                    }
                }
            }

            //BACK HISTORY
            code if app.input.keybinds_normal.back_history.contains(&code) => {
                if key.is_press() {
                    if let Some(previous_entry) = app.dir.history.pop() {
                        app.change_root(previous_entry, false);
                    } else {
                        app.exit = true;
                    }
                }
            }

            _ => {}
        },
    }
}

fn output_dir(app: &mut App) {
    app.output = app.dir.root_dir.to_str().unwrap_or("").to_string();
    app.exit = true;
}

//TODO: #[cfg(target_os = "windows")] for explorer opening on windows
