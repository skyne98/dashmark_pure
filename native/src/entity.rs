use anyhow::Result;
use core::fmt::Debug;
use downcast_rs::impl_downcast;
use generational_arena::Index;
use rapier2d_f64::{
    na::{Isometry2, Point2, UnitComplex, Vector2},
    parry::shape::Shape,
    prelude::Aabb,
};

use crate::{matrix::TransformMatrix, state::entity_manager::EntityManager, transform::Transform};

// Shape
pub trait EntityShape: Shape {}
impl<T> EntityShape for T where T: Shape {}

impl_downcast!(EntityShape);

impl dyn EntityShape {
    pub fn as_shape<S: Shape>(&self) -> Option<&S> {
        self.downcast_ref::<S>()
    }

    pub fn as_shape_mut<S: Shape>(&mut self) -> Option<&mut S> {
        self.downcast_mut::<S>()
    }
}

pub struct Entity {
    pub index: Index,
    parent: Option<Index>,

    // Coliisions
    pub shape: Option<Box<dyn EntityShape>>,

    // Rendering
    pub priority: i32,
    pub color: i32,
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("index", &self.index)
            .field("parent", &self.parent)
            .field(
                "shape",
                &self.shape.as_ref().map(|_| "Some(Shape)".to_string()),
            )
            .finish()
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            index: Index::from_raw_parts(usize::MAX, u64::MAX),
            parent: None,
            shape: None,
            priority: 0,
            color: 0,
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

    pub fn set_parent(&mut self, parent: Option<Index>) {
        self.parent = parent;
    }

    pub fn get_parent(&self) -> Option<Index> {
        self.parent
    }

    pub fn get_parents(&self, entities: &EntityManager) -> Vec<Index> {
        let mut parents = Vec::new();
        let mut current = self.parent;
        while let Some(index) = current {
            parents.push(index);
            current = entities.get_entity(index).unwrap().get_parent();
        }
        parents
    }

