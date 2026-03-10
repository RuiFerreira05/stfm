use crossterm::event::KeyCode;

#[derive(Debug)]
pub struct KeybindsNormal {
    pub exit: Vec<KeyCode>,
    pub select: Vec<KeyCode>,
    pub back_dir: Vec<KeyCode>,
    pub back_history: Vec<KeyCode>,
    pub navigate_up: Vec<KeyCode>,
    pub navigate_down: Vec<KeyCode>,
    pub toggle_log: Vec<KeyCode>,
}

impl Default for KeybindsNormal {
    fn default() -> Self {
        Self {
            exit: parse_keybind("q"),
            select: parse_keybind("RIGHT|ENTER"),
            back_dir: parse_keybind("LEFT"),
            back_history: parse_keybind("BACKSPACE|ESC"),
            navigate_up: parse_keybind("UP"),
            navigate_down: parse_keybind("DOWN"),
            toggle_log: parse_keybind("t"),
        }
    }
}

impl KeybindsNormal {
    pub fn new() -> KeybindsNormal {
        KeybindsNormal::default()
    }
}

#[derive(Debug)]
pub struct KeybindsLog {
    pub toggle_log: Vec<KeyCode>,
    pub toggle_scroll: Vec<KeyCode>,
}

impl Default for KeybindsLog {
    fn default() -> Self {
        Self {
            toggle_log: parse_keybind("t"),
            toggle_scroll: parse_keybind("s"),
        }
    }
}

impl KeybindsLog {
    pub fn new() -> KeybindsLog {
        KeybindsLog::default()
    }
}

pub fn parse_keybind(keys: &str) -> Vec<KeyCode> {
    let parts = keys.split("|");
    let mut keybind = Vec::new();
    for part in parts {
        match part.to_uppercase().trim() {
            "BACKSPACE" => keybind.push(KeyCode::Backspace),
            "ENTER" => keybind.push(KeyCode::Enter),
            "LEFT" => keybind.push(KeyCode::Left),
            "RIGHT" => keybind.push(KeyCode::Right),
            "UP" => keybind.push(KeyCode::Up),
            "DOWN" => keybind.push(KeyCode::Down),
            "ESC" => keybind.push(KeyCode::Esc),
            "TAB" => keybind.push(KeyCode::Tab),
            _ => {
                if part.len() == 1 {
                    let chars: Vec<char> = part.chars().collect();
                    keybind.push(KeyCode::Char(chars[0]));
                } else {
                    keybind.push(KeyCode::Null);
                }
            }
        }
    }
    return keybind;
}
