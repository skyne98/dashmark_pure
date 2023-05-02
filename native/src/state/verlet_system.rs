use generational_arena::Index;
use rapier2d::na::Vector2;

use crate::verlet::Body;

use super::transform_manager::TransformManager;

pub struct VerletSystem {
    bodies: Vec<Body>,
    gravity: Vector2<f32>,
}

impl VerletSystem {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vector2::new(0.0, 100.0),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn get_bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn body(&self, index: Index) -> Option<&Body> {
        let index = index.into_raw_parts().0;
        self.bodies.get(index)
    }

    pub fn body_mut(&mut self, index: Index) -> Option<&mut Body> {
        let index = index.into_raw_parts().0;
        self.bodies.get_mut(index)
    }

    pub fn apply_basic_physics(&mut self, delta_time: f64) {
        for body in &mut self.bodies {
            body.acceleration += self.gravity;
            let velocity = body.position - body.old_position;
            let new_position = body.position
                + velocity
                + (body.acceleration - velocity * body.friction) * (delta_time * delta_time) as f32;
            body.old_position = body.position;
            body.position = new_position;
            body.acceleration = Vector2::new(0.0, 0.0);
        }
    }

    pub fn apply_interaction_physics_substep(
        &mut self,
        delta_time: f64,
        screen_width: f32,
        screen_height: f32,
    ) {
        let steps = 10;
        let substep_delta_time = delta_time / steps as f64;

        for _ in 0..steps {
            self.apply_interaction_physics(substep_delta_time);
            self.apply_constraints(screen_width, screen_height);
        }
    }

    pub fn apply_interaction_physics(&mut self, delta_time: f64) {
        for index in 0..self.bodies.len() {
            // Unsafely split into mutable `before`, `this` and `after` slices
            let (before, this_and_after) = self.bodies.split_at_mut(index);
            let (this, after) = this_and_after.split_at_mut(1);
            let this = &mut this[0];

            for other in before.iter_mut().chain(after.iter_mut()) {
                let distance_vec = this.position - other.position;
                let distance_squared = distance_vec.magnitude_squared();
                let radius = this.radius + other.radius;

                if distance_squared < radius * radius && distance_squared > f32::EPSILON {
                    let distance = distance_squared.sqrt();
                    let delta = 0.5 * (radius - distance);
                    let collision_vector = (distance_vec / distance) * delta;
                    this.position += collision_vector;
                    other.position -= collision_vector;
                } else if distance_squared == 0.0 {
                    // Resolve bodies that are on top of each other
                    this.position += Vector2::new(0.0, 1.0);
                    other.position -= Vector2::new(0.0, 1.0);
                }
            }
        }
    }

    pub fn apply_constraints(&mut self, screen_width: f32, screen_height: f32) {
        for body in &mut self.bodies {
            if body.position.y > screen_height - body.radius {
                body.position.y = screen_height - body.radius;
            }
            if body.position.y < body.radius {
                body.position.y = body.radius;
            }
            if body.position.x > screen_width - body.radius {
                body.position.x = screen_width - body.radius;
            }
            if body.position.x < body.radius {
                body.position.x = body.radius;
            }
        }
    }

    pub fn apply_to_transforms(&self, transforms: &mut TransformManager) {
        for (body, transform) in self.bodies.iter().zip(transforms.iter_mut()) {
            if body.initialized {
                transform.set_position(body.position.into());
            }
        }
    }
}
