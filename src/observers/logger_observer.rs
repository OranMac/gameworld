use crate::observers::observer::Observer;
use crate::observers::subject::GameStateEvent;
use colored::Colorize;
use emojis;

pub struct LoggerObserver;

impl LoggerObserver {
    // Create a new LoggerObserver instance
    pub fn new() -> Self { 
        Self 
    }
}

// Implements the Observer interface to react to state changes
impl Observer for LoggerObserver {
    // Log the game state event to the console when it occurs
    fn update(&mut self, event: &GameStateEvent) {
        match event {
            GameStateEvent::Idle        => {},  // no output for idle state
            GameStateEvent::GameStarted => println!("{} {}",emojis::get_by_shortcode("fast_forward").unwrap(),"Game started".green()),
            GameStateEvent::GamePaused  => println!("{} {}",emojis::get_by_shortcode("pause_button").unwrap(),"Game paused".yellow()),
            GameStateEvent::GameResumed => println!("{} {}",emojis::get_by_shortcode("arrow_forward").unwrap(),"Game resumed".blue()),
            GameStateEvent::GameEnded   => println!("{} {}",emojis::get_by_shortcode("stop_button").unwrap(),"Game ended".red()),
        }
    }

}
