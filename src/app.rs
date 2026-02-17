use std::{fs::DirEntry, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::TableState};

use crate::{errors::AppError, ui, utils};

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: Screens,
    pub root_dir: PathBuf,
    pub dir_table_state: TableState,
    pub dir_items: Vec<DirEntry>,
    pub interact_state: InteractState,
    pub exit: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App, AppError> {
        if path.exists() {
            let items = utils::get_dir_content(&path).unwrap_or_default();
            let mut table_state = TableState::new();
            table_state.select(Some(0));
            let app = App {
                root_dir: path,
                dir_table_state: table_state,
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
            terminal
                .draw(|frame| ui::render(self, frame))
                .expect("Terminal failed to render");

            if let Event::Key(key) = event::read().map_err(AppError::ReadEventErr)? {
                match self.interact_state {
                    InteractState::Normal => match key.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Up => {
                            if key.kind == KeyEventKind::Press {
                                self.dir_table_state.select_previous();
                            }
                        }
                        KeyCode::Down => {
                            if key.kind == KeyEventKind::Press {
                                self.dir_table_state.select_next();
                            }
                        }
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
