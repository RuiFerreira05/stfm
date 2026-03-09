use ratatui::{Frame, widgets::TableState};

use crate::{
    app::App,
    screens::{self, Screens, main_screen},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // render based on screen
    match app.ui.current_screen {
        screens::Screens::MainScreen => main_screen::render(app, frame),
        screens::Screens::LogScreen => todo!(),
    }
}

#[derive(Debug)]
pub struct UI {
    pub current_screen: Screens,
    pub error_message: String,
    pub dir_table_state: TableState,
}

impl Default for UI {
    fn default() -> Self {
        let mut table_state = TableState::new();
        table_state.select(Some(0));
        Self {
            current_screen: Default::default(),
            error_message: Default::default(),
            dir_table_state: table_state,
        }
    }
}
