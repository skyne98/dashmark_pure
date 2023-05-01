use std::cell::{Ref, RefCell};

use rapier2d_f64::na::{Isometry2, Matrix2x3, Point2, UnitComplex, Vector2};

use crate::matrix::TransformMatrix;

// Transform components
#[derive(Debug, Clone, Copy)]
pub enum Origin {
    Relative([f64; 2]),
    Absolute([f64; 2]),
}

impl Origin {
    pub fn new_relative(x: f64, y: f64) -> Self {
        Self::Relative([x, y])
    }

    pub fn new_absolute(x: f64, y: f64) -> Self {
        Self::Absolute([x, y])
    }

    pub fn to_absolute(&self, dimensions: [f64; 2]) -> [f64; 2] {
        match self {
            Origin::Relative(offset) => [offset[0] * dimensions[0], offset[1] * dimensions[1]],
            Origin::Absolute(offset) => *offset,
        }
    }

    pub fn to_relative(&self, dimensions: [f64; 2]) -> [f64; 2] {
        match self {
            Origin::Relative(offset) => *offset,
            Origin::Absolute(offset) => [offset[0] / dimensions[0], offset[1] / dimensions[1]],
        }
    }

    pub fn as_absolute(&self) -> Option<[f64; 2]> {
        match self {
            Origin::Relative(_) => None,
            Origin::Absolute(offset) => Some(*offset),
        }
    }

    pub fn as_relative(&self) -> Option<[f64; 2]> {
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
    position: [f64; 2],
    rotation: f64,
    scale: [f64; 2],
    /// Absolute origin of the transform
    origin: [f64; 2],
    matrix: RefCell<TransformMatrix>,
    dirty_matrix: RefCell<bool>,
    isometry: RefCell<Isometry2<f64>>,
    dirty_isometry: RefCell<bool>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: Default::default(),
            scale: [1.0, 1.0],
            origin: Default::default(),
            matrix: RefCell::new(TransformMatrix::default()),
            dirty_matrix: RefCell::new(true),
            isometry: RefCell::new(Isometry2::identity()),
            dirty_isometry: RefCell::new(true),
        }
    }
}

impl Transform {
    pub fn new(position: [f64; 2], rotation: f64, scale: [f64; 2]) -> Self {
        Self {
            position,
            rotation,
            scale,
            ..Default::default()
        }
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.dirty_matrix.replace(value);
        self.dirty_isometry.replace(value);
    }

    pub fn set_all(
        &mut self,
        position: [f64; 2],
        origin: [f64; 2],
        rotation: f64,
        scale: [f64; 2],
    ) {
        self.position = position;
        self.origin = origin;
        self.rotation = rotation;
        self.scale = scale;
        self.set_dirty(true);
    }

    pub fn position(&self) -> [f64; 2] {
        self.position
    }

    pub fn set_position(&mut self, position: [f64; 2]) {
        self.position = position;
        self.set_dirty(true);
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
        self.set_dirty(true);
    }

    pub fn scale(&self) -> [f64; 2] {
        self.scale
    }

    pub fn set_scale(&mut self, scale: [f64; 2]) {
        self.scale = scale;
        self.set_dirty(true);
    }

    pub fn origin(&self) -> Origin {
        Origin::Absolute(self.origin.into())
    }

    pub fn set_origin_absolute(&mut self, origin: [f64; 2]) {
        self.origin = origin;
        self.set_dirty(true);
    }

    pub fn set_origin_relative(&mut self, origin: [f64; 2], dimensions: [f64; 2]) {
        self.origin = Origin::Relative(origin.into()).to_absolute(dimensions);
        self.set_dirty(true);
    }

    pub fn transform_matrix(&self) -> Ref<TransformMatrix> {
        if *self.dirty_matrix.borrow() {
            let mut matrix = self.matrix.borrow_mut();
            matrix.build_transform(self.position, self.rotation, self.scale, self.origin);
            self.dirty_matrix.replace(false);
        }
        self.matrix.borrow()
    }

    pub fn matrix(&self) -> Matrix2x3<f64> {
        self.transform_matrix().matrix
    }

    pub fn isometry(&self, natural_offset: Vector2<f64>) -> Ref<Isometry2<f64>> {
        if *self.dirty_isometry.borrow() {
            let mut isometry = self.isometry.borrow_mut();
            let position: Vector2<f64> = self.position.into();
            let origin: Vector2<f64> = self.origin.into();
            isometry.translation.vector = position - origin + natural_offset;
            isometry.rotation = UnitComplex::new(0.0);
            isometry.append_rotation_wrt_point_mut(
                &UnitComplex::new(self.rotation),
                &Point2::from(self.position),
            );
            self.dirty_isometry.replace(false);
        }
        self.isometry.borrow()
    }
}
