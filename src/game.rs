use crate::systems::movement::MovementStrategy;

pub struct Game;

impl Game {

    pub fn run<M: MovementStrategy>(mut movement_strategy: M) {
        let mut waypoint = (0.0, 0.0);
        println!("\n--- Starting Movement Simulation ---");
        for i in 0..5 {
            waypoint = movement_strategy.compute_next_waypoint(waypoint);
            println!("Step {}: Moved to position {:?}", i + 1, waypoint);
        }
        println!("------------------------------------\n");
    }

}
