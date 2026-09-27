use glam::{Vec3, Quat, Mat4};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_quat(mut self, rotation: Quat) -> Transform {
        self.rotation = rotation;
        self
    }
    pub fn with_axis_angle(mut self, axis: Vec3, angle: f32) -> Transform {
        self.rotation = Quat::from_axis_angle(axis.normalize(), angle);
        self
    }

    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Transform {
        self.position = Vec3::new(x, y, z);
        self }
    pub fn new() -> Transform {
        Transform {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        } }

    pub fn get_scaling_matrix(&self) -> Mat4 { Mat4::from_scale(self.scale) }

    pub fn get_translation_matrix(&self) -> Mat4 { Mat4::from_translation(self.position) }

    pub fn get_rotation_matrix(&self) -> Mat4 { Mat4::from_quat(self.rotation) }

    pub fn get_model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position, ) }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos; }

    pub fn set_rotation(&mut self, rot: Quat) {
        self.rotation = rot;
    }

    pub fn translate(&mut self, vec: Vec3) {
        self.position += vec;
    }

    pub fn rotate(&mut self, rot: Quat) {
        self.rotation = rot * self.rotation;
    }

    pub fn scale(&mut self, a: Vec3) {
        self.scale = a;
    }
    pub fn rotate_world(&mut self, rot: Quat) {
        self.rotation = rot * self.rotation;
    }
}