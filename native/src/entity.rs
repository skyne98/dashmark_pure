use core::fmt::Debug;
use downcast_rs::impl_downcast;
use generational_arena::Index;
use rapier2d::{
    na::{Point2, Vector2},
    parry::shape::Shape,
    prelude::Aabb,
};

use crate::{state::entity_manager::EntityManager, transform::Transform};

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

    pub fn get_size(&self) -> Vector2<f32> {
        match self.get_local_aabb() {
            Some(aabb) => Vector2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y),
            None => Vector2::new(0.0, 0.0),
        }
    }

    pub fn get_shape_natural_offset(&self) -> Vector2<f32> {
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

    pub fn get_local_aabb_and_size(&self) -> (Aabb, Vector2<f32>) {
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
