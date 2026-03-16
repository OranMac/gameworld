pub mod systems;
pub mod game;

use crate::systems::movement::{MovementStrategy, PatrolMovement, RandomMovement};
use crate::systems::combat::{CombatStrategy, AggressiveCombat};
use crate::game::Game;

fn main() {

    println!("Select Movement Strategy:");
    println!("1) Patrol Movement");
    println!("2) Random Movement");

    let mut movement_strategy = String::new();
    std::io::stdin().read_line(&mut movement_strategy).expect("Failed to read input");

    let movement: Box<dyn MovementStrategy> = match movement_strategy.trim() {
        "1" => {
            println!("You selected: Patrol Movement");
            Box::new(PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]))
        }
        "2" => {
            println!("You selected: Random Movement");
            Box::new(RandomMovement::new(2.0))
        }
        _ => {
            println!("Invalid choice, defaulting to Patrol Movement.");
            Box::new(PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]))
        }
    };

    println!("Select Combat Strategy:");
    println!("1) Aggressive Combat");
    println!("2) \"Exercise\" Combat, to be completed later Movement");

    let mut combat_strategy = String::new();
    std::io::stdin().read_line(&mut combat_strategy).expect("Failed to read input");

    let combat: Box<dyn CombatStrategy> = match combat_strategy.trim() {
        "1" => {
            println!("You selected: Aggressive Combat");
            Box::new(AggressiveCombat::new(1.5))
        }
        "2" => {
            println!("You selected: Exercise Combat (defaulting to Aggressive)");
            Box::new(AggressiveCombat::new(1.2)) 
        }
        _ => {
            println!("Invalid choice, defaulting to Aggressive Combat");
            Box::new(AggressiveCombat::new(1.2))
        }
    };

    Game::run(movement, combat); // generic call; objects defined at runtime

}
