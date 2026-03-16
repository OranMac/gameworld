use crate::systems::movement::MovementStrategy;
use crate::systems::combat::CombatStrategy;
use crate::factories::entity_factory::EntityFactory;

// Emotion target is public due to enums being private by default
#[derive(PartialEq)]
pub enum EmotionTarget {
    Pride,
    Ego,
    Greed,
}

pub struct Entity {
    pub name: String,
    pub position: (f32, f32),
    pub emo_weakness: EmotionTarget,
    pub movement: Box<dyn MovementStrategy>,
    pub combat: Box<dyn CombatStrategy>,
}

impl Entity {
    // Create entity via Abstract (Entity)Factory
    pub fn new(name: &str, emo_target: EmotionTarget, factory: &dyn EntityFactory) -> Self {
        Self {
            name: name.to_string(),
            position: (0.0,0.0),
            emo_weakness: emo_target,
            movement: factory.create_movement(),
            combat: factory.create_combat(),
        }
    }

    // Create entity via menu-selected strategies
    pub fn from_strategies(name: &str, emo_target: EmotionTarget, movement: Box<dyn MovementStrategy>, combat: Box<dyn CombatStrategy>) -> Self {
        Self {
            name: name.to_string(),
            position: (0.0,0.0),
            emo_weakness: emo_target,
            movement,
            combat,
        }
    }

    pub fn update_position(&mut self) {
        self.position = self.movement.compute_next_waypoint(self.position);
    }

    pub fn attack(&self,oponent: &Entity, attack_power: f32) -> f32 {
        self.combat.calculate_attack_damage(oponent,attack_power)
    }

    pub fn defend(&self, damage: f32) -> f32 {
        self.combat.calculate_damage_taken(damage)
    }
}
