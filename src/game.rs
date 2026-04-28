use crate::entities::entity::Entity;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use colored::Colorize;
pub struct Game;

impl Game {
    // Run movement and combat for two entities (player and enemy)
       pub fn run_with_state_events(player: &mut Entity, enemy: &mut Entity, ctx: &mut GameContext) {
        ctx.handle_event(GameEvent::StartGame,player);
        // Save checkpoint before movement
        let player_checkpoint = player.create_memento();
        println!("\n--- PLAYER MOVEMENT ---");
        for i in 0..5 {
            if i == 2 { ctx.handle_event(GameEvent::PauseGame, player); }
            if i == 4 { ctx.handle_event(GameEvent::ResumeGame, player); }
            ctx.handle_event(GameEvent::Tick,player);
        }
        println!("------------------------------------\n");
         // Restore to checkpoint
        println!("\n--- RESTORING FROM CHECKPOINT ---");
        player.restore_from_memento(&player_checkpoint);
        println!("Player restored to: {:?}", player.position);
        
        println!("\n--- ENEMY MOVEMENT ---");
        for i in 0..5 {
            enemy.update_position();
            println!("Step {} → Enemy '{}' at {:?}", i + 1, enemy.name.red(), enemy.position);
        }
        println!("------------------------------------\n");

        let attack_power = 10.0;  // attack strength
        let damage_dealt = player.attack(&enemy,attack_power);
        let damage_received = enemy.defend(damage_dealt);

        println!(
            "Combat Simulation: Player '{}' attacks for {} → Enemy '{}' receives {}",
            player.name.green(), damage_dealt, enemy.name.red(), damage_received
        );

        ctx.handle_event(GameEvent::EndGame, player);
    }


    
}

