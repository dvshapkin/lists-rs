pub trait HasChild {
    fn add_child(&mut self, other: Self);
    fn add_child_by_addr(&mut self, addr: *const Self);
}
