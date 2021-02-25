pub trait ApiHelpers {
    type Simplified = Self;
    fn field_count() -> usize;
}
