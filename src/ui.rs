use ratatui::Frame;

use crate::{
    app::App,
    screens::{self, main_screen},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // render based on screen
    match app.current_screen {
        screens::Screens::MainScreen => main_screen::render(app, frame),
        screens::Screens::LogScreen => todo!(),
    }
}