    pub fn get_children(&self, entities: &EntityManager) -> Vec<Index> {
        entities
            .iter()
            .filter(|(_, entity)| entity.get_parent() == Some(self.index))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn get_size(&self) -> Vector2<f64> {
        match self.get_local_aabb() {
            Some(aabb) => Vector2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y),
            None => Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_shape_natural_offset(&self) -> Vector2<f64> {
        match self.get_local_aabb() {
            Some(aabb) => Vector2::new(-aabb.mins.x, -aabb.mins.y),
            None => Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_shape(&self) -> Option<&dyn EntityShape> {
        self.shape.as_ref().map(|shape| shape.as_ref())
    }

    pub fn set_shape<S: EntityShape>(&mut self, shape: S) {
        self.shape = Some(Box::new(shape));
    }

    pub fn set_shape_box(&mut self, shape: Box<dyn EntityShape>) {
        self.shape = Some(shape);
    }

    pub fn set_shape_none(&mut self) {
        self.shape = None;
    }

    pub fn get_local_aabb(&self) -> Option<Aabb> {
        self.shape.as_ref().map(|shape| shape.compute_local_aabb())
    }

    pub fn get_global_aabb(&self, transform: &Transform) -> Option<Aabb> {
        let natural_offset = self.get_shape_natural_offset();
        let transform_iso = transform.isometry(natural_offset);
        let shape = self.shape.as_ref();
        shape.map(|shape| shape.compute_aabb(&transform_iso))
    }

    pub fn get_local_aabb_and_size(&self) -> (Aabb, Vector2<f64>) {
        match self.get_local_aabb() {
            Some(aabb) => {
                let size = Vector2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y);
                (aabb, size)
            }
            None => (
                Aabb::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
                Vector2::new(0.0, 0.0),
            ),
        }
    }
}

// ===== Tests =====
#[cfg(test)]
mod test_entity {
    use rapier2d_f64::prelude::SharedShape;

    use super::*;

    fn assert_points_equal(a: Point2<f64>, b: Point2<f64>) {
        assert!((a.x - b.x).abs() < 0.0001, "{:?} != {:?}", a, b);
        assert!((a.y - b.y).abs() < 0.0001, "{:?} != {:?}", a, b);
    }

    #[test]
    fn entity() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        let mut transform = Transform::default();
        transform.set_origin_relative(Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0));
        transform.set_position(Vector2::new(0.0, 0.0));
        transform.set_rotation(0.0);
        let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
        entity.set_shape(shape);
        let aabb = entity.get_global_aabb(&transform).unwrap();
        assert_points_equal(aabb.mins, Point2::new(0.0, 0.0));
        assert_points_equal(aabb.maxs, Point2::new(2.0, 2.0));
    }

    #[test]
    fn origins_relative() {
        // Test all origins from 0.0 to 1.0 in 0.1 increments
        for x in 0..11 {
            for y in 0..11 {
                let x = x as f64 * 0.1;
                let y = y as f64 * 0.1;
                let mut transform = Transform::default();
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                transform.set_origin_relative(Vector2::new(x, y), Vector2::new(2.0, 2.0));
                transform.set_position(Vector2::new(0.0, 0.0));
                transform.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_global_aabb(&transform).unwrap();
                let expected_min = Point2::new(-x * 2.0, -y * 2.0);
                assert_points_equal(aabb.mins, expected_min);
                let expected_max = Point2::new((1.0 - x) * 2.0, (1.0 - y) * 2.0);
                assert_points_equal(aabb.maxs, expected_max);
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
                let mut transform = Transform::default();
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                transform.set_origin_absolute(Vector2::new(x, y));
                transform.set_position(Vector2::new(0.0, 0.0));
                transform.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_global_aabb(&transform).unwrap();
                let expected_min = Point2::new(-x, -y);
                assert_points_equal(aabb.mins, expected_min);
                let expected_max = Point2::new(2.0 - x, 2.0 - y);
                assert_points_equal(aabb.maxs, expected_max);
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
                let mut transform = Transform::default();
                let mut entity = Entity::new(Index::from_raw_parts(0, 0));
                transform.set_origin_relative(Vector2::new(x, y), Vector2::new(2.0, 2.0));
                transform.set_position(Vector2::new(100.0, 100.0));
                transform.set_rotation(0.0);
                let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
                entity.set_shape(shape);
                let aabb = entity.get_global_aabb(&transform).unwrap();
                let translation = Vector2::new(100.0, 100.0);

                let expected_min = Point2::new(-x * 2.0, -y * 2.0) + translation;
                assert_points_equal(aabb.mins, expected_min);
                let expected_max = Point2::new((1.0 - x) * 2.0, (1.0 - y) * 2.0) + translation;
                assert_points_equal(aabb.maxs, expected_max);
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
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(0.5, 0.5), Vector2::new(2.0, 2.0));
            transform.set_position(Vector2::new(0.0, 0.0));
            transform.set_rotation(rotation.to_radians());
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_global_aabb(&transform).unwrap();

            assert_points_equal(Point2::new(-1.0, -1.0), aabb.mins);
            assert_points_equal(Point2::new(1.0, 1.0), aabb.maxs);
        }
    }

    #[test]
    fn rotation_with_center_at_position() {
        // Try out 360 degrees of rotation
        // with relative at 0.5, 0.5 the min and max of the AABB should be the same
        // each time
        for rotation in 0..360 {
            let rotation = rotation as f64;
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(0.5, 0.5), Vector2::new(2.0, 2.0));
            transform.set_position(Vector2::new(100.0, 100.0));
            transform.set_rotation(rotation.to_radians());
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_global_aabb(&transform).unwrap();

            let expected_min = Vector2::new(-1.0, -1.0) + Vector2::new(100.0, 100.0);
            assert_points_equal(expected_min.into(), aabb.mins);
            let expected_max = Vector2::new(1.0, 1.0) + Vector2::new(100.0, 100.0);
            assert_points_equal(expected_max.into(), aabb.maxs);
        }
    }

    #[test]
    fn rotation_at_zero_zero() {
        let expected_minxs = vec![0.0, -2.0, -2.0, 0.0];
        let expected_minys = vec![0.0, 0.0, -2.0, -2.0];
        let expected_maxxs = vec![2.0, 0.0, 0.0, 2.0];
        let expected_maxys = vec![2.0, 2.0, 0.0, 0.0];

        // Skip every 90 degrees and test new expected values
        for rotation_i in 0..4 {
            let rotation = rotation_i as f64 * 90.0;
            let rotation_rad = rotation.to_radians();
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0));
            transform.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_global_aabb(&transform).unwrap();

