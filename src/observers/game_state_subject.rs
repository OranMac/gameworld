use crate::observers::observer::Observer;
use crate::observers::subject::{GameStateEvent, Subject};

// ConcreteSubject - manages observers and holds the current state
pub struct GameStateSubject {
    observers: Vec<(usize, Box<dyn Observer>)>,
    next_id: usize,
    current_state: GameStateEvent,  // GoF subjectState
}

impl GameStateSubject {
    
    // Create a new GameStateSubject with initial state
    pub fn new() -> Self { 
        Self { 
            observers: Vec::new(),
            next_id: 1,
            current_state: GameStateEvent::Idle,  // no notifications until set_state() called
        } 
    }

    // Returns current state for pull-model observers
    pub fn get_state(&self) -> GameStateEvent {
        self.current_state
    }

    // Stores new state and notifies all observers
    pub fn set_state(&mut self, state: GameStateEvent) {
        self.current_state = state;
        self.notify();
    }
}

// Implements the Subject trait to manage observers and broadcast state changes
impl Subject for GameStateSubject {
    
    // Attach a new observer and return its ID for later detachment
    fn attach(&mut self, obs: Box<dyn Observer>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.observers.push((id, obs));
        id
    }

    // Detach an observer by ID; returns true if removed
    fn detach(&mut self, id: usize) -> bool {
        let before = self.observers.len();
        self.observers.retain(|(observer_id, _)| *observer_id != id);
        self.observers.len() != before
    }
    
    // broadcasts state change to all observers
    fn notify(&mut self) {
        let state = self.current_state;
        for (_, o) in self.observers.iter_mut() { 
            o.update(&state); 
        }
    }
}
