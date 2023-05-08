use std::ops::Deref;

use rapier2d::{na::Vector2, prelude::Aabb};

pub trait IntoAabb {
    fn into_aabb(&self) -> Aabb;
}
impl<T: IntoAabb> IntoAabb for &T {
    fn into_aabb(&self) -> Aabb {
        (*self).into_aabb()
    }
}
impl<T: IntoAabb> IntoAabb for &mut T {
    fn into_aabb(&self) -> Aabb {
        (*self).into_aabb()
    }
}

#[derive(Clone, Debug)]
pub struct Body {
    pub id: usize,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub forces: Vector2<f32>,
    pub friction: f32,        // 0.97
    pub ground_friction: f32, // 0.4
    pub radius: f32,
    pub mass: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vector2::new(0.0, 0.0),
            old_position: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            forces: Vector2::new(0.0, 0.0),
            friction: 0.97,
            ground_friction: 0.4,
            radius: 2.0,
            mass: 1.0,
        }
    }
}

impl Body {
    pub fn new(id: usize) -> Self {
        let mut body = Self::default();
        body.id = id;
        body
    }

    pub fn aabb(&self) -> Aabb {
        Aabb::new(
            Vector2::new(self.position.x - self.radius, self.position.y - self.radius).into(),
            Vector2::new(self.position.x + self.radius, self.position.y + self.radius).into(),
        )
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
        self.old_position = position;
    }

    pub fn set_position_keep_movement(&mut self, position: Vector2<f32>) {
        let velocity = self.position - self.old_position;
        self.position = position;
        self.old_position = position - velocity;
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.old_position = self.position - velocity;
    }
}

impl IntoAabb for Body {
    fn into_aabb(&self) -> Aabb {
        self.aabb()
    }
}
