use crate::systems::movement::MovementStrategy;
use crate::systems::combat::CombatStrategy;

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

}
