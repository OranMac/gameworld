pub mod systems;
pub mod game;

use systems::movement::{PatrolMovement, RandomMovement};
use systems::combat::{CombatStrategy, AggressiveCombat};
use game::Game;

fn main() {

    println!("Select Movement Strategy:");
    println!("1) Patrol Movement");
    println!("2) Random Movement");

    let mut movement_strategy = String::new();
    std::io::stdin().read_line(&mut movement_strategy).expect("Failed to read input");

      match movement_strategy.trim() {
        "1" => {
            println!("You selected: Patrol Movement");
            let movement_route = PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]);
             Game::run(movement_route, AggressiveCombat::new(1.5)); // generic call, type known at compile time
        }
        "2" => {
            println!("You selected: Random Movement");
            let movement_route = RandomMovement::new(2.0);
            Game::run(movement_route, AggressiveCombat::new(2.0)); // generic call, type known at compile time
        }
        _ => {
            println!("Invalid choice, defaulting to Patrol Movement.");
            let movement_route = PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]);
             Game::run(movement_route, AggressiveCombat::new(1.3));
        }
    }
}
