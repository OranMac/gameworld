use crate::entities::entity::Entity;
use crate::GameStateSubject;
use crate::observers::subject::GameStateEvent;
use colored::Colorize;
pub struct Game;

impl Game {
    // Run movement and combat for two entities (player and enemy)
    pub fn run_with_state_events(player: &mut Entity, enemy: &mut Entity, subject: &mut GameStateSubject) {
        subject.set_state(GameStateEvent::GameStarted);
        
        println!("\n--- PLAYER MOVEMENT ---");
        for i in 0..5 {
            player.update_position();
            println!("Step {} → Player '{}' at {:?}", i + 1, player.name.green(), player.position);
            if i == 1 { subject.set_state(GameStateEvent::GamePaused); }
            if i == 2 { subject.set_state(GameStateEvent::GameResumed); }
        }
        println!("------------------------------------\n");

        println!("--- ENEMY MOVEMENT ---");
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

        subject.set_state(GameStateEvent::GameEnded);
    }


    
}

