use crossterm::event::KeyCode;

use crate::utils;

#[derive(Debug)]
pub struct KeybindsNormal {
    pub exit: Vec<KeyCode>,
    pub select: Vec<KeyCode>,
    pub back_dir: Vec<KeyCode>,
    pub back_history: Vec<KeyCode>,
    pub navigate_up: Vec<KeyCode>,
    pub navigate_down: Vec<KeyCode>,
}

impl Default for KeybindsNormal {
    fn default() -> Self {
        Self {
            exit: utils::parse_keybind("q"),
            select: utils::parse_keybind("RIGHT|ENTER"),
            back_dir: utils::parse_keybind("LEFT"),
            back_history: utils::parse_keybind("BACKSPACE|ESC"),
            navigate_up: utils::parse_keybind("UP"),
            navigate_down: utils::parse_keybind("DOWN"),
        }
    }
}

impl KeybindsNormal {
    pub fn new() -> KeybindsNormal {
        KeybindsNormal::default()
    }
}
