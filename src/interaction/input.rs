use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InteractState};

pub fn handle_interaction(key: KeyEvent, app: &mut App) {
    app.logger
        .log_debug(format!("key pressed -> {}", key.code).as_str());
    match app.interact_state {
        InteractState::Normal => match key.code {
            //EXIT
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.exit = true,
            code if app.keybinds_normal.exit.contains(&code) => app.exit = true,

            //NAVIGATE UP
            code if app.keybinds_normal.navigate_up.contains(&code) => {
                if key.is_press() {
                    app.dir_table_state.select_previous();
                }
            }

            //NAVIGATE DOWN
            code if app.keybinds_normal.navigate_down.contains(&code) => {
                if key.is_press() {
                    app.dir_table_state.select_next();
                }
            }

            //SELECT
            code if app.keybinds_normal.select.contains(&code) => {
                if key.is_press() {
                    // 1. Check for CONTROL using .contains() (Robust)
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        output_dir(app);
                    }
                    // 2. Check for NONE (or handle else for default behavior)
                    // strict equality is fine for NONE, or use key.modifiers.is_empty()
                    else {
                        if let Some(item_index) = app.dir_table_state.selected() {
                            let item = &app.dir_items[item_index];
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
            code if app.keybinds_normal.back_dir.contains(&code) => {
                if key.is_press() {
                    if let Some(dir) = app.root_dir.parent() {
                        app.change_root(dir.to_path_buf(), true);
                    }
                }
            }

            //BACK HISTORY
            code if app.keybinds_normal.back_history.contains(&code) => {
                if key.is_press() {
                    if let Some(previous_entry) = app.history.pop() {
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
    app.output = app.root_dir.to_str().unwrap_or("").to_string();
    app.exit = true;
}

//TODO: #[cfg(target_os = "windows")] for explorer opening on windows

//     // q
//     KeyCode::Char('q') => app.exit = true,

//     //CTRL-c
//     KeyCode::Char('c') => {
//         if key.modifiers == KeyModifiers::CONTROL {
//             app.exit = true;
//         }
//     }

//     //UP
//     KeyCode::Up => {
//         if key.kind == KeyEventKind::Press {
//             app.dir_table_state.select_previous();
//         }
//     }

//     //DOWN
//     KeyCode::Down => {
//         if key.kind == KeyEventKind::Press {
//             app.dir_table_state.select_next();
//         }
//     }

//     //ENTER | RIGHT
//     KeyCode::Enter | KeyCode::Right => {
//         if key.kind == KeyEventKind::Press {
//             if let Some(item_index) = app.dir_table_state.selected() {
//                 let item = &app.dir_items[item_index];
//                 if let Ok(item_type) = item.file_type() {
//                     match item_type {
//                         _ if item_type.is_dir() => {
//                             app.change_root(item.path(), true);
//                         }
//                         _ if item_type.is_file() => {
//                             todo!("Handle interaction with files")
//                         }
//                         _ => {}
//                     }
//                 }
//             };
//         }
//     }

//     // ESC | DELETE
//     KeyCode::Esc | KeyCode::Backspace => {
//         if key.is_press() {
//             if let Some(previous_entry) = app.history.pop() {
//                 app.change_root(previous_entry, false);
//             } else {
//                 app.exit = true;
//             }
//         }
//     }

//     KeyCode::Left => {
//         if key.is_press() {
//             if let Some(dir) = app.root_dir.parent() {
//                 app.change_root(dir.to_path_buf(), true);
//             }
//         }
//     }
