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
    rng: fastrand::Rng,
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
            rng: fastrand::Rng::with_seed(404),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        let handle = self.grid.insert((&body).into(), self.bodies.len());
        self.grid_handles.insert(self.bodies.len(), handle);
        self.bodies.push(body);
    }

    pub fn get_bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn body(&self, index: usize) -> Option<&Body> {
        self.bodies.get(index)
    }

    pub fn body_mut(&mut self, index: usize) -> Option<&mut Body> {
        self.bodies.get_mut(index)
    }

    pub fn simulate(&mut self, dt: f64) {
        // Update AABBS based on the potentially new positions
        for (index, body) in self.bodies.iter().enumerate() {
            let handle = self.grid_handles.get(&index).unwrap();
            self.grid.set_aabb(*handle, body.into());
        }

        let dt = dt * 2.0;
        let sub_dt = dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.solve_collisions();
            self.update_bodies(sub_dt);
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
                    let delta = 0.5 * (radius - distance) * 0.8;
                    let collision_vector = (distance_vec / distance) * delta;
                    let mass_sum = body.mass + other.mass;
                    body.position += (collision_vector * other.mass) / mass_sum;
                    other.position -= (collision_vector * body.mass) / mass_sum;
                    // Add a tiny random vector
                    let random_vector =
                        Vector2::new(self.rng.f32() * 0.0001, self.rng.f32() * 0.0001);
                    body.position += random_vector;
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

    pub fn update_bodies(&mut self, delta_time: f64) {
        let length = self.bodies.len();
        let mut overall_velocity = 0.0;
        for body_i in 0..length {
            let body = &mut self.bodies[body_i];
            // Apply gravity
            body.acceleration += self.gravity;
            // Apply verlet integration
            let velocity = body.position - body.old_position;
            overall_velocity += velocity.magnitude();
            let new_position = body.position
                + velocity
                + (body.acceleration - velocity * body.friction) * (delta_time * delta_time) as f32;
            body.old_position = body.position;
            body.position = new_position;
            body.acceleration = Vector2::new(0.0, 0.0);

            // Apply map constraints
            if body.position.y > self.screen_size.y - body.radius {
                let penetration = body.position.y - (self.screen_size.y - body.radius);
                let penetration = penetration * self.collision_damping;
                body.position.y -= penetration * 2.0;
            } else if body.position.y < body.radius {
                let penetration = body.radius - body.position.y;
                let penetration = penetration * self.collision_damping;
                body.position.y += penetration * 2.0;
            } else if body.position.x > self.screen_size.x - body.radius {
                let penetration = body.position.x - (self.screen_size.x - body.radius);
                let penetration = penetration * self.collision_damping;
                body.position.x -= penetration * 2.0;
            } else if body.position.x < body.radius {
                let penetration = body.radius - body.position.x;
                let penetration = penetration * self.collision_damping;
                body.position.x += penetration * 2.0;
            }
        }
        println!("Overall velocity: {}", overall_velocity);
    }

    pub fn apply_to_transforms(&self, transforms: &mut TransformManager) {
        for (body, transform) in self.bodies.iter().zip(transforms.iter_mut()) {
            transform.set_position(body.position.into());
        }
    }
}
