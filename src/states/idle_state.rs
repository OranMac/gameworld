use crate::states::game_state::GameState;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::states::paused_state::PausedState;
use crate::states::game_over_state::GameOverState;
use crate::observers::subject::GameStateEvent;
use crate::entities::entity::Entity;
use crate::states::playing_state::PlayingState;

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

    fn update(&mut self, _ctx: &mut GameContext, ent: &mut Entity) {
        println!("Step {} → Player  at {:?}", ent.name, ent.position);
    }

    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent, ent: &mut Entity) {
        match event {
            GameEvent::StartGame => { ctx.set_state(Box::new(PlayingState::new()))},
            GameEvent::PauseGame => ctx.set_state(Box::new(PausedState::new())),
            GameEvent::ResumeGame => {}, // Game already in play
            GameEvent::EndGame => ctx.set_state(Box::new(GameOverState::new())),
            GameEvent::Tick => self.update(ctx, ent),
        }
    }
}
