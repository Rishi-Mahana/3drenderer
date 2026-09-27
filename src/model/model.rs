use std::sync::Arc;
use crate::model::mesh::{ Mesh};
use crate::model::material::{Material};
use crate::model::transform::Transform;
pub struct Model {
    pub mesh: Arc<Mesh>,
    pub material: Arc<Material>,
    pub transform: Transform,
}


impl Model {

    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            mesh: Arc::new(Mesh::new()),
            material: Arc::new(Material::new()),
        }
    }



    pub fn with_mesh(mut self, mesh: impl Into<Arc<Mesh>>) -> Self {
        self.mesh = mesh.into();
        self
    }

    pub fn with_material(mut self, material: impl Into<Arc<Material>>) -> Self {
        self.material = material.into();
        self
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform=transform;
        self
    }
}