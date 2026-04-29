use crate::entities::entity::Entity;
use crate::states::game_context::GameContext;
use crate::states::game_event::GameEvent;
use crate::utils::input::read_choice;
use colored::Colorize;

pub struct Game;

impl Game {
    /// Display the game event menu and return the user's choice
    fn display_event_menu(x: u32) -> String {
        println!("\n{}", "=".repeat(40).cyan());
        println!("{} {}", "x:".bold(), x.to_string().yellow());
        println!("{}", "=".repeat(40).cyan());
        println!("{}", "--- Game Event Menu ---".bold().green());
        println!("1) {} - Transition to Playing state", "Start Game".green());
        println!("2) {} - Transition to Paused state", "Pause Game".yellow());
        println!("3) {} - Resume", "Resume Game".cyan());
        println!("4) {} - Transition to GameOver state", "End Game".red());
        println!("5) {} - Execute game tick/update", "Tick".blue());
        println!("6) {} - Exit the loop", "Exit Loop".magenta());
        println!("{}", "=".repeat(40).cyan());
        
        read_choice("> ", &["1", "2", "3", "4", "5", "6"])
    }

    /// Convert user menu choice to GameEvent
    fn choice_to_event(choice: &str) -> Option<GameEvent> {
        match choice {
            "1" => Some(GameEvent::StartGame),
            "2" => Some(GameEvent::PauseGame),
            "3" => Some(GameEvent::ResumeGame),
            "4" => Some(GameEvent::EndGame),
            "5" => Some(GameEvent::Tick),
            "6" => None,
            _ => None,
        }
    }

    // Run movement and combat for two entities (player and enemy)
    pub fn run_with_state_events(player: &mut Entity, enemy: &mut Entity, ctx: &mut GameContext) {
        ctx.handle_event(GameEvent::StartGame, player);
        // Save checkpoint before movement
        let player_checkpoint = player.create_memento();
        println!("\n--- PLAYER MOVEMENT (Interactive Mode) ---");
        
        let mut iteration = 0;
        loop {
            iteration += 1;
            
            // Display menu and get user choice
            let choice = Self::display_event_menu(iteration);
            
            // Convert choice to event
            match Self::choice_to_event(&choice) {
                Some(event) => {
                    println!("\n{} {:?}", "Executing event:".bold(), event);
                    ctx.handle_event(event, player);
                    
                    // Check if game ended
                    if matches!(event, GameEvent::EndGame) {
                        println!("{}", "\nGame ended by user selection.".red().bold());
                        break;
                    }
                }
                None => {
                    // User chose to exit
                    println!("{}", "\nExiting game loop".magenta().bold());
                    break;
                }
            }
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

