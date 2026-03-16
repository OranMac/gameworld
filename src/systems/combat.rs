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
    fn calculate_attack_damage(&self, oponent: &Entity, attack_power: f32) -> f32 {
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
