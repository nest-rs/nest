use {cgm, Shape, RendTri};
use cgm::{Rotation, Rotation2, Rad};

/// `Rotate` represents a shape which has been rotated.
#[derive(Copy, Clone, Debug)]
pub struct Rotate<S> {
    shape: S,
    rotation: cgm::Basis2<f32>,
}

impl<S> Rotate<S> {
    pub(crate) fn new(shape: S, angle: f32) -> Self {
        Rotate {
            shape: shape,
            rotation: cgm::Basis2::from_angle(Rad(angle)),
        }
    }
}

impl<S> IntoIterator for Rotate<S> where S: Shape {
    type Item = RendTri;
    type IntoIter = RotateIter<S::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        RotateIter {
            iter: self.shape.into_iter(),
            rotation: self.rotation,
        }
    }
}

/// Iterator which is produced by `Rotate`
#[derive(Clone, Debug)]
pub struct RotateIter<I> {
    iter: I,
    rotation: cgm::Basis2<f32>,
}

impl<I> Iterator for RotateIter<I> where I: Iterator<Item=RendTri> {
    type Item = RendTri;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| t.map_pos(|p| self.rotation.rotate_point(p)))
    }
}