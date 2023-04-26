pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lights;
pub mod material;
pub mod matrices;
pub mod rays;
pub mod shapes;
pub mod spheres;
pub mod transformations;
mod tuples;
pub mod world;

pub use tuples::{Point, Vector};

const EQUALITY_EPSILON: f64 = 0.00001;
