pub trait CombatStrategy {
    // Compute attack damage
    fn calculate_attack_damage(&self, attack_power: f32) -> f32;
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
    fn calculate_attack_damage(&self, attack_power: f32) -> f32 {
        attack_power * self.damage_multiplier
    }

    // Handle defense: aggressive strategy does not reduce incoming damage
    fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
        incoming_damage
    }
}


// Future Idea for enum bases emotional damage, but no target yet to really implement it
// Emotion target is public due to enums being private by default
// pub enum EmotionTarget {
//     Pride,
//     Ego,
//     Worth,
// }

// pub struct EmotionalDamageCombat {
//     emotional_damage: EmotionTarget
// }

// impl EmotionalDamageCombat {
//     pub fn new(target: EmotionTarget) -> Self {
//         Self {
//             // Select the which part of ones person to target
//             emotional_damage: target,
//         }
//     }
// }

// impl CombatStrategy for EmotionalDamageCombat {
//     // Calculate attack damage: multiply attack_power by damage_multiplier
//     fn calculate_attack_damage(&self, attack_power: f32) -> f32 {
//         5.0
//     }

//     // Handle defense: aggressive strategy does not reduce incoming damage
//     fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
//         incoming_damage
//     }
// }

pub struct PrideCombat{
    emotion_multiplier: f32,
}

impl PrideCombat {
    pub fn new(multiplier: f32) -> Self {
        Self {
            // Multiplier applied to attack_power
            emotion_multiplier: multiplier/2 as f32,
        }
    }
}

impl CombatStrategy for PrideCombat {
    fn calculate_attack_damage(&self, attack_power: f32) -> f32 {
        attack_power * self.emotion_multiplier
    }

    fn calculate_damage_taken(&self, incoming_damage: f32) -> f32 {
        f32::abs(incoming_damage - incoming_damage*self.emotion_multiplier)
    }
}