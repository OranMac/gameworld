use crate::states::game_state::GameState;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::states::playing_state::PlayingState;
use crate::states::idle_state::IdleState;
use crate::states::game_over_state::GameOverState;
use crate::observers::subject::GameStateEvent;
use crate::entities::entity::Entity;
// PausedState (ConcreteState)
// Game is paused - no updates, waiting for resume
pub struct PausedState;

impl PausedState {
    pub fn new() -> Self {
        Self
    }
}

impl GameState for PausedState {
    fn enter(&mut self, ctx: &mut GameContext) {
        println!("[PausedState] Entered - game is paused");
        ctx.notify_observers(GameStateEvent::GamePaused);
    }

    fn exit(&mut self, _ctx: &mut GameContext) {
        println!("[PausedState] Exiting");
    }

    fn update(&mut self, _ctx: &mut GameContext,ent: &mut Entity) {
        println!("Step {} → Player  at {:?}", ent.name, ent.position);
    }

    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent, ent: &mut Entity) {
        match event {
            GameEvent::StartGame => ctx.set_state(Box::new(IdleState::new())),
            GameEvent::PauseGame => {}, // Already paused
            GameEvent::ResumeGame => {
                ctx.notify_observers(GameStateEvent::GameResumed);
                ctx.set_state(Box::new(PlayingState::new()));
            }
            GameEvent::EndGame => ctx.set_state(Box::new(GameOverState::new())),
            GameEvent::Tick => {}, // Ignore ticks while paused
        }
    }
}
