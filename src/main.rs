pub mod systems;
pub mod game;
pub mod utils;
pub mod entities;
pub mod factories;
pub mod observers;

use observers::game_state_subject::GameStateSubject;
use observers::subject::Subject;
use observers::logger_observer::LoggerObserver;
use systems::movement::{MovementStrategy, PatrolMovement, RandomMovement};
use systems::combat::{CombatStrategy, AggressiveCombat};
use utils::input::{read_choice};
use game::Game;
use entities::entity::{EmotionTarget,Entity};
use factories::player_factory::PlayerFactory;
use factories::enemy_factory::EnemyFactory;

use colored::Colorize;
use crate::systems::combat::EmotionalDamageCombat;
use crate::systems::movement::ScaredMovement;
use crate::utils::input::read_line;
fn main() {
    // ---------------- Create menu-driven Entity ----------------
        // Game state observer setup
    let mut state_subject = GameStateSubject::new();
    let _logger_id = state_subject.attach(Box::new(LoggerObserver::new()));
    let mut player_custom = Entity::default();
    let mut enemy_custom = Entity::default();
    for entity in [&mut player_custom,&mut enemy_custom]{

        // ----------------------- Define Name -----------------------//
        entity.name = read_line(
            "Define Name:"
        );

        // ----------------------- Emotional Weakness Choice -----------------------//
        let emotional_weakness = read_choice(
            "Select Emotional Weakness:\n1) Pride\n2) Ego\n3) Greed\n>",
            &["1", "2", "3"],
        );

        let emotion: EmotionTarget= match emotional_weakness.as_str() {
            "1" => {
                println!("You selected: Pride");
                EmotionTarget::Pride
            }
            "2" => {
                println!("You selected: Ego");
                EmotionTarget::Ego
            }
            "3" => {
                println!("You selected: Greed");
                EmotionTarget::Greed
            }
            _ => {
                unreachable!() // read_choice ensures this case is never hit
            }
        };
        entity.emo_weakness = emotion;
        // ----------------------- Movement Strategy Choice -----------------------//
        let movement_strategy = read_choice(
            "Select Movement Strategy:\n1) Patrol Movement\n2) Random Movement\n3) Scared Movement\n>",
            &["1", "2", "3"],
        );

        let movement: Box<dyn MovementStrategy> = match movement_strategy.as_str() {
            "1" => {
                println!("You selected: Patrol Movement");
                let mut vec: Vec<(f32,f32)> = vec![];
                loop {
                    let x = read_line("Add point for patrol fo X<f32>: ").parse::<f32>().unwrap();
                    let y = read_line("Add point for patrol fo Y<f32>: ").parse::<f32>().unwrap();
                    vec.push((x,y));
                    let choice = read_choice(
                    "Continue:\n1) Yes\n2) No",
                    &["1", "2"],
                    );
                    match choice.as_str() {
                        "1" => {}
                        "2" => {
                            break;
                        }
                        _ => {
                            unreachable!() // read_choice ensures this case is never hit
                        }
                    };
                }
                Box::new(PatrolMovement::new(vec))
            }
            "2" => {
                println!("You selected: Random Movement");
                
                Box::new(RandomMovement::new(
                    read_line("Randomnes <f32>: ").parse::<f32>().unwrap()))
            }
            "3" => {
                println!("You selected: Scared Movement");
                Box::new(ScaredMovement::new(
                    read_line("Speed <f32>: ").parse::<f32>().unwrap(),
                    (read_line("Scared of X<f32>: ").parse::<f32>().unwrap(),
                        read_line("Scared of Y<f32>: ").parse::<f32>().unwrap())))
            }
            _ => {
                unreachable!() // read_choice ensures this case is never hit
            }
        };
        entity.movement = movement;
        // ----------------------- Combat Strategy Choice -----------------------//
        
        let combat_strategy = read_choice(
            "\nSelect Combat Strategy:\n1) Aggressive Combat\n2) \"Exercise\" Emotonal Combat\n> ",
            &["1", "2"],
        );
        
        let combat: Box<dyn CombatStrategy> = match combat_strategy.as_str() {
        "1" => {
            println!("You selected: Aggressive Combat");
            Box::new(AggressiveCombat::new(
                read_line("Aggression <f32>: ").parse::<f32>().unwrap()))
        }
        "2" => {
            println!("You selected: Emotonal Combat");
                let emotional_target = read_choice(
                "Select Emotional Target:\n1) Pride\n2) Ego\n3) Greed\n>",
                &["1", "2", "3"],
            );

            let emotion: EmotionTarget= match emotional_target.as_str() {
                "1" => {
                    println!("You selected: Pride");
                    EmotionTarget::Pride
                }
                "2" => {
                    println!("You selected: Ego");
                    EmotionTarget::Ego
                }
                "3" => {
                    println!("You selected: Greed");
                    EmotionTarget::Greed
                }
                _ => {
                    unreachable!() // read_choice ensures this case is never hit
                }
            };
            Box::new(EmotionalDamageCombat::new(emotion))
        }
        _ => {
                unreachable!() // read_choice ensures this case is never hit
            }
        };
        entity.combat = combat;

        println!("{}", entity);
    }
    
    println!("\n>>> {} vs {} <<<",player_custom.name.bold().green(), enemy_custom.name.bold().bright_red());
    Game::run_entity(&mut player_custom, &mut enemy_custom);
    Game::run_with_state_events(&mut player_custom, &mut enemy_custom, &mut state_subject);

    // ---------------- Create factory-based Entities ----------------
    let player_factory = PlayerFactory;
    let enemy_factory = EnemyFactory;

    let mut player_factory_entity = Entity::new("Player (factory made)", EmotionTarget::Greed ,&player_factory);
    let mut enemy_factory_entity = Entity::new("Enemy (factory made)", EmotionTarget::Ego ,&enemy_factory);

    // // ---------------- Run Simulation ----------------

    println!("\n>>> {} vs {} <<<",player_factory_entity.name.bold().green(), enemy_factory_entity.name.bold().bright_red());
    Game::run_entity(&mut player_factory_entity, &mut enemy_factory_entity);
    Game::run_with_state_events(&mut player_factory_entity, &mut enemy_factory_entity, &mut state_subject);
}
