use std::{fs::DirEntry, path::PathBuf};

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, widgets::TableState};

use crate::{
    dir::Dir,
    errors::AppError,
    interaction::{InteractState, input, keybinds::KeybindsNormal},
    logger::Logger,
    ui::{self, UI},
    utils,
};

#[derive(Debug, Default)]
pub struct App {
    pub ui: UI,
    pub dir: Dir,
    pub root_dir: PathBuf,
    pub history: Vec<PathBuf>,
    pub dir_items: Vec<DirEntry>,
    pub interact_state: InteractState,
    pub keybinds_normal: KeybindsNormal,
    pub items_changed: bool,
    pub logger: Logger,
    pub output: String,
    pub exit: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App, AppError> {
        if path.exists() {
            let items = utils::get_dir_content(&path).unwrap_or_default();
            let app = App {
                root_dir: path,
                dir_items: items,
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
            if self.items_changed {
                self.dir_items = utils::get_dir_content(&self.root_dir)?;
                self.items_changed = false;
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
            self.history.push(self.root_dir.clone());
        }
        self.root_dir = dir;
        self.items_changed = true;

        self.ui.dir_table_state.select_first();
    }
}
