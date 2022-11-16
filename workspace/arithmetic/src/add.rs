use std::ops::Add;

pub fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Copy + Clone,
{
    a + b
}
