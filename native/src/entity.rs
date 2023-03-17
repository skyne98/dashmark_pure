use anyhow::Result;
use core::fmt::Debug;
use generational_arena::Index;
use rapier2d_f64::{
    na::{Isometry2, Point2, UnitComplex, Vector2},
    parry::query::RayCast,
    prelude::{Aabb, Shape},
};
use std::ops::Mul;

// Shape
pub trait EntityShape: RayCast + Shape + Debug {}
impl<T> EntityShape for T where T: RayCast + Shape + Debug {}

// Transform components
#[derive(Debug)]
pub enum Origin {
    Relative(Vector2<f64>),
    Absolute(Vector2<f64>),
}

#[derive(Debug)]
pub struct Entity {
    pub index: Index,

    // Transform
    origin: Origin,
    position: Vector2<f64>,
    rotation: f64,
    // ...other transform components
    transform_isometry: Isometry2<f64>,
    dirty_transforms: bool,

    // Coliisions
    pub shape: Option<Box<dyn EntityShape>>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            index: Index::from_raw_parts(usize::MAX, u64::MAX),
            origin: Origin::Absolute(Vector2::new(0.0, 0.0)),
            position: Default::default(),
            rotation: Default::default(),
            transform_isometry: Default::default(),
            dirty_transforms: true,
            shape: None,
        }
    }
}

impl Entity {
    pub fn new(index: Index) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }

    pub fn get_size(&self) -> Vector2<f64> {
        match self.get_local_aabb() {
            Some(aabb) => Vector2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y),
            None => Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_absolute_offset(&self) -> Vector2<f64> {
        let size = self.get_size();
        match self.origin {
            Origin::Relative(offset) => Vector2::new(offset.x * size.x, offset.y * size.y),
            Origin::Absolute(offset) => offset,
        }
    }

    pub fn get_aabb(&mut self) -> Option<Aabb> {
        let transform = self.get_transform_isometry();
        let shape = self.shape.as_ref();
        shape.map(|shape| shape.compute_aabb(&transform))
    }

    pub fn get_transform_isometry(&mut self) -> Isometry2<f64> {
        if self.dirty_transforms {
            self.recalculate_transforms();
        }
        self.transform_isometry
    }

    pub fn get_local_aabb(&self) -> Option<Aabb> {
        self.shape.as_ref().map(|shape| shape.compute_local_aabb())
    }

    pub fn set_origin(&mut self, origin: Origin) {
        self.origin = origin;
        self.dirty_transforms = true;
    }

    pub fn set_position(&mut self, position: Vector2<f64>) {
        self.position = position;
        self.dirty_transforms = true;
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
        self.dirty_transforms = true;
    }

    pub fn set_shape<S: EntityShape>(&mut self, shape: S) {
        self.shape = Some(Box::new(shape));
        self.dirty_transforms = true;
    }

    pub fn recalculate_transforms(&mut self) {
        let absolute_offset = self.get_absolute_offset();
        let size = self.get_size();
        let half_size = size * 0.5;
        self.transform_isometry.translation.vector = self.position - absolute_offset + half_size;
        self.transform_isometry.rotation = UnitComplex::new(0.0);
        self.transform_isometry.append_rotation_wrt_point_mut(
            &UnitComplex::new(self.rotation),
            &Point2::from(self.position),
        );
        self.dirty_transforms = false;
    }
}

// ===== Tests =====
#[cfg(test)]
mod test_entity {
    use super::*;

