use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

use crate::{
    dir::Navigator,
    errors::AppError,
    interaction::{Input, input},
    logger::Logger,
    ui::{self, UI},
    utils,
};

#[derive(Debug, Default)]
pub struct App {
    pub ui: UI,
    pub navigator: Navigator,
    pub input: Input,
    pub logger: Logger,
    pub output: String,
    pub exit: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App, AppError> {
        if path.exists() {
            let app = App {
                navigator: Navigator::new(path)?,
                ..Default::default()
            };
            Ok(app)
        } else {
            Err(AppError::GhostPath)
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        while !self.exit {
            // Find new items
            if self.navigator.items_changed {
                self.navigator.dir_items = utils::get_dir_content(&self.navigator.root_dir)?;
                self.navigator.items_changed = false;
            }

            // Draw the terminal
            terminal
                .draw(|frame| ui::render(self, frame))
                .expect("Terminal failed to render");

            //Handle user Interaction
            if let Event::Key(key) = event::read().map_err(AppError::ReadEventErr)? {
                input::handle_interaction(key, self);
            }
        }
        Ok(())
    }

    pub fn change_root(&mut self, dir: PathBuf, add_to_history: bool) {
        if add_to_history {
            self.navigator.history.push(self.navigator.root_dir.clone());
        }
        self.navigator.root_dir = dir;
        self.navigator.items_changed = true;

        self.ui.dir_table_state.select_first();
    }
}
