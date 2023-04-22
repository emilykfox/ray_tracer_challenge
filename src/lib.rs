pub mod canvas;
pub mod intersections;
pub mod matrices;
pub mod rays;
pub mod spheres;
pub mod transformations;
mod tuples;

pub use tuples::{Point, Vector};

const EQUALITY_EPSILON: f64 = 0.00001;
