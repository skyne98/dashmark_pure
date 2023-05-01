use std::cell::{Ref, RefCell};

use rapier2d::na::{Isometry2, Matrix2x3, Point2, UnitComplex, Vector1, Vector2};

use crate::matrix::TransformMatrix;

// Transform components
#[derive(Debug, Clone, Copy)]
pub enum Origin {
    Relative([f32; 2]),
    Absolute([f32; 2]),
}

impl Origin {
    pub fn new_relative(x: f32, y: f32) -> Self {
        Self::Relative([x, y])
    }

    pub fn new_absolute(x: f32, y: f32) -> Self {
        Self::Absolute([x, y])
    }

    pub fn to_absolute(&self, dimensions: [f32; 2]) -> [f32; 2] {
        match self {
            Origin::Relative(offset) => [offset[0] * dimensions[0], offset[1] * dimensions[1]],
            Origin::Absolute(offset) => *offset,
        }
    }

    pub fn to_relative(&self, dimensions: [f32; 2]) -> [f32; 2] {
        match self {
            Origin::Relative(offset) => *offset,
            Origin::Absolute(offset) => [offset[0] / dimensions[0], offset[1] / dimensions[1]],
        }
    }

    pub fn as_absolute(&self) -> Option<[f32; 2]> {
        match self {
            Origin::Relative(_) => None,
            Origin::Absolute(offset) => Some(*offset),
        }
    }

    pub fn as_relative(&self) -> Option<[f32; 2]> {
        match self {
            Origin::Relative(offset) => Some(*offset),
            Origin::Absolute(_) => None,
        }
    }
}

impl Default for Origin {
    fn default() -> Self {
        Self::Absolute([0.0, 0.0])
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    position: [f32; 2],
    rotation: f32,
    scale: [f32; 2],
    /// Absolute origin of the transform
    origin: [f32; 2],
    matrix: TransformMatrix,
    pub dirty_matrix: bool,
    isometry: Isometry2<f32>,
    pub dirty_isometry: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: Default::default(),
            scale: [1.0, 1.0],
            origin: Default::default(),
            matrix: TransformMatrix::default(),
            dirty_matrix: true,
            isometry: Isometry2::identity(),
            dirty_isometry: true,
        }
    }
}

impl Transform {
    pub fn new(position: [f32; 2], rotation: f32, scale: [f32; 2]) -> Self {
        Self {
            position,
            rotation,
            scale,
            ..Default::default()
        }
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.dirty_matrix = value;
        self.dirty_isometry = value;
    }

    pub fn set_all(
        &mut self,
        position: [f32; 2],
        origin: [f32; 2],
        rotation: f32,
        scale: [f32; 2],
    ) {
        self.position = position;
        self.origin = origin;
        self.rotation = rotation;
        self.scale = scale;
        self.set_dirty(true);
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    pub fn set_position(&mut self, position: [f32; 2]) {
        self.position = position;
        self.set_dirty(true);
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.set_dirty(true);
    }

    pub fn scale(&self) -> [f32; 2] {
        self.scale
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.scale = scale;
        self.set_dirty(true);
    }

    pub fn origin(&self) -> Origin {
        Origin::Absolute(self.origin.into())
    }

    pub fn set_origin_absolute(&mut self, origin: [f32; 2]) {
        self.origin = origin;
        self.set_dirty(true);
    }

    pub fn set_origin_relative(&mut self, origin: [f32; 2], dimensions: [f32; 2]) {
        self.origin = Origin::Relative(origin.into()).to_absolute(dimensions);
        self.set_dirty(true);
    }

    pub fn transform_matrix(&self) -> &TransformMatrix {
        &self.matrix
    }

    pub fn matrix(&self) -> &Matrix2x3<f32> {
        &self.transform_matrix().matrix
    }

    pub fn isometry(&self, natural_offset: Vector2<f32>) -> &Isometry2<f32> {
        &self.isometry
    }

    pub fn update_matrix(&mut self) {
        self.matrix
            .build_transform(self.position, self.rotation, self.scale, self.origin);
        self.dirty_matrix = false;
    }

    pub fn update_isometry(&mut self, natural_offset: Vector2<f32>) {
        let position: Vector2<f32> = self.position.into();
        let origin: Vector2<f32> = self.origin.into();
        self.isometry.translation.vector = position - origin + natural_offset;
        self.isometry.rotation = UnitComplex::new(0.0);
        self.isometry.append_rotation_wrt_point_mut(
            &UnitComplex::new(self.rotation),
            &Point2::from(self.position),
        );
        self.dirty_isometry = false;
    }
}
