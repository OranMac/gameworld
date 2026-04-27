use crate::states::game_state::GameState;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::states::idle_state::IdleState;
use crate::observers::subject::GameStateEvent;

// GameOverState (ConcreteState)
// Game is paused - no updates, waiting for resume
pub struct GameOverState;

impl GameOverState {
    pub fn new() -> Self {
        Self
    }
}

impl GameState for GameOverState {
    fn enter(&mut self, ctx: &mut GameContext) {
        println!("[GameOverState] Game is over");
        ctx.notify_observers(GameStateEvent::GameEnded);
    }

    fn exit(&mut self, _ctx: &mut GameContext) {
        
    }

    fn update(&mut self, _ctx: &mut GameContext) {
        // No updates while paused
    }

    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent) {
        match event {
            GameEvent::StartGame => ctx.set_state(Box::new(IdleState::new())),
            GameEvent::PauseGame => {},
            GameEvent::ResumeGame => {},
            GameEvent::EndGame =>{},
            GameEvent::Tick => {},
        }
    }
}
