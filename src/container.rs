use {BaseShape, Tri};

pub trait Container: Iterator {}

impl<SI> Container for SI
where
    SI: Iterator<Item = BaseShape>,
{
}
