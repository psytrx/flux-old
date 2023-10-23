mod camera;
mod interaction;
mod list;
mod ray;
mod render;
mod scene;
mod sphere;

pub use camera::Camera;
use glam::Vec2;
pub use list::ShapeList;
pub use render::render_image;
pub use scene::Scene;
pub use sphere::Sphere;

pub struct CameraSample {
    p_film: Vec2,
}