    #[test]
    fn entity() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_origin(Origin::Absolute(Vector2::new(0.0, 0.0)));
        entity.set_position(Vector2::new(0.0, 0.0));
        entity.set_rotation(0.0);
        let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
        entity.set_shape(shape);
        let aabb = entity.get_aabb().unwrap();
        assert_eq!(aabb.mins, Point2::new(0.0, 0.0));
        assert_eq!(aabb.maxs, Point2::new(2.0, 2.0));
    }

    #[test]
    fn origins_relative() {
        // Test all origins from 0.0 to 1.0 in 0.1 increments
        for x in 0..11 {
            for y in 0..11 {
                let x = x as f64 * 0.1;
                let y = y as f64 * 0.1;
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                entity.set_origin(Origin::Relative(Vector2::new(x, y)));
                entity.set_position(Vector2::new(0.0, 0.0));
                entity.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_aabb().unwrap();
                let expected_minx = ((-x * 2.0) * 100.0).round() as i32;
                let actual_minx = (aabb.mins.x * 100.0).round() as i32;
                assert_eq!(expected_minx, actual_minx, "x: {}, y: {}", x, y);
                let expected_miny = ((-y * 2.0) * 100.0).round() as i32;
                let actual_miny = (aabb.mins.y * 100.0).round() as i32;
                assert_eq!(expected_miny, actual_miny, "x: {}, y: {}", x, y);
                let expected_maxx = ((1.0 - x) * 2.0 * 100.0).round() as i32;
                let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
                assert_eq!(expected_maxx, actual_maxx, "x: {}, y: {}", x, y);
                let expected_maxy = ((1.0 - y) * 2.0 * 100.0).round() as i32;
                let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
                assert_eq!(expected_maxy, actual_maxy, "x: {}, y: {}", x, y);
            }
        }
    }

    #[test]
    fn origins_absolute() {
        // Test all origins from 0.0 to 2.0 in 0.1 increments
        for x in 0..21 {
            for y in 0..21 {
                let x = x as f64 * 0.1;
                let y = y as f64 * 0.1;
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                entity.set_origin(Origin::Absolute(Vector2::new(x, y)));
                entity.set_position(Vector2::new(0.0, 0.0));
                entity.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_aabb().unwrap();
                let expected_minx = (-x * 100.0).round() as i32;
                let actual_minx = (aabb.mins.x * 100.0).round() as i32;
                assert_eq!(expected_minx, actual_minx, "x: {}, y: {}", x, y);
                let expected_miny = (-y * 100.0).round() as i32;
                let actual_miny = (aabb.mins.y * 100.0).round() as i32;
                assert_eq!(expected_miny, actual_miny, "x: {}, y: {}", x, y);
                let expected_maxx = ((2.0 - x) * 100.0).round() as i32;
                let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
                assert_eq!(expected_maxx, actual_maxx, "x: {}, y: {}", x, y);
                let expected_maxy = ((2.0 - y) * 100.0).round() as i32;
                let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
                assert_eq!(expected_maxy, actual_maxy, "x: {}, y: {}", x, y);
            }
        }
    }

    #[test]
    fn origins_relative_at_position() {
        // Test all origins from 0.0 to 1.0 in 0.1 increments
        for x in 0..11 {
            for y in 0..11 {
                let x = x as f64 * 0.1;
                let y = y as f64 * 0.1;
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                entity.set_origin(Origin::Relative(Vector2::new(x, y)));
                entity.set_position(Vector2::new(100.0, 100.0));
                entity.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_aabb().unwrap();
                let translation = 100 * 100;

                let offset_minx = (-x * 2.0 * 100.0).round() as i32;
                let expected_minx = offset_minx + translation;
                let actual_minx = (aabb.mins.x * 100.0).round() as i32;
                assert_eq!(expected_minx, actual_minx, "x: {}, y: {}", x, y);
                let offset_miny = (-y * 2.0 * 100.0).round() as i32;
                let expected_miny = offset_miny + translation;
                let actual_miny = (aabb.mins.y * 100.0).round() as i32;
                assert_eq!(expected_miny, actual_miny, "x: {}, y: {}", x, y);
                let offset_maxx = ((1.0 - x) * 2.0 * 100.0).round() as i32;
                let expected_maxx = offset_maxx + translation;
                let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
                assert_eq!(expected_maxx, actual_maxx, "x: {}, y: {}", x, y);
                let offset_maxy = ((1.0 - y) * 2.0 * 100.0).round() as i32;
                let expected_maxy = offset_maxy + translation;
                let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
                assert_eq!(expected_maxy, actual_maxy, "x: {}, y: {}", x, y);
            }
        }
    }

    #[test]
    fn rotation_with_center() {
        // Try out 360 degrees of rotation
        // with relative at 0.5, 0.5 the min and max of the AABB should be the same
        // each time
        for rotation in 0..360 {
            let rotation = rotation as f64;
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(0.5, 0.5)));
            entity.set_position(Vector2::new(0.0, 0.0));
            entity.set_rotation(rotation.to_radians());
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_aabb().unwrap();

            let expected_minx = -1 * 100;
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = -1 * 100;
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = 1 * 100;
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = 1 * 100;
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }

    #[test]
    fn rotation_with_center_at_position() {
        // Try out 360 degrees of rotation
        // with relative at 0.5, 0.5 the min and max of the AABB should be the same
        // each time
        for rotation in 0..360 {
            let rotation = rotation as f64;
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(0.5, 0.5)));
            entity.set_position(Vector2::new(100.0, 100.0));
            entity.set_rotation(rotation.to_radians());
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_aabb().unwrap();

            let expected_minx = -1 * 100 + 100 * 100;
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = -1 * 100 + 100 * 100;
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = 1 * 100 + 100 * 100;
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = 1 * 100 + 100 * 100;
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }

    #[test]
    fn rotation_at_zero_zero() {
        let expected_minxs = vec![0, -200, -200, 0];
        let expected_minys = vec![0, 0, -200, -200];
        let expected_maxxs = vec![200, 0, 0, 200];
        let expected_maxys = vec![200, 200, 0, 0];

        // Skip every 90 degrees and test new expected values
        for rotation_i in 0..4 {
            let rotation = rotation_i as f64 * 90.0;
            let rotation_rad = rotation.to_radians();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(0.0, 0.0)));
            entity.set_position(Vector2::new(0.0, 0.0));
            entity.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_aabb().unwrap();

            // Each time the AABB will jump around, calculate the expected values for min and max
            let expected_minx = expected_minxs[rotation_i];
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = expected_minys[rotation_i];
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = expected_maxxs[rotation_i];
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = expected_maxys[rotation_i];
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }

    #[test]
    fn rotation_at_one_one_at_position() {
        let expected_minxs = vec![-200, 0, 0, -200];
        let expected_minys = vec![-200, -200, 0, 0];
        let expected_maxxs = vec![0, 200, 200, 0];
        let expected_maxys = vec![0, 0, 200, 200];

        // Skip every 90 degrees and test new expected values
        for rotation_i in 0..4 {
            let rotation = rotation_i as f64 * 90.0;
            let rotation_rad = rotation.to_radians();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(1.0, 1.0)));
            entity.set_position(Vector2::new(100.0, 100.0));
            entity.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_aabb().unwrap();

            // Each time the AABB will jump around, calculate the expected values for min and max
            let expected_minx = expected_minxs[rotation_i] + 100 * 100;
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = expected_minys[rotation_i] + 100 * 100;
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = expected_maxxs[rotation_i] + 100 * 100;
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = expected_maxys[rotation_i] + 100 * 100;
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }

    #[test]
    fn rotation_at_one_one() {
        let expected_minxs = vec![-200, 0, 0, -200];
        let expected_minys = vec![-200, -200, 0, 0];
        let expected_maxxs = vec![0, 200, 200, 0];
        let expected_maxys = vec![0, 0, 200, 200];

        // Skip every 90 degrees and test new expected values
        for rotation_i in 0..4 {
            let rotation = rotation_i as f64 * 90.0;
            let rotation_rad = rotation.to_radians();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(1.0, 1.0)));
            entity.set_position(Vector2::new(0.0, 0.0));
            entity.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_aabb().unwrap();

            // Each time the AABB will jump around, calculate the expected values for min and max
            let expected_minx = expected_minxs[rotation_i];
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = expected_minys[rotation_i];
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = expected_maxxs[rotation_i];
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = expected_maxys[rotation_i];
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }
}
