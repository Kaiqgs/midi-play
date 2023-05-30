use super::game_mode::GameMode;

#[derive(PartialEq, Clone, Default)]
pub enum DrawState{
    #[default]
    Ok,
    ScaleChange,
    GameModeChange(GameMode),
    Pause,
}
