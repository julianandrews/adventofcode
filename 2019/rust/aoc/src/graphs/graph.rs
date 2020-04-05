pub trait Graph<'a> {
    type Item;

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn neighbors(&'a self, value: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}
