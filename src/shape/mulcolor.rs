use {RendTri, Shape, Color};

/// `Mulcolor` represents a shape which has had its color multiplied by another.
#[derive(Copy, Clone, Debug)]
pub struct Mulcolor<S> {
    shape: S,
    color: Color,
}

impl<S> Mulcolor<S> {
    pub(crate) fn new(shape: S, color: Color) -> Self {
        Mulcolor {
            shape: shape,
            color: color,
        }
    }
}

impl<S> IntoIterator for Mulcolor<S>
where
    S: Shape,
{
    type Item = RendTri;
    type IntoIter = MulcolorIter<S::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        MulcolorIter {
            iter: self.shape.into_iter(),
            color: self.color,
        }
    }
}

/// Iterator which is produced by `Mulcolor`
#[derive(Clone, Debug)]
pub struct MulcolorIter<I> {
    iter: I,
    color: Color,
}

impl<I> Iterator for MulcolorIter<I>
where
    I: Iterator<Item = RendTri>,
{
    type Item = RendTri;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|t| t.map_color(|c| c.multiply(self.color)))
    }
}
