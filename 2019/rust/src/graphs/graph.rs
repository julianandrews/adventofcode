pub trait Graph<'a> {
    type Item;

    fn nodes(&'a self) -> impl Iterator<Item = Self::Item> + 'a;

    fn neighbors(&'a self, value: &Self::Item) -> impl Iterator<Item = Self::Item> + 'a;
}
