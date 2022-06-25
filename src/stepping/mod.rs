pub mod absolute;
pub mod blend;
pub mod geometric;
pub mod parabolic;

pub use absolute::Absolute;
pub use blend::Blend;
pub use geometric::Geometric;
pub use parabolic::Parabolic;

pub trait Stepping {
    fn calculate(&self, step: i32, cur: usize, max: usize) -> f32;
}
