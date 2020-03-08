pub trait Graph<T> {
    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;

    fn neighbors<'a>(&'a self, value: &T) -> Box<dyn Iterator<Item = &'a T> + 'a>;
}
