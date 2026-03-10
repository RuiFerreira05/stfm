use crate::interaction::keybinds::{KeybindsLog, KeybindsNormal};

pub mod input;
pub mod keybinds;

#[derive(Debug, Default)]
pub enum InteractState {
    #[default]
    Normal,
    Log,
}

#[derive(Debug, Default)]
pub struct Input {
    pub interact_state: InteractState,
    pub keybinds_normal: KeybindsNormal,
    pub keybinds_log: KeybindsLog,
}
