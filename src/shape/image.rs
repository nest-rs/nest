use *;
use glium::texture::Texture2d;
use std::iter::{Chain, Once, once};
use std::rc::Rc;

/// Texture rectangle.
#[derive(Clone, Debug)]
pub struct Image {
    rect: Rect,
    texture: Rc<Texture2d>,
}

impl IntoIterator for Image {
    type IntoIter = Chain<Once<RendTri>, Once<RendTri>>;
    type Item = RendTri;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iterator::chain(once(RendTri::from(Tri::new(
            [
                [self.rect.0[0], self.rect.0[1]],
                [self.rect.1[0], self.rect.0[1]],
                [self.rect.0[0], self.rect.1[1]],
            ],
            [
                [1.0, 1.0],
                [0.0, 1.0],
                [1.0, 0.0],
            ],
            Color::WHITE,
        )).map_texture(self.texture.clone())), once(RendTri::from(Tri::new(
            [
                [self.rect.1[0], self.rect.1[1]],
                [self.rect.0[0], self.rect.1[1]],
                [self.rect.1[0], self.rect.0[1]],
            ],
            [
                [0.0, 0.0],
                [1.0, 0.0],
                [0.0, 1.0],
            ],
            Color::WHITE,
        )).map_texture(self.texture)))
    }
}

/// Takes two points and a texture and draws the texture on the rectangle specified by the two points.
#[inline]
pub fn image<A, B, T>(first: A, second: B, texture: T) -> Image
    where A: Into<cgm::Point2<f32>>, B: Into<cgm::Point2<f32>>, T: Into<Rc<Texture2d>>
{
    Image {
        rect: Rect(first.into().into(), second.into().into()),
        texture: texture.into(),
    }
}

/// Takes a width and a texture and automatically adjusts the height to be proportional for the texture.
/// The resulting image rectangle is centered at the origin.
#[inline]
pub fn image_w<T: Into<Rc<Texture2d>>>(texture: T, width: f32) -> Image {
    let tex = texture.into();
    let height = tex.height() as f32 / tex.width() as f32 * width;
    Image {
        rect: Rect([-width / 2.0, -height / 2.0], [width / 2.0, height / 2.0]),
        texture: tex,
    }
}

/// Takes a height and a texture and automatically adjusts the width to be proportional for the texture.
/// The resulting image rectangle is centered at the origin.
#[inline]
pub fn image_h<T: Into<Rc<Texture2d>>>(texture: T, height: f32) -> Image {
    let tex = texture.into();
    let width = tex.width() as f32 / tex.height() as f32 * height;
    Image {
        rect: Rect([-width / 2.0, -height / 2.0], [width / 2.0, height / 2.0]),
        texture: tex,
    }
}