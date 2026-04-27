// Input events that drive state transitions
// These are commands/inputs, distinct from GameStateEvent which are notifications/outputs
#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    StartGame,
    PauseGame,
    ResumeGame,
    EndGame,
    Tick,
}
