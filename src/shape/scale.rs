use {cgm, RendTri, Shape};

/// `Scale` represents a shape which has been scald.
#[derive(Copy, Clone, Debug)]
pub struct Scale<S> {
	shape: S,
	v: cgm::Vector2<f32>,
}

impl<S> Scale<S> {
	pub(crate) fn new(shape: S, v: cgm::Vector2<f32>) -> Self {
		Scale { shape: shape, v: v }
	}
}

impl<S> IntoIterator for Scale<S>
where
	S: Shape,
{
	type Item = RendTri;
	type IntoIter = ScaleIter<S::IntoIter>;

	fn into_iter(self) -> Self::IntoIter {
		ScaleIter {
			iter: self.shape.into_iter(),
			v: self.v,
		}
	}
}

/// Iterator which is produced by `Scale`
#[derive(Clone, Debug)]
pub struct ScaleIter<I> {
	iter: I,
	v: cgm::Vector2<f32>,
}

impl<I> Iterator for ScaleIter<I>
where
	I: Iterator<Item = RendTri>,
{
	type Item = RendTri;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter
			.next()
			.map(|t| t.map_pos(|p| cgm::Point2::new(p.x * self.v.x, p.y * self.v.y)))
	}
}
