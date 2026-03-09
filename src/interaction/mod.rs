use crate::interaction::keybinds::KeybindsNormal;

pub mod input;
pub mod keybinds;

#[derive(Debug, Default)]
pub enum InteractState {
    #[default]
    Normal,
}

#[derive(Debug, Default)]
pub struct Input {
    pub interact_state: InteractState,
    pub keybinds_normal: KeybindsNormal,
}
