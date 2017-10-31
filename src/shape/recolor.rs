use {RendTri, Shape, Color};
use cgm::{Rad, Rotation, Rotation2};

/// `Recolor` represents a shape which has had its color set to a new one.
#[derive(Copy, Clone, Debug)]
pub struct Recolor<S> {
    shape: S,
    color: Color,
}

impl<S> Recolor<S> {
    pub(crate) fn new(shape: S, color: Color) -> Self {
        Recolor {
            shape: shape,
            color: color,
        }
    }
}

impl<S> IntoIterator for Recolor<S>
where
    S: Shape,
{
    type Item = RendTri;
    type IntoIter = RecolorIter<S::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        RecolorIter {
            iter: self.shape.into_iter(),
            color: self.color,
        }
    }
}

/// Iterator which is produced by `Recolor`
#[derive(Clone, Debug)]
pub struct RecolorIter<I> {
    iter: I,
    color: Color,
}

impl<I> Iterator for RecolorIter<I>
where
    I: Iterator<Item = RendTri>,
{
    type Item = RendTri;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|t| t.map_color(|_| self.color))
    }
}
