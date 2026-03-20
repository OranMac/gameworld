use crate::systems::movement::{MovementStrategy, ScaredMovement};
use crate::systems::combat::{CombatStrategy, EmotionalDamageCombat};
use crate::factories::entity_factory::EntityFactory;

// Emotion target is public due to enums being private by default
#[derive(PartialEq,Debug)]
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

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: String::default(),
            position: (0.0, 0.0),
            emo_weakness: EmotionTarget::Pride,
            movement: Box::new(ScaredMovement::new(0.0,(0.0,0.0))),
            combat: Box::new(EmotionalDamageCombat::new(EmotionTarget::Pride)),
        }
    }
}

use std::fmt;

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Position: {},{}, Emotion: {:?}", 
        self.name, self.position.0,self.position.1,self.emo_weakness)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::movement::{MovementStrategy};
    use crate::systems::combat::{CombatStrategy};

    // ---------- Mock Strategies ----------

    struct MockMovement;

    impl MovementStrategy for MockMovement {
        fn compute_next_waypoint(&mut self, _current: (f32, f32)) -> (f32, f32) {
            (10.0, 10.0)
        }

        fn update(&mut self) {}
    }

    struct MockCombat;

    impl CombatStrategy for MockCombat {
        fn calculate_attack_damage(&self, _opponent: &Entity, attack_power: f32) -> f32 {
            attack_power + 5.0
        }

        fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
            incoming_damage - 2.0
        }
    }

    // ---------- Constructor Tests ----------

    #[test]
    fn entity_from_strategies_sets_fields_correctly() {
        let entity = Entity::from_strategies(
            "Hero",
            EmotionTarget::Ego,
            Box::new(MockMovement),
            Box::new(MockCombat),
        );

        assert_eq!(entity.name, "Hero");
        assert_eq!(entity.position, (0.0, 0.0));
        assert_eq!(entity.emo_weakness, EmotionTarget::Ego);
    }

    #[test]
    fn default_entity_has_expected_values() {
        let entity = Entity::default();

        assert_eq!(entity.name, "");
        assert_eq!(entity.position, (0.0, 0.0));
        assert_eq!(entity.emo_weakness, EmotionTarget::Pride);
    }

    // ---------- Movement Tests ----------

    #[test]
    fn update_position_uses_movement_strategy() {
        let mut entity = Entity::from_strategies(
            "Mover",
            EmotionTarget::Pride,
            Box::new(MockMovement),
            Box::new(MockCombat),
        );

        entity.update_position();

        assert_eq!(entity.position, (10.0, 10.0));
    }

    // ---------- Combat Tests ----------

    #[test]
    fn attack_uses_combat_strategy() {
        let attacker = Entity::from_strategies(
            "Attacker",
            EmotionTarget::Pride,
            Box::new(MockMovement),
            Box::new(MockCombat),
        );

        let defender = Entity::default();

        let damage = attacker.attack(&defender, 10.0);

        assert_eq!(damage, 15.0); // 10 + 5 from mock
    }

    #[test]
    fn defend_uses_combat_strategy() {
        let entity = Entity::from_strategies(
            "Defender",
            EmotionTarget::Pride,
            Box::new(MockMovement),
            Box::new(MockCombat),
        );

        let damage_taken = entity.defend(10.0);

        assert_eq!(damage_taken, 8.0); // 10 - 2 from mock
    }

    // ---------- Display टेस्ट ----------

    #[test]
    fn display_formats_correctly() {
        let entity = Entity::from_strategies(
            "DisplayTest",
            EmotionTarget::Greed,
            Box::new(MockMovement),
            Box::new(MockCombat),
        );

        let output = format!("{}", entity);

        assert!(output.contains("DisplayTest"));
        assert!(output.contains("0"));
        assert!(output.contains("Greed"));
    }
}