pub mod systems;
pub mod game;
pub mod utils;
pub mod entities;
pub mod factories;

use systems::movement::{MovementStrategy, PatrolMovement, RandomMovement};
use systems::combat::{CombatStrategy, AggressiveCombat};
use utils::input::{read_choice};
use game::Game;
use entities::entity::Entity;
use factories::player_factory::PlayerFactory;
use factories::enemy_factory::EnemyFactory;

use crate::systems::combat::PrideCombat;
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
        "\nSelect Combat Strategy:\n1) Aggressive Combat\n2) \"Exercise\" Pride Combat\n> ",
          &["1", "2"],
    );
    
    let combat: Box<dyn CombatStrategy> = match combat_strategy.as_str() {
       "1" => {
           println!("You selected: Aggressive Combat");
           Box::new(AggressiveCombat::new(1.5))
       }
       "2" => {
           println!("You selected: Pride Combat");
           Box::new(PrideCombat::new(1.5))
       }
       _ => {
            unreachable!() // read_choice ensures this case is never hit
          }
    };
 
    // ---------------- Create menu-driven Entity ----------------
    let mut player_custom = Entity::from_strategies("Player (user choice)", movement, combat);

    // ---------------- Create factory-based Entities ----------------
    let player_factory = PlayerFactory;
    let enemy_factory = EnemyFactory;

    let mut player_factory_entity = Entity::new("Player (factory made)", &player_factory);
    let mut enemy_factory_entity = Entity::new("Enemy (factory made)", &enemy_factory);

    // ---------------- Run Simulation ----------------
    println!("\n>>> Running Player (user choice) vs Enemy (factory made) <<<");
    Game::run_entity(&mut player_custom, &mut enemy_factory_entity);
    println!("\n>>> Running Player (factory made) vs Enemy (factory made) <<<");
    Game::run_entity(&mut player_factory_entity, &mut enemy_factory_entity);
}
