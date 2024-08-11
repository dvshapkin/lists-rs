pub trait HasChild {
    fn add_child(&mut self, other: Self);
}
