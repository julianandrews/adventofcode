pub trait Graph<'a, T> {
    fn nodes(&'a self) -> Box<dyn Iterator<Item = T> + 'a>;

    fn neighbors(&'a self, value: &T) -> Box<dyn Iterator<Item = T> + 'a>;
}
