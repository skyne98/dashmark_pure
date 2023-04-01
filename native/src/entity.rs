use anyhow::Result;
use core::fmt::Debug;
use downcast_rs::impl_downcast;
use generational_arena::Index;
use rapier2d_f64::{
    na::{Isometry2, Point2, UnitComplex, Vector2},
    parry::shape::Shape,
    prelude::Aabb,
};

use crate::{matrix::Transform, state::entity_manager::EntityManager};

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

// Transform components
#[derive(Debug)]
pub enum Origin {
    Relative(Vector2<f64>),
    Absolute(Vector2<f64>),
}

pub struct Entity {
    pub index: Index,
    parent: Option<Index>,

    // Transform
    origin: Origin,
    position: Vector2<f64>,
    scale: Vector2<f64>,
    rotation: f64,
    // ...other transform components
    transform_isometry: Isometry2<f64>,
    transform: Transform,
    dirty_transforms: bool,

    // Coliisions
    pub shape: Option<Box<dyn EntityShape>>,
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("index", &self.index)
            .field("parent", &self.parent)
            .field("origin", &self.origin)
            .field("position", &self.position)
            .field("scale", &self.scale)
            .field("rotation", &self.rotation)
            .field("transform_isometry", &self.transform_isometry)
            .field("transform", &self.transform)
            .field("dirty_transforms", &self.dirty_transforms)
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
            origin: Origin::Absolute(Vector2::new(0.0, 0.0)),
            position: Default::default(),
            scale: Vector2::new(1.0, 1.0),
            rotation: Default::default(),
            transform_isometry: Default::default(),
            transform: Default::default(),
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

