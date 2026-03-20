// Defines the contract for movement behaviours.
// Any struct implementing this trait, must define the two functions.
pub trait MovementStrategy {
    fn compute_next_waypoint(&mut self, current_waypoint: (f32, f32)) -> (f32, f32);
    fn update(&mut self);
}

// This struct stores waypoints for patrolling behavior.
// waypoints: a list of (x, y) coordinates to patrol between.
// next_waypoint_index: the index of the waypoint that will be returned next
pub struct PatrolMovement {
    waypoints: Vec<(f32, f32)>,
    next_waypoint_index: usize,
}

impl PatrolMovement {
    pub fn new(waypoints: Vec<(f32, f32)>) -> Self {
        Self {
            waypoints,
            next_waypoint_index: 0,
        }
    }
}

impl MovementStrategy for PatrolMovement {

    // Checks if there are any waypoints; if not, stay in place.
    // Otherwise, get the next target waypoint (looping back to the start when 
    // reaching the end) and return it.  
    fn compute_next_waypoint(&mut self, current_waypoint: (f32, f32)) -> (f32, f32) {
        if self.waypoints.is_empty() {
            return current_waypoint;
        }

        let target = self.waypoints[self.next_waypoint_index];
        self.next_waypoint_index = (self.next_waypoint_index + 1) % self.waypoints.len();
        target
    }
    
    fn update(&mut self) {
        // placeholder for future logic (e.g., timing, animations)
    }
}
use rand::Rng;
pub struct RandomMovement {
    range: f32,
}

impl RandomMovement {
    pub fn new(range: f32) -> Self {
        Self { range }
    }
}

impl MovementStrategy for RandomMovement {
    fn compute_next_waypoint(&mut self, current_waypoint: (f32, f32)) -> (f32, f32) {
        let mut rng =  rand::thread_rng();
        let dx = rng.gen_range(-self.range..=self.range);
        let dy = rng.gen_range(-self.range..=self.range);
        (current_waypoint.0 + dx, current_waypoint.1 + dy)
    }

    fn update(&mut self) {
        // Update random logic
    }
}

pub struct ScaredMovement {
    speed: f32,
    point_of_fear: (f32,f32,),
}

impl ScaredMovement {
    pub fn new(speed:f32, point_of_fear:(f32,f32)) -> Self {
        Self {
            speed,
            point_of_fear,
        }
    }
}

impl MovementStrategy for ScaredMovement {
    fn compute_next_waypoint(&mut self,current_waypoint: (f32, f32)) -> (f32, f32) {
        let mut new_position: (f32,f32) = (0.0,0.0);
        if current_waypoint.0 >= self.point_of_fear.0{
            new_position.0 = current_waypoint.0 + self.speed;
        } else {
            new_position.0 = current_waypoint.0 - self.speed;
        }
        if current_waypoint.1 >= self.point_of_fear.1{
            new_position.1 = current_waypoint.1 + self.speed;
        } else {
            new_position.1 = current_waypoint.1 - self.speed;
        }
    
    
        new_position
    }

    fn update(&mut self) {
        // Update random logic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- PatrolMovement Tests ----------

    #[test]
    fn patrol_returns_waypoints_in_order() {
        let mut patrol = PatrolMovement::new(vec![(1.0, 1.0), (2.0, 2.0)]);

        let first = patrol.compute_next_waypoint((0.0, 0.0));
        let second = patrol.compute_next_waypoint((0.0, 0.0));

        assert_eq!(first, (1.0, 1.0));
        assert_eq!(second, (2.0, 2.0));
    }

    #[test]
    fn patrol_loops_back_to_start() {
        let mut patrol = PatrolMovement::new(vec![(1.0, 1.0)]);

        let first = patrol.compute_next_waypoint((0.0, 0.0));
        let second = patrol.compute_next_waypoint((0.0, 0.0));

        assert_eq!(first, (1.0, 1.0));
        assert_eq!(second, (1.0, 1.0)); // loops
    }

    #[test]
    fn patrol_empty_returns_current_position() {
        let mut patrol = PatrolMovement::new(vec![]);

        let current = (5.0, 5.0);
        let result = patrol.compute_next_waypoint(current);

        assert_eq!(result, current);
    }

    // ---------- RandomMovement Tests ----------

    #[test]
    fn random_movement_stays_within_range() {
        let mut movement = RandomMovement::new(5.0);
        let current = (10.0, 10.0);

        let result = movement.compute_next_waypoint(current);

        let dx = result.0 - current.0;
        let dy = result.1 - current.1;

        assert!(dx >= -5.0 && dx <= 5.0);
        assert!(dy >= -5.0 && dy <= 5.0);
    }

    // ---------- ScaredMovement Tests ----------

    #[test]
    fn scared_moves_away_from_fear_point_positive_direction() {
        let mut movement = ScaredMovement::new(1.0, (0.0, 0.0));
        let current = (5.0, 5.0);

        let result = movement.compute_next_waypoint(current);

        assert_eq!(result, (6.0, 6.0));
    }

    #[test]
    fn scared_moves_away_from_fear_point_negative_direction() {
        let mut movement = ScaredMovement::new(1.0, (10.0, 10.0));
        let current = (5.0, 5.0);

        let result = movement.compute_next_waypoint(current);

        assert_eq!(result, (4.0, 4.0));
    }

    #[test]
    fn scared_moves_correctly_mixed_axes() {
        let mut movement = ScaredMovement::new(2.0, (5.0, 0.0));
        let current = (3.0, 3.0);

        let result = movement.compute_next_waypoint(current);

        // x < fear.x → move left, y > fear.y → move up
        assert_eq!(result, (1.0, 5.0));
    }
}