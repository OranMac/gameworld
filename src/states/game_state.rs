use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;

// Defines the interface for different game states
// Each state handles its own behaviour for entering, exiting, updating, and event handling
pub trait GameState {
    // Called when transitioning INTO this state
    fn enter(&mut self, ctx: &mut GameContext);
    
    // Called when transitioning OUT OF this state
    fn exit(&mut self, ctx: &mut GameContext);
    
    // Called each game tick while in this state
    fn update(&mut self, ctx: &mut GameContext);
    
    // Handle events (input, game events, etc.)
    fn handle_event(&mut self, ctx: &mut GameContext, event: GameEvent);
}
