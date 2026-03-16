use crate::factories::entity_factory::EntityFactory;
use crate::systems::movement::{RandomMovement, MovementStrategy};
use crate::systems::combat::{AggressiveCombat, CombatStrategy};

pub struct EnemyFactory;

impl EntityFactory for EnemyFactory {
    fn create_movement(&self) -> Box<dyn MovementStrategy> {
        Box::new(RandomMovement::new(2.0))
    }

    fn create_combat(&self) -> Box<dyn CombatStrategy> {
        Box::new(AggressiveCombat::new(1.5))
    }
}
