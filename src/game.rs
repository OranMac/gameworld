use crate::systems::movement::MovementStrategy;
use crate::systems::combat::CombatStrategy;
use crate::entities::entity::Entity;

pub struct Game;

impl Game {

    pub fn run(
        mut movement_strategy: Box<dyn MovementStrategy>,
        combat_strategy: Box<dyn CombatStrategy>,
    ) {
        
        println!("\n--- Starting Movement Simulation ---");
        let mut waypoint = (0.0, 0.0);
        for i in 0..5 {
            waypoint = movement_strategy.compute_next_waypoint(waypoint);
            println!("Step {}: Moved to position {:?}", i + 1, waypoint);
        }
        println!("------------------------------------\n");

        let attack_power = 10.0;  // attack strength
        let damage_dealt = combat_strategy.calculate_attack_damage(attack_power);
        let damage_received = combat_strategy.calculate_damage_taken(5.0);
        println!(
            "Combat Simulation: base attack strength {} -> damage dealt {} | damage received {}",
            attack_power, damage_dealt, damage_received
        );

    }
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
        let damage_dealt = player.attack(attack_power);
        let damage_received = enemy.defend(damage_dealt);

        println!(
            "Combat Simulation: Player '{}' attacks for {} → Enemy '{}' receives {}",
            player.name, damage_dealt, enemy.name, damage_received
        );
        println!("------------------------------------\n");
    }


}
