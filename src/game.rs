use crate::entities::entity::Entity;

pub struct Game;

impl Game {
        // Run movement and combat for two entities (player and enemy)
    pub fn run_entity(player: &mut Entity, enemy: &mut Entity) {
        
        println!("\n--- PLAYER MOVEMENT ---");
        for i in 0..5 {
            player.update_position();
            println!("Step {} → Player '{}' at {:?}", i + 1, player.name, player.position);
        }
        println!("------------------------------------\n");

        println!("--- ENEMY MOVEMENT ---");
        for i in 0..5 {
            enemy.update_position();
            println!("Step {} → Enemy '{}' at {:?}", i + 1, enemy.name, enemy.position);
        }
        println!("------------------------------------\n");

        // Combat
        let attack_power = 10.0;  // attack strength
        let damage_dealt = player.attack(enemy,attack_power);
        let damage_received = enemy.defend(damage_dealt);

        println!(
            "Combat Simulation: Player '{}' attacks for {} → Enemy '{}' receives {}",
            player.name, damage_dealt, enemy.name, damage_received
        );
        println!("------------------------------------\n");
    }


}