    pub fn set_parent(&mut self, parent: Option<Index>) {
        self.parent = parent;
        self.dirty_transforms = true;
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

    pub fn get_absolute_offset(&self) -> Vector2<f64> {
        let size = self.get_size();
        match self.origin {
            Origin::Relative(offset) => Vector2::new(offset.x * size.x, offset.y * size.y),
            Origin::Absolute(offset) => offset,
        }
    }

    pub fn get_relative_offset(&self) -> Vector2<f64> {
        let size = self.get_size();
        match self.origin {
            Origin::Relative(offset) => offset,
            Origin::Absolute(offset) => Vector2::new(offset.x / size.x, offset.y / size.y),
        }
    }

    pub fn get_aabb_iso(&mut self) -> Option<Aabb> {
        let transform_iso = self.get_transform_isometry();
        let shape = self.shape.as_ref();
        shape.map(|shape| shape.compute_aabb(&transform_iso))
    }

    pub fn get_aabb(&mut self) -> Option<Aabb> {
        let aabb = self.get_local_aabb();
        let transform = self.borrow_transform();
        aabb.map(|mut aabb| {
            aabb.mins = transform.transform_point(&aabb.mins);
            aabb.maxs = transform.transform_point(&aabb.maxs);
            aabb
        })
    }

    pub fn get_transform_isometry(&mut self) -> Isometry2<f64> {
        if self.dirty_transforms {
            self.recalculate_transforms();
        }
        self.transform_isometry
    }

    pub fn borrow_transform(&mut self) -> &Transform {
        if self.dirty_transforms {
            self.recalculate_transforms();
        }
        &self.transform
    }

    pub fn get_local_aabb(&self) -> Option<Aabb> {
        self.shape.as_ref().map(|shape| shape.compute_local_aabb())
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

    pub fn get_shape(&self) -> Option<&dyn EntityShape> {
        self.shape.as_ref().map(|shape| shape.as_ref())
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

    pub fn set_shape_from_box(&mut self, shape: Box<dyn EntityShape>) {
        self.shape = Some(shape);
        self.dirty_transforms = true;
    }

    pub fn set_shape_to_none(&mut self) {
        self.shape = None;
        self.dirty_transforms = true;
    }

    pub fn recalculate_transforms(&mut self) {
        let relative_offset = self.get_relative_offset();
        let absolute_offset = self.get_absolute_offset();
        let (local_aabb, size) = self.get_local_aabb_and_size();
        // Natural offset - how far shape's AABB's min corner is from the zero
        let natural_offset = Vector2::new(-local_aabb.mins.x, -local_aabb.mins.y);

        // Isometry is a combination of translation and rotation
        self.transform_isometry.translation.vector =
            self.position - absolute_offset + natural_offset;
        self.transform_isometry.rotation = UnitComplex::new(0.0);
        self.transform_isometry.append_rotation_wrt_point_mut(
            &UnitComplex::new(self.rotation),
            &Point2::from(self.position),
        );

        // Affine transform is a combination of translation, rotation and scale
        self.transform = Transform::new();
        self.transform.build_transform(
            self.position,
            self.rotation,
            Vector2::new(self.scale.x, self.scale.y),
            absolute_offset - natural_offset,
        );

        self.dirty_transforms = false;
    }
}

// ===== Tests =====
#[cfg(test)]
mod test_entity {
    use rapier2d_f64::prelude::SharedShape;

    use super::*;

    fn assert_points_equal(a: Point2<f64>, b: Point2<f64>) {
        assert!((a.x - b.x).abs() < 0.2, "{:?} != {:?}", a, b);
        assert!((a.y - b.y).abs() < 0.2, "{:?} != {:?}", a, b);
    }

    #[test]
    fn entity() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_origin(Origin::Absolute(Vector2::new(0.0, 0.0)));
        entity.set_position(Vector2::new(0.0, 0.0));
        entity.set_rotation(0.0);
        let shape = rapier2d_f64::parry::shape::Ball::new(1.0);
        entity.set_shape(shape);
        let aabb = entity.get_aabb_iso().unwrap();
        assert_points_equal(aabb.mins, Point2::new(0.0, 0.0));
        assert_points_equal(aabb.maxs, Point2::new(2.0, 2.0));
        let aabb_transform = entity.get_aabb().unwrap();
        assert_points_equal(aabb.mins, aabb_transform.mins);
        assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
                let aabb = entity.get_aabb_iso().unwrap();
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

                let aabb_transform = entity.get_aabb().unwrap();
                assert_points_equal(aabb.mins, aabb_transform.mins);
                assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
                let aabb = entity.get_aabb_iso().unwrap();
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

                let aabb_transform = entity.get_aabb().unwrap();
                assert_points_equal(aabb.mins, aabb_transform.mins);
                assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
                let aabb = entity.get_aabb_iso().unwrap();
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

                let aabb_transform = entity.get_aabb().unwrap();
                assert_points_equal(aabb.mins, aabb_transform.mins);
                assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
            let aabb = entity.get_aabb_iso().unwrap();

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

            let aabb_transform = entity.get_aabb().unwrap();
            assert_points_equal(aabb.mins, aabb_transform.mins);
            assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
            let aabb = entity.get_aabb_iso().unwrap();

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

            let aabb_transform = entity.get_aabb().unwrap();
            assert_points_equal(aabb.mins, aabb_transform.mins);
            assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
            let aabb = entity.get_aabb_iso().unwrap();

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

            let aabb_transform = entity.get_aabb().unwrap();
            assert_points_equal(aabb.mins, aabb_transform.mins);
            assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
            let aabb = entity.get_aabb_iso().unwrap();

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

            let aabb_transform = entity.get_aabb().unwrap();
            assert_points_equal(aabb.mins, aabb_transform.mins);
            assert_points_equal(aabb.maxs, aabb_transform.maxs);
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
            let aabb = entity.get_aabb_iso().unwrap();

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
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_origin(Origin::Relative(Vector2::new(0.5, 0.5)));
        entity.set_position(Vector2::new(0.0, 0.0));
        entity.set_rotation(0.0);
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
        entity.set_shape(shape);

        let size = entity.get_size();
        assert_eq!(size, Vector2::new(4.0, 4.0));
        let aabb = entity.get_aabb_iso().unwrap();
        assert_points_equal(aabb.mins, Vector2::new(-2.0, -2.0).into());
        assert_points_equal(aabb.maxs, Vector2::new(2.0, 2.0).into());
    }

    #[test]
    fn compound_shape_with_zero_origin_works() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_origin(Origin::Relative(Vector2::new(0.0, 0.0)));
        entity.set_position(Vector2::new(0.0, 0.0));
        entity.set_rotation(0.0);
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
        entity.set_shape(shape);

        let size = entity.get_size();
        assert_eq!(size, Vector2::new(4.0, 4.0));
        let aabb = entity.get_aabb_iso().unwrap();
        assert_points_equal(aabb.mins, Vector2::new(0.0, 0.0).into());
        assert_points_equal(aabb.maxs, Vector2::new(4.0, 4.0).into());

        // Test all internal shapes are at correct positions
        let isometry = entity.get_transform_isometry();
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
            let mut entity = Entity::new(Index::from_raw_parts(0, 0));
            entity.set_origin(Origin::Relative(Vector2::new(0.0, 0.0)));
            entity.set_position(Vector2::new(0.0, 0.0));
            entity.set_rotation(rotation_rad);
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
            entity.set_shape(shape);

            let size = entity.get_size();
            assert_eq!(size, Vector2::new(4.0, 4.0));
            let aabb = entity.get_aabb_iso().unwrap();

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
