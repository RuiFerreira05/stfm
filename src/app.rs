use std::path::PathBuf;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

use crate::{
    errors::AppError,
    interaction::{Input, InteractState, input},
    logger::Logger,
    navigator::Navigator,
    screens::Screens,
    ui::{self, UI},
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
            let mut app = App {
                navigator: Navigator::new(path.clone())?,
                ..Default::default()
            };
            app.logger
                .log_info(format!("App initialized with root path: {}", path.display()).as_str());
            Ok(app)
        } else {
            Err(AppError::GhostPath)
        }
    }

    pub fn toggle_logs(&mut self) {
        match self.ui.current_screen {
            Screens::MainScreen => {
                self.logger
                    .log_info("Toggling screen: MainScreen -> LogScreen");
                self.ui.current_screen = Screens::LogScreen;
                self.input.interact_state = InteractState::Log;
            }
            Screens::LogScreen => {
                self.logger
                    .log_info("Toggling screen: LogScreen -> MainScreen");
                self.ui.current_screen = Screens::MainScreen;
                self.input.interact_state = InteractState::Normal;
            }
        }
    }

    pub fn traverse(&mut self, dir: PathBuf, add_to_history: bool) {
        self.logger.log_info(
            format!(
                "Traversing to: {} (add_to_history: {})",
                dir.display(),
                add_to_history
            )
            .as_str(),
        );
        if let Err(e) = self.navigator.change_root(dir, add_to_history) {
            self.ui
                .display_error("Could not display directory's contents");
            self.logger.log_error(e.to_string().as_str());
        } else {
            self.logger.log_info(
                format!(
                    "Directory changed to: {} ({} items)",
                    self.navigator.root_dir.display(),
                    self.navigator.dir_items.len()
                )
                .as_str(),
            );
            self.ui.dir_table_state.select_first();
            self.ui.clear_error_msg();
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        self.logger.log_info("App run loop started");
        while !self.exit {
            // Draw the terminal
            terminal
                .draw(|frame| ui::render(self, frame))
                .expect("Terminal failed to render");

            //Handle user Interaction
            if let Event::Key(key) = event::read().map_err(AppError::ReadEventErr)? {
                input::handle_interaction(key, self);
            }
        }
        self.logger.log_info("App exiting");
        Ok(())
    }
}
