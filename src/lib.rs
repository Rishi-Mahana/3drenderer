pub mod model;
pub mod rendering;

pub use glam::Vec3;
pub use model::{
    loading::object::{Object, ObjectError},
    model::Model,
    transform::Transform,
};
pub use rendering::{
    camera::{Camera, PerspectiveMode},
    lighting::lightsrc::LightSource,
    renderer::{BlendFactor, Renderer},
    shaders::{Shader, ShaderType},
    window::{Window, WindowMode},
};