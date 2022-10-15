use std::ops::{Rem, Sub};

pub fn closest_multiple<T: Copy + Sub<T, Output = T> + Rem<f32, Output = T>>(e: T, i: f32) -> T {
    e - (e % i)
}
