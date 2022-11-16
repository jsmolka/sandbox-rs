use std::ops::Sub;

pub fn sub<T>(a: T, b: T) -> T
where
    T: Sub<Output = T> + Copy + Clone,
{
    a - b
}
