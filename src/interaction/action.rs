#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Exit,
    NavigateUp,
    NavigateDown,
    Select,
    CtrlSelect,
    BackDir,
    BackHistory,
    ToggleLogs,
    ToggleScroll,
}
