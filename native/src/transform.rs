use std::cell::{Ref, RefCell};

use rapier2d_f64::na::{Isometry2, Matrix2x3, Point2, UnitComplex, Vector2};

use crate::matrix::TransformMatrix;

// Transform components
#[derive(Debug, Clone, Copy)]
pub enum Origin {
    Relative(Vector2<f64>),
    Absolute(Vector2<f64>),
}

impl Origin {
    pub fn new_relative(x: f64, y: f64) -> Self {
        Self::Relative(Vector2::new(x, y))
    }

    pub fn new_absolute(x: f64, y: f64) -> Self {
        Self::Absolute(Vector2::new(x, y))
    }

    pub fn to_absolute(&self, size: Vector2<f64>) -> Vector2<f64> {
        match self {
            Origin::Relative(offset) => Vector2::new(offset.x * size.x, offset.y * size.y),
            Origin::Absolute(offset) => *offset,
        }
    }

    pub fn to_relative(&self, size: Vector2<f64>) -> Vector2<f64> {
        match self {
            Origin::Relative(offset) => *offset,
            Origin::Absolute(offset) => Vector2::new(offset.x / size.x, offset.y / size.y),
        }
    }
}

impl Default for Origin {
    fn default() -> Self {
        Self::Relative(Vector2::new(0.0, 0.0))
    }
}

#[derive(Debug, Clone)]
pub struct Transform {
    position: Vector2<f64>,
    rotation: f64,
    scale: Vector2<f64>,
    origin: Vector2<f64>,
    matrix: RefCell<TransformMatrix>,
    dirty_matrix: RefCell<bool>,
    isometry: RefCell<Isometry2<f64>>,
    dirty_isometry: RefCell<bool>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0),
            origin: Default::default(),
            matrix: RefCell::new(TransformMatrix::default()),
            dirty_matrix: RefCell::new(true),
            isometry: RefCell::new(Isometry2::identity()),
            dirty_isometry: RefCell::new(true),
        }
    }
}

impl Transform {
    pub fn new(position: Vector2<f64>, rotation: f64, scale: Vector2<f64>) -> Self {
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

    pub fn position(&self) -> Vector2<f64> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f64>) {
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

    pub fn scale(&self) -> Vector2<f64> {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vector2<f64>) {
        self.scale = scale;
        self.set_dirty(true);
    }

    pub fn origin(&self) -> Origin {
        Origin::Relative(self.origin)
    }

    pub fn set_origin_relative(&mut self, origin: Vector2<f64>) {
        self.origin = origin;
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

    pub fn isometry(&self, size: Vector2<f64>) -> Ref<Isometry2<f64>> {
        if *self.dirty_isometry.borrow() {
            let mut isometry = self.isometry.borrow_mut();
            let origin = self.origin();
            let absolute_origin = origin.to_absolute(size);
            isometry.translation.vector = self.position - absolute_origin;
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
