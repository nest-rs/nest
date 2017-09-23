use {Shape, RendTri};
use std::iter;

/// `Combine` represents the combination of two shapes.
#[derive(Copy, Clone, Debug)]
pub struct Combine<A, B>(pub(crate) A, pub(crate) B);

impl<A, B> IntoIterator for Combine<A, B> where A: Shape, B: Shape {
    type Item = RendTri;
    type IntoIter = iter::Chain<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        Iterator::chain(self.0.into_iter(), self.1.into_iter())
    }
}