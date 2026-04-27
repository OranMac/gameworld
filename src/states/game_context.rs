use crate::states::game_state::GameState;
use crate::states::game_event::GameEvent;
use crate::observers::game_state_subject::GameStateSubject;
use crate::observers::subject::GameStateEvent;
use crate::observers::subject::Subject;

pub struct GameContext {
    current_state: Option<Box<dyn GameState>>,
    subject: GameStateSubject,
}

impl GameContext {
    // Create a new GameContext with an initial state
    pub fn new(initial_state: Box<dyn GameState>) -> Self {
        let mut ctx = Self {
            current_state: None,
            subject: GameStateSubject::new(),
        };
        ctx.set_state(initial_state);
        ctx
    }

    // Transition to a new state
    // Calls exit() on old state, then enter() on new state
    pub fn set_state(&mut self, new_state: Box<dyn GameState>) {
        // Exit current state if one exists
        if let Some(mut old_state) = self.current_state.take() {
            old_state.exit(self);
        }
        
        // Call enter on new state
        let mut state = new_state;
        state.enter(self);
        self.current_state = Some(state);
    }

    // Delegate update to current state
    // Delegate update to current state
       // Delegate update to current state
    pub fn update(&mut self) {
        if let Some(mut state) = self.current_state.take() {
            state.update(self);
            // Check if a state transition happened during update
            if self.current_state.is_some() {
                // A new state was set, call exit on the old state
                state.exit(self);
            } else {
                // No state transition, restore the original state
                self.current_state = Some(state);
            }
        }
    }

    // Delegate event handling to current state
    pub fn handle_event(&mut self, event: GameEvent) {
        if let Some(mut state) = self.current_state.take() {
            state.handle_event(self, event);
            // Check if a state transition happened during event handling
            if self.current_state.is_some() {
                // A new state was set, call exit on the old state
                state.exit(self);
            } else {
                // No state transition, restore the original state
                self.current_state = Some(state);
            }
        }
    }


    // Notify observers of state change (helper for states to use)
    pub fn notify_observers(&mut self, event: GameStateEvent) {
        self.subject.set_state(event);
    }

    // Attach an observer (returns ID for later detachment)
    pub fn attach_observer(&mut self, observer: Box<dyn crate::observers::observer::Observer>) -> usize {
        self.subject.attach(observer)
    }
}
