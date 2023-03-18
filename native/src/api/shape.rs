use rapier2d_f64::na::{Point2, UnitComplex, Vector2};
use rapier2d_f64::parry::math::Isometry;
use rapier2d_f64::parry::shape::{
    Ball as ParryBall, Compound as ParryCompound, Shape as ParryShape, SharedShape,
};

use crate::entity::EntityShape;

#[derive(Debug, Clone)]
pub struct ShapeTransform {
    pub position_x: f64,
    pub position_y: f64,
    pub rotation: f64,
    pub absolute_origin_x: f64,
    pub absolute_origin_y: f64,
}

impl Into<Isometry<f64>> for ShapeTransform {
    fn into(self) -> Isometry<f64> {
        let mut isometry = Isometry::new(Vector2::new(self.position_x, self.position_y), 0.0);
        isometry.append_rotation_wrt_point_mut(
            &UnitComplex::new(self.rotation),
            &Point2::new(self.absolute_origin_x, self.absolute_origin_y),
        );
        isometry
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Ball {
        radius: f64,
    },
    Compound {
        children: Vec<Box<Shape>>,
        transforms: Vec<ShapeTransform>,
    },
}

impl Into<Box<dyn EntityShape>> for Shape {
    fn into(self) -> Box<dyn EntityShape> {
        match self {
            Shape::Ball { radius } => Box::new(ParryBall::new(radius)),
            Shape::Compound {
                children,
                transforms,
            } => {
                let mut shapes = Vec::new();
                let mut transforms = transforms.into_iter();
                for child in children {
                    let child: SharedShape = (*child).into();
                    shapes.push((transforms.next().unwrap().into(), child));
                }
                Box::new(ParryCompound::new(shapes))
            }
        }
    }
}

impl Into<SharedShape> for Shape {
    fn into(self) -> SharedShape {
        match self {
            Shape::Ball { radius } => SharedShape::new(ParryBall::new(radius)),
            Shape::Compound {
                children,
                transforms,
            } => {
                let mut shapes = Vec::new();
                let mut transforms = transforms.into_iter();
                for child in children {
                    let child: SharedShape = (*child).into();
                    shapes.push((transforms.next().unwrap().into(), child.into()));
                }
                SharedShape::new(ParryCompound::new(shapes))
            }
        }
    }
}

impl FromIterator<Shape> for Vec<Box<Shape>> {
    fn from_iter<I: IntoIterator<Item = Shape>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(Box::new(item));
        }
        vec
    }
}
