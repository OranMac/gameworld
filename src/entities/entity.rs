use crate::systems::movement::MovementStrategy;
use crate::systems::combat::CombatStrategy;
use crate::factories::entity_factory::EntityFactory;

pub struct Entity {
    pub name: String,
    pub position: (f32, f32),
    pub movement: Box<dyn MovementStrategy>,
    pub combat: Box<dyn CombatStrategy>,
}

impl Entity {
    // Create entity via Abstract (Entity)Factory
    pub fn new(name: &str, factory: &dyn EntityFactory) -> Self {
        Self {
            name: name.to_string(),
            position: (0.0,0.0),
            movement: factory.create_movement(),
            combat: factory.create_combat(),
        }
    }

    // Create entity via menu-selected strategies
    pub fn from_strategies(name: &str, movement: Box<dyn MovementStrategy>, combat: Box<dyn CombatStrategy>) -> Self {
        Self {
            name: name.to_string(),
            position: (0.0,0.0),
            movement,
            combat,
        }
    }

    pub fn update_position(&mut self) {
        self.position = self.movement.compute_next_waypoint(self.position);
    }

    pub fn attack(&self, attack_power: f32) -> f32 {
        self.combat.calculate_attack_damage(attack_power)
    }

    pub fn defend(&self, damage: f32) -> f32 {
        self.combat.calculate_damage_taken(damage)
    }
}
