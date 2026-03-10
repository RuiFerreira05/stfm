pub mod log_screen;
pub mod main_screen;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screens {
    #[default]
    MainScreen = 0,
    LogScreen = 1,
}
