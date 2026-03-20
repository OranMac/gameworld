use crate::entities::entity::EmotionTarget;
use crate::systems::movement::MovementStrategy;
use crate::systems::combat::CombatStrategy;

pub trait EntityFactory {
    fn create_emotional_weakness(&self) -> EmotionTarget;
    fn create_movement(&self) -> Box<dyn MovementStrategy>;
    fn create_combat(&self) -> Box<dyn CombatStrategy>;
}
