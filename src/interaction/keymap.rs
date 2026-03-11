use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::screens::Screens;

use super::action::Action;

#[derive(Debug)]
pub struct Keymap {
    global: HashMap<KeyEvent, Action>,
    screen: HashMap<Screens, HashMap<KeyEvent, Action>>,
}

/// Helper to build a `KeyEvent` for press events with given code and modifiers.
fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent::new(code, modifiers)
}

/// Shorthand for a plain key press (no modifiers).
fn plain_key(code: KeyCode) -> KeyEvent {
    key(code, KeyModifiers::NONE)
}

impl Default for Keymap {
    fn default() -> Self {
        // GLOBAL
        let mut global: HashMap<KeyEvent, Action> = HashMap::new();

        global.insert(key(KeyCode::Char('c'), KeyModifiers::CONTROL), Action::Exit);
        global.insert(plain_key(KeyCode::Char('q')), Action::Exit);
        global.insert(plain_key(KeyCode::Char('t')), Action::ToggleLogs);

        // MAIN SCREEN
        let mut main_screen: HashMap<KeyEvent, Action> = HashMap::new();

        main_screen.insert(plain_key(KeyCode::Up), Action::NavigateUp);
        main_screen.insert(plain_key(KeyCode::Down), Action::NavigateDown);

        main_screen.insert(plain_key(KeyCode::Right), Action::Select);
        main_screen.insert(plain_key(KeyCode::Enter), Action::Select);
        main_screen.insert(
            key(KeyCode::Right, KeyModifiers::CONTROL),
            Action::CtrlSelect,
        );
        main_screen.insert(
            key(KeyCode::Enter, KeyModifiers::CONTROL),
            Action::CtrlSelect,
        );

        main_screen.insert(plain_key(KeyCode::Left), Action::BackDir);

        main_screen.insert(plain_key(KeyCode::Backspace), Action::BackHistory);
        main_screen.insert(plain_key(KeyCode::Esc), Action::BackHistory);

        // LOGSCREEN
        let mut log_screen: HashMap<KeyEvent, Action> = HashMap::new();
        log_screen.insert(plain_key(KeyCode::Char('s')), Action::ToggleScroll);

        let mut screen: HashMap<Screens, HashMap<KeyEvent, Action>> = HashMap::new();
        screen.insert(Screens::MainScreen, main_screen);
        screen.insert(Screens::LogScreen, log_screen);

        Self { global, screen }
    }
}

impl Keymap {
    /// Resolve a key event to an action.
    /// Screen-specific bindings take priority over global ones.
    pub fn resolve(&self, screen: &Screens, key: &KeyEvent) -> Option<&Action> {
        self.screen
            .get(screen)
            .and_then(|m| m.get(key))
            .or_else(|| self.global.get(key))
    }
}
