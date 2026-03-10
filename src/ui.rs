use ratatui::{Frame, widgets::TableState};

use crate::{
    app::App,
    screens::{self, Screens, log_screen, main_screen},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // render based on screen
    match app.ui.current_screen {
        screens::Screens::MainScreen => {
            app.logger.log_info("Rendering MainScreen");
            main_screen::render(app, frame);
        }
        screens::Screens::LogScreen => {
            app.logger.log_info("Rendering LogScreen");
            log_screen::render(app, frame);
        }
    }
}

#[derive(Debug)]
pub struct UI {
    pub current_screen: Screens,
    pub error_message: String,
    pub dir_table_state: TableState,
    pub log_list_state: TableState,
}

impl Default for UI {
    fn default() -> Self {
        let mut table_state = TableState::new();
        table_state.select_first();
        let mut log_state = TableState::new();
        log_state.select_last();
        Self {
            current_screen: Default::default(),
            error_message: Default::default(),
            dir_table_state: table_state,
            log_list_state: log_state,
        }
    }
}

impl UI {
    pub fn display_error(&mut self, str: &str) {
        self.error_message = str.to_string()
    }

    pub(crate) fn clear_error_msg(&mut self) {
        self.error_message = String::new()
    }
}
