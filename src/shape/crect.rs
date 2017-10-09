use ::*;
use std::iter::{once, Chain, Once};

/// Two points make a rectangle.
#[derive(Copy, Clone, Debug)]
pub struct CRect(pub [f32; 2], pub [f32; 2], pub Color);

impl IntoIterator for CRect {
    type IntoIter = Chain<Once<RendTri>, Once<RendTri>>;
    type Item = RendTri;
    fn into_iter(self) -> Self::IntoIter {
        Iterator::chain(
            once(
                Tri::new(
                    [
                        [self.0[0], self.0[1]],
                        [self.1[0], self.0[1]],
                        [self.0[0], self.1[1]],
                    ],
                    [[0.0f32; 2]; 3],
                    self.2,
                ).into(),
            ),
            once(
                Tri::new(
                    [
                        [self.1[0], self.1[1]],
                        [self.0[0], self.1[1]],
                        [self.1[0], self.0[1]],
                    ],
                    [[0.0f32; 2]; 3],
                    self.2,
                ).into(),
            ),
        )
    }
}

/// Takes two points and creates a rectangle from those two points.
pub fn crect<A: Into<cgm::Point2<f32>>, B: Into<cgm::Point2<f32>>, C: Into<Color>>(
    first: A,
    second: B,
    color: C,
) -> CRect {
    CRect(first.into().into(), second.into().into(), color.into())
}
