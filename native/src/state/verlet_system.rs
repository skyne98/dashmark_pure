use rapier2d::{na::Vector2, prelude::Aabb};

use crate::{grid::SpatialGrid, verlet::Body};

use super::transform_manager::TransformManager;

pub struct VerletSystem {
    pub sub_steps: u8,
    pub screen_size: Vector2<f32>,
    pub collision_damping: f32, // how much of the velocity is lost on collision

    bodies: Vec<Body>,
    rng: fastrand::Rng,

    gravity: Vector2<f32>,
    sleep_threshold: f32,

    biggest_radius: f32,
    grid: SpatialGrid,
}

impl VerletSystem {
    pub fn new() -> Self {
        Self {
            sub_steps: 8,
            screen_size: Vector2::new(0.0, 0.0),
            collision_damping: 0.5,
            bodies: Vec::new(),
            gravity: Vector2::new(0.0, 32.0 * 20.0),
            sleep_threshold: 0.001,
            rng: fastrand::Rng::with_seed(404),
            biggest_radius: 0.0,
            grid: SpatialGrid::new(0, 0, 0.0),
        }
    }

    pub fn screen_size(&mut self, width: f32, height: f32) {
        self.screen_size = Vector2::new(width, height);
        self.grid = SpatialGrid::new(
            (width / self.biggest_radius) as u32,
            (height / self.biggest_radius) as u32,
            self.biggest_radius * 2.0,
        );
    }

    pub fn new_body(&mut self, position: Vector2<f32>, radius: f32) {
        let id = self.bodies.len();
        let mut body = Body::new(id);
        body.position = position;
        body.old_position = position;
        body.radius = radius;
        if radius > self.biggest_radius {
            self.biggest_radius = radius;
            self.grid = SpatialGrid::new(
                (self.screen_size.x / self.biggest_radius) as u32,
                (self.screen_size.y / self.biggest_radius) as u32,
                self.biggest_radius * 2.0,
            );
        }
        self.bodies.push(body);
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn get_bodies(&self) -> &[Body] {
        &self.bodies
    }

    pub fn get_aabbs(&self) -> Vec<Aabb> {
        self.bodies.iter().map(|b| b.aabb()).collect()
    }

    pub fn body(&self, index: usize) -> Option<&Body> {
        self.bodies.get(index)
    }

    pub fn body_mut(&mut self, index: usize) -> Option<&mut Body> {
        self.bodies.get_mut(index)
    }

    pub fn simulate(&mut self, dt: f64) {
        let sub_dt = dt / self.sub_steps as f64;
        for _ in 0..self.sub_steps {
            self.grid.clear();
            for body in &mut self.bodies {
                self.grid
                    .add_atom_world(body.id, body.position.x, body.position.y);
            }
            self.solve_collisions();
            self.update_bodies(sub_dt);
        }
    }

    pub fn solve_collisions(&mut self) {
        for index in 0..self.grid.len() {
            let atoms = self.grid.get(index).unwrap().atoms.clone();
            for atom in atoms {
                let neightbours = self.grid.get_neighbours(index);
                for neighbour in neightbours {
                    self.solve_atom_cell(atom, neighbour);
                }
            }
        }
    }

    pub fn solve_atom_cell(&mut self, atom: usize, cell: usize) {
        let atoms = self
            .grid
            .get(cell)
            .expect("Cell should exist")
            .atoms
            .clone();
        for other_atom in atoms {
            self.solve_contact(atom, other_atom);
        }
    }

    pub fn solve_contact(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        let (body_a, body_b) = self.bodies_get_a_and_b_mut(a, b);
        let distance_vec = body_a.position - body_b.position;
        let distance_squared = distance_vec.magnitude_squared();
        let radius = body_a.radius + body_b.radius;

        if distance_squared < radius * radius && distance_squared > f32::EPSILON {
            let distance = distance_squared.sqrt();
            let delta = 0.5 * (radius - distance);
            let collision_vector = (distance_vec / distance) * delta;
            body_a.position += collision_vector;
            body_b.position -= collision_vector;
        } else if distance_squared == 0.0 {
            body_a.position += Vector2::new(0.0, 0.1);
        }
    }

    pub fn bodies_get_a_and_b_mut(&mut self, a: usize, b: usize) -> (&mut Body, &mut Body) {
        unsafe {
            let bodies = self.bodies.as_mut_ptr();
            let body_a = bodies.add(a).as_mut().unwrap();
            let body_b = bodies.add(b).as_mut().unwrap();
            (body_a, body_b)
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
                + (body.acceleration - velocity * 20.0) * (delta_time * delta_time) as f32;
            body.old_position = body.position;
            body.position = new_position;
            body.acceleration = Vector2::new(0.0, 0.0);

            // Apply map constraints
            if body.position.y > self.screen_size.y - body.radius {
                body.position.y = self.screen_size.y - body.radius;
                // bounce off with losing some energy
                body.old_position.y = body.position.y + velocity.y * body.ground_friction;
            } else if body.position.y < body.radius {
                body.position.y = body.radius;
                // bounce off with losing some energy
                body.old_position.y = body.position.y + velocity.y * body.ground_friction;
            }
            if body.position.x > self.screen_size.x - body.radius {
                body.position.x = self.screen_size.x - body.radius;
                // bounce off with losing some energy
                body.old_position.x = body.position.x + velocity.x * body.ground_friction;
            } else if body.position.x < body.radius {
                body.position.x = body.radius;
                // bounce off with losing some energy
                body.old_position.x = body.position.x + velocity.x * body.ground_friction;
            }
        }
    }

    pub fn apply_to_transforms(&self, transforms: &mut TransformManager) {
        for (body, transform) in self.bodies.iter().zip(transforms.iter_mut()) {
            transform.set_position(body.position.into());
        }
    }
}
