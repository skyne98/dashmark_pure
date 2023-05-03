use std::collections::HashMap;

use flat_spatial::{aabbgrid::AABBGridHandle, AABBGrid};
use generational_arena::Index;
use rapier2d::na::Vector2;

use crate::verlet::{Body, BodyAabb};

use super::transform_manager::TransformManager;

pub struct VerletSystem {
    pub sub_steps: u8,
    pub screen_size: Vector2<f32>,
    pub collision_damping: f32, // how much of the velocity is lost on collision

    bodies: Vec<Body>,
    gravity: Vector2<f32>,
    grid: AABBGrid<usize, BodyAabb>,
    grid_handles: HashMap<usize, AABBGridHandle>,
}

impl VerletSystem {
    pub fn new() -> Self {
        Self {
            sub_steps: 4,
            screen_size: Vector2::new(0.0, 0.0),
            collision_damping: 0.5,
            bodies: Vec::new(),
            gravity: Vector2::new(0.0, 100.0),
            grid: AABBGrid::new(48),
            grid_handles: HashMap::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        let handle = self.grid.insert((&body).into(), self.bodies.len());
        self.grid_handles.insert(self.bodies.len(), handle);
        self.bodies.push(body);
    }

    pub fn initialize_body(&mut self, index: usize, position: Vector2<f32>) {
        let body = &mut self.bodies[index];
        if body.initialized == false {
            body.position = position;
            let handle = self.grid_handles[&index];
            self.grid.set_aabb(handle, body.into());
            body.initialized = true;
        }
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

    pub fn simulate(&mut self, dt: f64) {
        let dt = dt * 2.0;
        let sub_dt = dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.solve_collisions();
            self.update_bodies(sub_dt);
        }
    }

    pub fn update_bodies(&mut self, delta_time: f64) {
        let length = self.bodies.len();
        for body_i in 0..length {
            let body = &mut self.bodies[body_i];
            // Apply gravity
            body.acceleration += self.gravity;
            // Apply verlet integration
            let velocity = body.position - body.old_position;
            let new_position = body.position
                + velocity
                + (body.acceleration - velocity * body.friction) * (delta_time * delta_time) as f32;
            body.old_position = body.position;
            body.position = new_position;
            body.acceleration = Vector2::new(0.0, 0.0);
            // Apply map constraints
            if body.position.y > self.screen_size.y - body.radius {
                body.position.y = self.screen_size.y - body.radius;
            }
            if body.position.y < body.radius {
                body.position.y = body.radius;
            }
            if body.position.x > self.screen_size.x - body.radius {
                body.position.x = self.screen_size.x - body.radius;
            }
            if body.position.x < body.radius {
                body.position.x = body.radius;
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        for index in 0..self.bodies.len() {
            // Query the grid for nearby bodies
            let (before, including_after) = self.bodies.split_at_mut(index);
            let (body, after) = including_after.split_first_mut().unwrap();
            let collided_ids = self
                .grid
                .query(body.into())
                .map(|(_, _, index)| *index)
                .collect::<Vec<_>>();
            for other_index in collided_ids {
                if index == other_index {
                    continue;
                }
                let other = if other_index < index {
                    &mut before[other_index]
                } else {
                    &mut after[other_index - index - 1]
                };

                let distance_vec = body.position - other.position;
                let distance_squared = distance_vec.magnitude_squared();
                let radius = body.radius + other.radius;

                if distance_squared < radius * radius && distance_squared > f32::EPSILON {
                    let distance = distance_squared.sqrt();
                    let delta = 0.5 * (radius - distance);
                    let collision_vector =
                        (distance_vec / distance) * delta * self.collision_damping;
                    body.position += collision_vector;
                    other.position -= collision_vector;
                } else if distance_squared == 0.0 {
                    // // Resolve bodies that are on top of each other
                    body.position += Vector2::new(0.0, 1.0);
                    other.position -= Vector2::new(0.0, 1.0);
                }
            }
        }

        // Update the aabbs in the grid
        let len = self.bodies.len();
        for index in 0..len {
            let body = &self.bodies[index];
            let handle = self.grid_handles[&index];
            self.grid.set_aabb(handle, body.into());
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
