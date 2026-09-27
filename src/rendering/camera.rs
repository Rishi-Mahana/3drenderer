use glam::{Mat4, Quat, Vec3};
use glam::camera::rh::proj::opengl::{orthographic, perspective};

pub enum PerspectiveMode{
    Orthogonal(f32,f32,f32,f32,f32,f32),
    Perspective(f32,f32,f32,f32),
}
pub struct Camera{
    pub position: Vec3,
    pub up: Vec3,
    pub perspective:PerspectiveMode,
    pub orientation:Quat,
    mouse_pitch:f32,

}
impl Camera {
    pub fn new()->Self{
        Camera{
            position:Vec3::ZERO,
            up: Vec3::Y,
            perspective: PerspectiveMode::Perspective(45.0f32.to_radians(),1.0,0.1,1000.0),
            orientation: Quat::IDENTITY,
            mouse_pitch:0.0,
        }
    }
    pub fn follow_cursor(&mut self, delta_x: f32, delta_y: f32, sensitivity: f32) {
        self.rotate_world(Quat::from_rotation_y(-delta_x * sensitivity));

        let max_pitch = 89.0_f32.to_radians();
        let next_pitch = (self.mouse_pitch - delta_y * sensitivity).clamp(-max_pitch, max_pitch);
        let pitch_change = next_pitch - self.mouse_pitch;
        self.mouse_pitch = next_pitch;

        let right = self.orientation * Vec3::X;
        self.rotate_world(Quat::from_axis_angle(right, pitch_change));
    }
    pub fn with_position(mut self, pos:Vec3 )->Self{
        self.position = pos;
        self
    }
    pub fn with_up_vec(mut self, vec:Vec3)->Self{
        self.up=vec;
        self
    }
    pub fn with_perspective(mut self, persp:PerspectiveMode)->Self{
        self.perspective=persp;
        self
    }
    pub fn with_euler_angle(mut self, pitch:f32, roll:f32, yaw:f32)->Self{
        self.orientation=Quat::from_euler(
            glam::EulerRot::YXZ,
            yaw,
            pitch,
            roll
        );
        self
    }
    pub fn with_quat(mut self, quat:Quat)->Self{
        self.orientation=quat;
        self
    }
    


    pub fn get_direction(&self)->Vec3{
        self.orientation*-Vec3::Z
    }
    pub fn get_bases(&self)->(Vec3,Vec3,Vec3){
        let right = self.orientation * Vec3::X;
        let up = self.orientation * Vec3::Y;
        let forward = self.orientation * Vec3::Z;
        (right, up, forward)
    }
    pub fn set_up(&mut self, up:Vec3){
        self.up=up.normalize();
    }
    pub fn get_view_matrix(&self)->Mat4{
        Mat4::from_quat(self.orientation.conjugate())
            * Mat4::from_translation(-self.position)

    }
    pub fn get_projection_matrix(&self)->Mat4{
        match self.perspective{
            PerspectiveMode::Orthogonal(x1,x2,y1,y2,z1,z2)=>{
                orthographic(x1,x2,y1,y2,z1,z2)
            }
            PerspectiveMode::Perspective(fov,ar,z1,z2)=>{
                perspective(fov,ar,z1,z2)
            }
        }
    }
    pub fn rotate_world(&mut self, rot:Quat){
        self.orientation=(rot*self.orientation).normalize();
    }
    pub fn rotate_local(&mut self, rot:Quat){
        self.orientation= (self.orientation*rot).normalize();
    }
    pub fn translate(&mut self, x:f32,y:f32,z:f32){
        let right= self.orientation * Vec3::X;
        let forward= self.orientation * -Vec3::Z;
        self.position+=(x)*right+(y)*self.up+(z)*forward;

    }
}