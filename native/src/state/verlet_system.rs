use rapier2d::{na::Vector2, prelude::Aabb};

use crate::{
    grid::SpatialGrid,
    thread::{get_logical_core_count, ThreadPool},
    verlet::Body,
};

use super::transform_manager::TransformManager;

pub struct VerletSystem {
    pub sub_steps: u8,
    pub screen_size: Vector2<f32>,
    pub collision_damping: f32, // how much of the velocity is lost on collision

    bodies: Vec<Body>,

    gravity: Vector2<f32>,

    biggest_radius: f32,
    grid: SpatialGrid,
    // threadpool: ThreadPool,
}

impl VerletSystem {
    pub fn new() -> Self {
        Self {
            sub_steps: 8,
            screen_size: Vector2::new(0.0, 0.0),
            collision_damping: 0.5,
            bodies: Vec::new(),
            gravity: Vector2::new(0.0, 32.0 * 20.0),
            biggest_radius: 0.0,
            grid: SpatialGrid::new(0, 0, 0.0),
            // threadpool: ThreadPool::new(get_logical_core_count()),
        }
    }

    pub fn screen_size(&mut self, width: f32, height: f32) {
        if width == self.screen_size.x && height == self.screen_size.y {
            return;
        }

        self.screen_size = Vector2::new(width, height);
        let biggest_radius = if self.biggest_radius == 0.0 {
            100.0
        } else {
            self.biggest_radius
        };
        self.grid = SpatialGrid::new(
            (width / biggest_radius) as u32,
            (height / biggest_radius) as u32,
            biggest_radius * 2.0,
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
        // Skip until the threadpool is ready
        // if self.threadpool.initialized() == false {
        //     return;
        // }

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
            let atoms = self
                .grid
                .get(index)
                .expect("Cell should exist")
                .atoms()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for atom in atoms {
                let neightbours = self.grid.get_neighbours(index as usize);
                for neighbour in neightbours {
                    Self::solve_atom_cell(atom, neighbour, &self.grid, &mut self.bodies);
                }
            }
        }
    }

    pub fn solve_atom_cell(atom: usize, cell: usize, grid: &SpatialGrid, bodies: &mut [Body]) {
        let atoms = grid
            .get(cell)
            .expect("Cell should exist")
            .atoms()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for other_atom in atoms {
            Self::solve_contact(atom, other_atom, bodies);
        }
    }

    pub fn solve_contact(a: usize, b: usize, bodies: &mut [Body]) {
        if a == b {
            return;
        }

        let (body_a, body_b) = Self::bodies_get_a_and_b_mut(bodies, a, b);
        let distance_vec = body_a.position - body_b.position;
        let distance_squared = distance_vec.magnitude_squared();
        let radius_sum = body_a.radius + body_b.radius;
        let radius_sum_squared = radius_sum * radius_sum;

        if distance_squared > f32::EPSILON && distance_squared < radius_sum_squared {
            let distance = distance_squared.sqrt();
            let delta = 0.5 * (radius_sum - distance);
            let collision_vector = (distance_vec / distance) * delta;
            body_a.position += collision_vector;
            body_b.position -= collision_vector;
        } else if distance_squared == 0.0 {
            body_a.position += Vector2::new(0.0, 0.1);
        }
    }

    pub fn bodies_get_a_and_b_mut(
        bodies: &mut [Body],
        a: usize,
        b: usize,
    ) -> (&mut Body, &mut Body) {
        assert!(a != b, "Indices must be different");
        let (min_idx, max_idx) = if a < b { (a, b) } else { (b, a) };
        let (first, second) = bodies.split_at_mut(max_idx);
        (&mut first[min_idx], &mut second[0])
    }

    pub fn update_bodies(&mut self, delta_time: f64) {
        let delta_time_squared = (delta_time * delta_time) as f32;
        let gravity = self.gravity;

        for body in &mut self.bodies {
            // Apply gravity
            body.acceleration += gravity;

            // Apply verlet integration
            let velocity = body.position - body.old_position;
            let new_position = body.position
                + velocity
                + (body.acceleration - velocity * 20.0) * delta_time_squared;
            body.old_position = body.position;
            body.position = new_position;
            body.acceleration = Vector2::new(0.0, 0.0);

            // Apply map constraints
            let radius = body.radius;
            if body.position.x < radius {
                body.position.x = radius;
            }
            if body.position.x > self.screen_size.x - radius {
                body.position.x = self.screen_size.x - radius;
            }
            if body.position.y < radius {
                body.position.y = radius;
            }
            if body.position.y > self.screen_size.y - radius {
                body.position.y = self.screen_size.y - radius;
            }
        }
    }

    pub fn apply_to_transforms(&self, transforms: &mut TransformManager) {
        for (body, transform) in self.bodies.iter().zip(transforms.iter_mut()) {
            transform.set_position(body.position.into());
        }
    }
}