            // Each time the AABB will jump around, calculate the expected values for min and max
            let expected_min = Point2::new(
                expected_minxs[rotation_i as usize],
                expected_minys[rotation_i as usize],
            );
            assert_points_equal(aabb.mins, expected_min);
            let expected_max = Point2::new(
                expected_maxxs[rotation_i as usize],
                expected_maxys[rotation_i as usize],
            );
            assert_points_equal(aabb.maxs, expected_max);
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
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
            transform.set_position(Vector2::new(100.0, 100.0));
            transform.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_global_aabb(&transform).unwrap();

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
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(1.0, 1.0), Vector2::new(2.0, 2.0));
            transform.set_position(Vector2::new(0.0, 0.0));
            transform.set_rotation(rotation_rad);
            let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
            entity.set_shape(shape);
            let aabb = entity.get_global_aabb(&transform).unwrap();

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
    fn compound_shape_works() {
        let shape = rapier2d_f64::parry::shape::Compound::new(vec![
            (
                Isometry2::new(Vector2::new(0.0, 0.0), 0.0),
                SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
            ),
            (
                Isometry2::new(Vector2::new(2.0, 2.0), 0.0),
                SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
            ),
        ]);
        let shape_size = shape.local_aabb().maxs - shape.local_aabb().mins;
        let mut transform = Transform::default();
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        transform.set_origin_relative(Vector2::new(0.5, 0.5), shape_size);
        transform.set_position(Vector2::new(0.0, 0.0));
        transform.set_rotation(0.0);

        entity.set_shape(shape);

        let size = entity.get_size();
        assert_eq!(size, Vector2::new(4.0, 4.0));
        let aabb = entity.get_global_aabb(&transform).unwrap();
        assert_points_equal(aabb.mins, Vector2::new(-2.0, -2.0).into());
        assert_points_equal(aabb.maxs, Vector2::new(2.0, 2.0).into());
    }

    #[test]
    fn compound_shape_with_zero_origin_works() {
        let shape = rapier2d_f64::parry::shape::Compound::new(vec![
            (
                Isometry2::new(Vector2::new(0.0, 0.0), 0.0),
                SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
            ),
            (
                Isometry2::new(Vector2::new(2.0, 2.0), 0.0),
                SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
            ),
        ]);
        let shape_size = shape.local_aabb().maxs - shape.local_aabb().mins;
        let mut transform = Transform::default();
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        transform.set_origin_relative(Vector2::new(0.0, 0.0), shape_size);
        transform.set_position(Vector2::new(0.0, 0.0));
        transform.set_rotation(0.0);
        entity.set_shape(shape);

        let size = entity.get_size();
        assert_eq!(size, Vector2::new(4.0, 4.0));
        let aabb = entity.get_global_aabb(&transform).unwrap();
        assert_points_equal(aabb.mins, Vector2::new(0.0, 0.0).into());
        assert_points_equal(aabb.maxs, Vector2::new(4.0, 4.0).into());

        // Test all internal shapes are at correct positions
        let isometry = transform.isometry(size);
        let compound: &rapier2d_f64::parry::shape::Compound =
            entity.get_shape().unwrap().as_shape().unwrap();
        let shape_aabbs = compound
            .aabbs()
            .iter()
            .map(|aabb| aabb.clone().transform_by(&isometry))
            .collect::<Vec<_>>();
        assert_eq!(shape_aabbs.len(), 2);
        assert_eq!(shape_aabbs[0].mins, Vector2::new(0.0, 0.0).into());
        assert_eq!(shape_aabbs[0].maxs, Vector2::new(2.0, 2.0).into());
        assert_eq!(shape_aabbs[1].mins, Vector2::new(2.0, 2.0).into());
        assert_eq!(shape_aabbs[1].maxs, Vector2::new(4.0, 4.0).into());
    }

    #[test]
    fn compound_shape_rotation() {
        let expected_minxs = vec![0, -400, -400, 0];
        let expected_minys = vec![0, 0, -400, -400];
        let expected_maxxs = vec![400, 0, 0, 400];
        let expected_maxys = vec![400, 400, 0, 0];

        for i in 0..4 {
            let rotation = i as f64 * 90.0;
            let rotation_rad = rotation.to_radians();
            let shape = rapier2d_f64::parry::shape::Compound::new(vec![
                (
                    Isometry2::new(Vector2::new(0.0, 0.0), 0.0),
                    SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
                ),
                (
                    Isometry2::new(Vector2::new(2.0, 2.0), 0.0),
                    SharedShape::new(rapier2d_f64::parry::shape::Ball::new(1.0)),
                ),
            ]);
            let shape_size = shape.local_aabb().maxs - shape.local_aabb().mins;
            let mut transform = Transform::default();
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            transform.set_origin_relative(Vector2::new(0.0, 0.0), shape_size);
            transform.set_position(Vector2::new(0.0, 0.0));
            transform.set_rotation(rotation_rad);
            entity.set_shape(shape);

            let size = entity.get_size();
            assert_eq!(size, Vector2::new(4.0, 4.0));
            let aabb = entity.get_global_aabb(&transform).unwrap();

            // Each time the AABB will jump around, calculate the expected values for min and max
            let expected_minx = expected_minxs[i];
            let actual_minx = (aabb.mins.x * 100.0).round() as i32;
            assert_eq!(expected_minx, actual_minx, "rotation: {}", rotation);
            let expected_miny = expected_minys[i];
            let actual_miny = (aabb.mins.y * 100.0).round() as i32;
            assert_eq!(expected_miny, actual_miny, "rotation: {}", rotation);
            let expected_maxx = expected_maxxs[i];
            let actual_maxx = (aabb.maxs.x * 100.0).round() as i32;
            assert_eq!(expected_maxx, actual_maxx, "rotation: {}", rotation);
            let expected_maxy = expected_maxys[i];
            let actual_maxy = (aabb.maxs.y * 100.0).round() as i32;
            assert_eq!(expected_maxy, actual_maxy, "rotation: {}", rotation);
        }
    }
}
