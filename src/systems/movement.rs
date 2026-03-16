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
