use {Shape, Tri};

pub trait Container: Iterator {}

impl<SI, S> Container for SI
where
    SI: Iterator<Item = S>,
    S: Shape,
{
}

pub trait IntoContainer {
    type IntoCont: Container;

    fn into_cont(self) -> Self::IntoCont;
}
