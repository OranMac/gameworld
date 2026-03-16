pub mod systems;

use systems::movement::{MovementStrategy, PatrolMovement};

fn main() {

    println!("\n--- Starting Movement Simulation ---");

    // Define the patrol route i.e. three waypoints to patrol here.
    let mut patrol_route = PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]);

    // Start at (0.0, 0.0) and move between the defined patrol waypoints
    let mut current_waypoint = (0.0, 0.0);
    for _ in 0..5 {
        current_waypoint = patrol_route.compute_next_waypoint(current_waypoint);
        println!("Moved to waypoint: {:?}", current_waypoint);
    }

    println!("------------------------------------\n");

}
