use crate::observers::subject::GameStateEvent;

// Defines the interface for objects that should be notified of state changes
pub trait Observer {
    fn update(&mut self, event: &GameStateEvent);
}

