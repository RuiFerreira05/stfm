pub mod log_screen;
pub mod main_screen;

#[derive(Debug, Default)]
pub enum Screens {
    #[default]
    MainScreen = 0,
    LogScreen = 1,
}
