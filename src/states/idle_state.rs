use crate::states::game_state::GameState;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::states::playing_state::PlayingState;
use crate::states::paused_state::PausedState;
use crate::states::game_over_state::GameOverState;
use crate::observers::subject::GameStateEvent;

// Initial state before the game starts
pub struct IdleState;

impl IdleState {
    pub fn new() -> Self {
        Self
    }
}

impl GameState for IdleState {
    fn enter(&mut self, ctx: &mut GameContext) {
        println!("[IdleState] Entered - waiting to start game");
        ctx.notify_observers(GameStateEvent::Idle);
    }

    fn exit(&mut self, _ctx: &mut GameContext) {
        println!("[IdleState] Exiting");
    }

    fn update(&mut self, _ctx: &mut GameContext) {
        // Nothing to update while idle
    }

    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent) {
        match event {
            GameEvent::StartGame => {}, // Cant just start new game while idle
            GameEvent::PauseGame => ctx.set_state(Box::new(PausedState::new())),
            GameEvent::ResumeGame => {}, // Game already in play
            GameEvent::EndGame => ctx.set_state(Box::new(GameOverState::new())),
            GameEvent::Tick => self.update(ctx),
        }
    }
}
