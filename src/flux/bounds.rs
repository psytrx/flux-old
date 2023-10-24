pub struct Bounds2<T> {
    pub min: T,
    pub max: T,
}

impl<T> Bounds2<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}
