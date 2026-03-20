// Possible Game State Events
#[derive(Clone, Copy, Debug)]
pub enum GameStateEvent {
    Idle,         // Initial state before game starts
    GameStarted,
    GamePaused,
    GameResumed,
    GameEnded,
}

use crate::observers::observer::Observer;

// Defines the Subject for managing and notifying observers
pub trait Subject {
    fn attach(&mut self, obs: Box<dyn Observer>) -> usize;
    fn detach(&mut self, id: usize) -> bool;
    fn notify(&mut self);  // GoF: broadcast to all observers
}
