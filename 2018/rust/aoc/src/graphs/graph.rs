pub trait Graph<T> {
    fn values(&self) -> Vec<T>;

    fn neighbors(&self, value: &T) -> Vec<T>;
}
