use crate::observers::observer::Observer;
use crate::observers::subject::GameStateEvent;

pub struct LoggerObserver;

impl LoggerObserver {
    // Create a new LoggerObserver instance
    pub fn new() -> Self { 
        Self 
    }
}

// Implements the Observer interface to react to state changes
impl Observer for LoggerObserver {

    // Log the game state event to the console when it occurs
    fn update(&mut self, event: &GameStateEvent) {
        match event {
            GameStateEvent::Idle        => {},  // no output for idle state
            GameStateEvent::GameStarted => println!("[State] Game started"),
            GameStateEvent::GamePaused  => println!("[State] Game paused"),
            GameStateEvent::GameResumed => println!("[State] Game resumed"),
            GameStateEvent::GameEnded   => println!("[State] Game ended"),
        }
    }

}
