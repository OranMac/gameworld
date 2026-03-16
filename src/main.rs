pub mod systems;
pub mod game;
pub mod utils;

use systems::movement::{MovementStrategy, PatrolMovement, RandomMovement};
use systems::combat::{CombatStrategy, AggressiveCombat};
use utils::input::{read_choice};
use game::Game;

fn main() {

    // ----------------------- Movement Strategy Choice -----------------------//
    let movement_strategy = read_choice(
        "Select Movement Strategy:\n1) Patrol Movement\n2) Random Movement\n> ",
        &["1", "2"],
    );

    let movement: Box<dyn MovementStrategy> = match movement_strategy.as_str() {
        "1" => {
            println!("You selected: Patrol Movement");
            Box::new(PatrolMovement::new(vec![(0.0, 0.0), (5.0, 0.0), (5.0, 5.0)]))
        }
        "2" => {
            println!("You selected: Random Movement");
            Box::new(RandomMovement::new(2.0))
        }
        _ => {
            unreachable!() // read_choice ensures this case is never hit
        }
    };

    // ----------------------- Combat Strategy Choice -----------------------//
    
    let combat_strategy = read_choice(
        "\nSelect Combat Strategy:\n1) Aggressive Combat\n2) \"Exercise\" Combat, to be completed later\n> ",
          &["1", "2"],
    );
    
    let combat: Box<dyn CombatStrategy> = match combat_strategy.as_str() {
       "1" => {
           println!("You selected: Aggressive Combat");
           Box::new(AggressiveCombat::new(1.5))
       }
       "2" => {
           println!("You selected: A combat approach not yet coded, defaulting to Aggressive");
           Box::new(AggressiveCombat::new(1.2))
       }
       _ => {
            unreachable!() // read_choice ensures this case is never hit
          }
    };
 
    Game::run(movement, combat); // generic call; objects defined at runtime
}
