pub mod systems;

use systems::movement::{MovementStrategy, PatrolMovement, RandomMovement};

fn main() {

    println!("Select Movement Strategy:");
    println!("1) Patrol Movement");
    println!("2) Random Movement");

    let mut movement_strategy = String::new();
    std::io::stdin().read_line(&mut movement_strategy).expect("Failed to read input");

    match movement_strategy.trim() {
        "1" => {
            println!("You selected: Patrol Movement");
            println!("\n--- Starting Movement Simulation ---");
            let mut patrol_route = PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]);
            let mut current_waypoint = (0.0, 0.0);
            for _ in 0..5 {
                current_waypoint = patrol_route.compute_next_waypoint(current_waypoint);
                println!("Moved to position: {:?}", current_waypoint);
            }
            println!("------------------------------------\n");
        }
        "2" => {
            println!("You selected: Random Movement");
            println!("\n--- Starting Movement Simulation ---");
            let mut random_route = RandomMovement::new(2.0);
            let mut current_waypoint = (0.0, 0.0);
            for _ in 0..5 {
                current_waypoint = random_route.compute_next_waypoint(current_waypoint);
                println!("Moved to position: {:?}", current_waypoint);
            }
            println!("------------------------------------\n");
        }
        _ => {
            println!("Invalid choice, defaulting to Patrol Movement.");
            println!("\n--- Starting Movement Simulation ---");
            let mut patrol_route = PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]);
            let mut current_waypoint = (0.0, 0.0);
            for _ in 0..5 {
                current_waypoint = patrol_route.compute_next_waypoint(current_waypoint);
                println!("Moved to position: {:?}", current_waypoint);
            }
            println!("------------------------------------\n");
        }
    }
} 
