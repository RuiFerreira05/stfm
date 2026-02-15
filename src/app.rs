use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::{errors::AppError, ui};

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: Screens,
    pub root_dir: PathBuf,
    pub interact_state: InteractState,
    pub exit: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App, AppError> {
        if path.exists() {
            let app = App {
                root_dir: path,
                ..Default::default()
            };
            Ok(app)
        } else {
            Err(AppError::GhostPath)
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        while !self.exit {
            terminal
                .draw(|frame| ui::render(self, frame))
                .expect("Terminal failed to render");

            if let Event::Key(key) = event::read().map_err(AppError::ReadEventErr)? {
                match self.interact_state {
                    InteractState::Normal => match key.code {
                        KeyCode::Char('q') => self.exit = true,
                        _ => {}
                    },
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub enum Screens {
    #[default]
    MainScreen,
}

#[derive(Debug, Default)]
pub enum InteractState {
    #[default]
    Normal,
}
