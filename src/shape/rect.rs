use ::*;
use std::iter::{once, Chain, Once};

/// Two points make a rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Rect(pub [f32; 2], pub [f32; 2]);

impl IntoIterator for Rect {
    type IntoIter = Chain<Once<RendTri>, Once<RendTri>>;
    type Item = RendTri;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iterator::chain(
            once(
                Tri::new_pos([
                    [self.0[0], self.0[1]],
                    [self.1[0], self.0[1]],
                    [self.0[0], self.1[1]],
                ]).into(),
            ),
            once(
                Tri::new_pos([
                    [self.1[0], self.1[1]],
                    [self.0[0], self.1[1]],
                    [self.1[0], self.0[1]],
                ]).into(),
            ),
        )
    }
}

/// Takes two points and creates a rectangle from those two points.
#[inline]
pub fn rect<A: Into<cgm::Point2<f32>>, B: Into<cgm::Point2<f32>>>(first: A, second: B) -> Rect {
    Rect(first.into().into(), second.into().into())
}
