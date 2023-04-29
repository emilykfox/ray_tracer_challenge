pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod matrices;
pub mod patterns;
pub mod rays;
pub mod shapes;
pub mod transformations;
mod tuples;
pub mod world;

pub use tuples::{Point, Vector};

const EQUALITY_EPSILON: f64 = 0.00001;
