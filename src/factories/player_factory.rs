use crate::factories::entity_factory::EntityFactory;
use crate::systems::movement::{PatrolMovement, MovementStrategy};
use crate::systems::combat::{AggressiveCombat, CombatStrategy};

pub struct PlayerFactory;

impl EntityFactory for PlayerFactory {
    fn create_movement(&self) -> Box<dyn MovementStrategy> {
        Box::new(PatrolMovement::new(vec![(0.0,0.0),(5.0,0.0),(5.0,5.0)]))
    }

    fn create_combat(&self) -> Box<dyn CombatStrategy> {
        Box::new(AggressiveCombat::new(1.2))
    }
}
