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
use crate::utils::input::{read_float, read_line};

fn emotion_user_select() -> EmotionTarget {
    let emotion = read_choice(
        "Select Emotion:\n1) Pride\n2) Ego\n3) Greed\n>",
        &["1", "2", "3"],
    );

    return match emotion.as_str() {
        "1" => {
            println!("You selected: {}", "Pride".underline().bold().italic());
            EmotionTarget::Pride
        }
        "2" => {
            println!("You selected: {}", "Ego".purple().blink());
            EmotionTarget::Ego
        }
        "3" => {
            println!("You selected: {}", "Greed".green().italic());
            EmotionTarget::Greed
        }
        _ => {
            unreachable!(); // read_choice ensures this case is never hit
        }
    };
}

fn custom_selection(entity: &mut Entity){
    // Name Definition
    entity.name = read_line(
        "Define Name:"
    );

    // Emotional Weakness
    entity.emo_weakness = emotion_user_select();
    
    // Movement Strategy
    let movement_strategy = read_choice(
        "Select Movement Strategy:\n1) Patrol Movement\n2) Random Movement\n3) Scared Movement\n>",
        &["1", "2", "3"],
    );

    let movement: Box<dyn MovementStrategy> = match movement_strategy.as_str() {
        "1" => {
            println!("You selected: Patrol Movement");
            let mut vec: Vec<(f32,f32)> = vec![];
            loop {
                let x = read_float("Add point for patrol fo X<f32>: ");
                let y = read_float("Add point for patrol fo Y<f32>: ");
                vec.push((x,y));
                let choice = read_choice(
                "Continue:\n1) Yes\n2) No\n>",
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
                read_float("Randomnes <f32>: ")))
        }
        "3" => {
            println!("You selected: Scared Movement");
            Box::new(ScaredMovement::new(
                read_float("Speed <f32>: "),
                (read_float("Scared of X<f32>: "),
                    read_float("Scared of Y<f32>: "))))
        }
        _ => {
            unreachable!() // read_choice ensures this case is never hit
        }
    };
    entity.movement = movement;
    // ----------------------- Combat Strategy Choice -----------------------//
    
    let combat_strategy = read_choice(
        "\nSelect Combat Strategy:\n1) Aggressive Combat\n2) Emotonal Combat\n> ",
        &["1", "2"],
    );
    
    let combat: Box<dyn CombatStrategy> = match combat_strategy.as_str() {
    "1" => {
        println!("You selected: {}", "Aggressive Combat".red().bold());
        Box::new(AggressiveCombat::new(
            read_float("Aggression <f32>: ")))
    }
    "2" => {
        println!("You selected: {}", "Emotonal Combat".blue().italic());
        Box::new(EmotionalDamageCombat::new(emotion_user_select()))
    }
    _ => {
            unreachable!() // read_choice ensures this case is never hit
        }
    };
    entity.combat = combat;

    println!("{}", entity);
}

fn main() {
    // ---------------- Create menu-driven Entity ----------------
        // Game state observer setup
    let mut state_subject = GameStateSubject::new();
    let _logger_id = state_subject.attach(Box::new(LoggerObserver::new()));
    let mut player = Entity::default();
    let mut enemy = Entity::default();
    let choice = read_choice(
        "Playthrough Method:\n1) Custom\n2) Factory\n>",
        &["1", "2"],
    );
    match choice.as_str() {
        "1" => {
            println!("You selected Custom");
            for entity in [&mut player,&mut enemy]{
                custom_selection(entity);
            }
            
        }
        "2" => {
            println!("You selected Factory");
            let player_factory = PlayerFactory;
            let enemy_factory = EnemyFactory;
            player = Entity::new(&read_line("Define Player Name:"), &player_factory);
            enemy = Entity::new(&read_line("Define Enemy Name:"), &enemy_factory);
        }
        _ => {
            unreachable!() // read_choice ensures this case is never hit
        }
    };
    Game::run_with_state_events(&mut player, &mut enemy, &mut state_subject);
}
