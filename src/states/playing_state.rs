use crate::states::game_state::GameState;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::states::paused_state::PausedState;
use crate::states::game_over_state::GameOverState;
use crate::states::idle_state::IdleState;
use crate::observers::subject::GameStateEvent;


// Active gameplay state - processes movement, combat, etc.
pub struct PlayingState {
    tick_count: u32,
}

impl PlayingState {
    pub fn new() -> Self {
        Self { tick_count: 0 }
    }
}

impl GameState for PlayingState {
    fn enter(&mut self, ctx: &mut GameContext) {
        println!("[PlayingState] Entered - game is now running");
        ctx.notify_observers(GameStateEvent::GameStarted);
    }

    fn exit(&mut self, _ctx: &mut GameContext) {
        println!("[PlayingState] Exiting after {} ticks", self.tick_count);
    }

    fn update(&mut self, _ctx: &mut GameContext) {
        self.tick_count += 1;
        // Game logic would go here: movement, combat, etc.
    }

    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent) {
        match event {
            GameEvent::StartGame => ctx.set_state(Box::new(IdleState::new())),
            GameEvent::PauseGame => ctx.set_state(Box::new(PausedState::new())),
            GameEvent::ResumeGame => {}, // Already playing
            GameEvent::EndGame => ctx.set_state(Box::new(GameOverState::new())),
            GameEvent::Tick => self.update(ctx),
        }
    }
}
