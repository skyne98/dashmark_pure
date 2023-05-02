use rapier2d::na::Vector2;

pub struct Body {
    pub initialized: bool,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub friction: f32,        // 0.97
    pub ground_friction: f32, // 0.7
    pub radius: f32,
    pub mass: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            initialized: false,
            position: Vector2::new(0.0, 0.0),
            old_position: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            friction: 0.97,
            ground_friction: 0.7,
            radius: 8.0,
            mass: 1.0,
        }
    }
}
