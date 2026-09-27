use std::sync::Arc;
use glam::{Vec3, Vec4};
use crate::rendering::textures::texture::Texture;

#[derive(Clone)]
pub enum AlphaMode {
    Opaque,
    Mask(f32),
    Blend,
}

#[derive(Clone)]
pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec4,
    pub diffusemap: Option<Texture>,
    pub specular: Vec3,
    pub shininess: f32,
    pub specularmap: Option<Texture>,
    pub alphamode: AlphaMode,
}

impl Material {
    pub fn new() -> Self {
        Self { ambient: Vec3::splat(0.2), diffuse: Vec4::new(0.8, 0.8, 0.8, 1.0), diffusemap: None, specular: Vec3::splat(0.5), shininess: 32.0, specularmap: None, alphamode: AlphaMode::Opaque }
    }

    pub fn wood() -> Self {
        Self::new().with_ambient(Vec3::new(0.30, 0.20, 0.10)).with_diffuse(Vec4::new(0.60, 0.40, 0.20, 1.0)).with_specular(Vec3::splat(0.10)).with_shininess(8.0)
    }

    pub fn plastic() -> Self {
        Self::new().with_ambient(Vec3::splat(0.1)).with_diffuse(Vec4::new(0.8, 0.2, 0.2, 1.0)).with_specular(Vec3::splat(0.5)).with_shininess(32.0)
    }

    pub fn metal() -> Self {
        Self::new().with_ambient(Vec3::splat(0.25)).with_diffuse(Vec4::new(0.45, 0.45, 0.45, 1.0)).with_specular(Vec3::splat(0.90)).with_shininess(128.0)
    }

    pub fn glass() -> Self {
        Self::new().with_ambient(Vec3::splat(0.05)).with_diffuse(Vec4::new(0.6, 0.7, 0.8, 0.35)).with_specular(Vec3::ONE).with_shininess(256.0).with_alpha_mode(AlphaMode::Blend)
    }

    pub fn gold() -> Self {
        Self::new().with_ambient(Vec3::new(0.24725, 0.1995, 0.0745)).with_diffuse(Vec4::new(0.75164, 0.60648, 0.22648, 1.0)).with_specular(Vec3::new(0.628281, 0.555802, 0.366065)).with_shininess(51.2)
    }

    pub fn with_ambient(mut self, ambient: Vec3) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: Vec4) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.diffuse.w = alpha;
        self
    }

    pub fn with_specular(mut self, specular: Vec3) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_diffusemap(mut self, diffusemap: Texture) -> Self {
        self.diffusemap = Some(diffusemap);
        self
    }

    pub fn with_specularmap(mut self, specularmap: Texture) -> Self {
        self.specularmap = Some(specularmap);
        self
    }

    pub fn with_alpha_mode(mut self, alphamode: AlphaMode) -> Self {
        self.alphamode = alphamode;
        self
    }

    pub fn shared_reference(self) -> Arc<Material> {
        Arc::new(self)
    }
}

