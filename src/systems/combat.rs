use crate::entities::entity::Entity;
use crate::EmotionTarget;

pub trait CombatStrategy {
    // Compute attack damage
    fn calculate_attack_damage(&self, oponent: &Entity, attack_power: f32) -> f32;
    // Compute damage taken after defense
    fn calculate_damage_taken(&self, incoming_damage: f32) -> f32;
}

// Aggressive combat strategy: prioritises offense, ignores defense
pub struct AggressiveCombat {
    damage_multiplier: f32,
}

impl AggressiveCombat {
    pub fn new(multiplier: f32) -> Self {
        Self {
            // Multiplier applied to attack_power
            damage_multiplier: multiplier,
        }
    }
}

impl CombatStrategy for AggressiveCombat {
    // Calculate attack damage: multiply attack_power by damage_multiplier
    fn calculate_attack_damage(&self, _oponent: &Entity, attack_power: f32) -> f32 {
        attack_power * self.damage_multiplier
    }

    // Handle defense: aggressive strategy does not reduce incoming damage
    fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
        incoming_damage
    }
}


// Future Idea for enum bases emotional damage, but no target yet to really implement it

#[derive(PartialEq)]
pub struct EmotionalDamageCombat {
    emotional_damage: EmotionTarget
}

impl EmotionalDamageCombat {
    pub fn new(target: EmotionTarget) -> Self {
        Self {
            // Select the which part of ones person to target
            emotional_damage: target,
        }
    }
}

impl CombatStrategy for EmotionalDamageCombat {
    // Calculate attack damage: multiply attack_power by damage_multiplier
    fn calculate_attack_damage(&self, oponent: &Entity, attack_power: f32) -> f32 {
        attack_power * if self.emotional_damage == oponent.emo_weakness { 2.0 } else { 0.5 }
    }

    // Handle defense: aggressive strategy does not reduce incoming damage
    fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
        incoming_damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // You need a minimal mock Entity for testing
    fn mock_entity(weakness: EmotionTarget) -> Entity {
        let mut entity = Entity::default();
        entity.emo_weakness = weakness;
        entity
    }

    // ---------- AggressiveCombat Tests ----------

    #[test]
    fn aggressive_attack_applies_multiplier() {
        let strategy = AggressiveCombat::new(2.0);
        let opponent = mock_entity(EmotionTarget::Ego);

        let damage = strategy.calculate_attack_damage(&opponent, 10.0);

        assert_eq!(damage, 20.0);
    }

    #[test]
    fn aggressive_damage_taken_is_unchanged() {
        let strategy = AggressiveCombat::new(3.0);

        let damage_taken = strategy.calculate_damage_taken(15.0);

        assert_eq!(damage_taken, 15.0);
    }

    // ---------- EmotionalDamageCombat Tests ----------

    #[test]
    fn emotional_damage_double_if_weakness_matches() {
        let strategy = EmotionalDamageCombat::new(EmotionTarget::Pride);
        let opponent = mock_entity(EmotionTarget::Pride);

        let damage = strategy.calculate_attack_damage(&opponent, 10.0);

        assert_eq!(damage, 20.0); // 2x multiplier
    }

    #[test]
    fn emotional_damage_reduced_if_no_match() {
        let strategy = EmotionalDamageCombat::new(EmotionTarget::Pride);
        let opponent = mock_entity(EmotionTarget::Ego);

        let damage = strategy.calculate_attack_damage(&opponent, 10.0);

        assert_eq!(damage, 5.0); // 0.5x multiplier
    }

    #[test]
    fn emotional_damage_taken_is_unchanged() {
        let strategy = EmotionalDamageCombat::new(EmotionTarget::Greed);

        let damage_taken = strategy.calculate_damage_taken(12.0);

        assert_eq!(damage_taken, 12.0);
    }

    // ---------- Edge Case Tests ----------

    #[test]
    fn zero_attack_power_results_in_zero_damage() {
        let strategy = AggressiveCombat::new(5.0);
        let opponent = mock_entity(EmotionTarget::Ego);

        let damage = strategy.calculate_attack_damage(&opponent, 0.0);

        assert_eq!(damage, 0.0);
    }

    #[test]
    fn emotional_damage_handles_zero_attack_power() {
        let strategy = EmotionalDamageCombat::new(EmotionTarget::Pride);
        let opponent = mock_entity(EmotionTarget::Pride);

        let damage = strategy.calculate_attack_damage(&opponent, 0.0);

        assert_eq!(damage, 0.0);
    }
}