use {cgm, Shape, RendTri};

/// `Translate` represents a shape which has been translated.
#[derive(Copy, Clone, Debug)]
pub struct Translate<S> {
    shape: S,
    v: cgm::Vector2<f32>,
}

impl<S> Translate<S> {
    pub(crate) fn new(shape: S, v: cgm::Vector2<f32>) -> Self {
        Translate {
            shape: shape,
            v: v,
        }
    }
}

impl<S> IntoIterator for Translate<S> where S: Shape {
    type Item = RendTri;
    type IntoIter = TranslateIter<S::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        TranslateIter {
            iter: self.shape.into_iter(),
            v: self.v,
        }
    }
}

/// Iterator which is produced by `Translate`
#[derive(Clone, Debug)]
pub struct TranslateIter<I> {
    iter: I,
    v: cgm::Vector2<f32>,
}

impl<I> Iterator for TranslateIter<I> where I: Iterator<Item=RendTri> {
    type Item = RendTri;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| t.map_pos(|p| p + self.v))
    }
}