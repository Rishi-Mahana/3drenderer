use crate::Vec3;
use crate::model::loading::object::Object;
use crate::model::material::AlphaMode;
use crate::model::transform::Transform;
use crate::rendering::shaders::{Shader, ShaderType};
use crate::rendering::window::Window;
use crate::{Camera, LightSource, Model};
pub struct Renderer {}
pub enum FuncType {
    Always,
    Never,
    Equal,
    Notequal,
    Less,
    Greater,
    Lequal,
    Gequal,

}
pub enum StencilMask {
    All,
    None,
    Custom(u32),
}

pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    Increment,
    Incrementwrap,
    Decrement,
    Decrementwrap,
    Invert,
}
pub enum BlendFactor {
    Zero,
    One,
    SourceColor,
    OneMinusSourceColor,
    DestinationColor,
    OneMinusDestinationColor,
    SourceAlpha,
    OneMinusSourceAlpha,
    DestinationAlpha,
    OneMinusDestinationAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
}
impl Renderer{
    pub fn new(window: &mut Window) -> Renderer{
        gl::load_with(|s| window.get_proc_address(s));
        unsafe {
            let (width, height)=window.window.get_size();
            gl::Viewport(0, 0, width, height);
        }
        Renderer{}

    }


    pub fn begin_frame(&self,red:f32,green:f32,blue:f32,alpha:f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }
    pub fn draw_object(&mut self, obj:&Object, shader: &mut Shader, camera: &Camera){
        let mut blend_models = Vec::new();

        self.set_blend(false);

        for model in &obj.models {
            match model.material.alphamode {
                AlphaMode::Opaque | AlphaMode::Mask(_) => {
                    self.draw_model_wrt_object(model, shader, camera, &obj.transform);
                }
                AlphaMode::Blend => {
                    let model_matrix = obj.transform.get_model_matrix() * model.transform.get_model_matrix();
                    let world_position = model_matrix.transform_point3(Vec3::ZERO);
                    let distance_squared = camera.position.distance_squared(world_position);

                    blend_models.push((model, distance_squared));
                }
            }
        }

        blend_models.sort_by(|a, b| b.1.total_cmp(&a.1));

        self.set_blend(true);
        self.set_blend_func(
            BlendFactor::SourceAlpha,
            BlendFactor::OneMinusSourceAlpha,
        );

        unsafe { gl::DepthMask(gl::FALSE); }

        for (model, _) in blend_models {
            self.draw_model_wrt_object(model, shader, camera, &obj.transform);
        }

        unsafe { gl::DepthMask(gl::TRUE); }

        self.set_blend(false);

    }
    pub fn draw_model(&mut self, model:&Model, shader: &mut Shader, camera: &Camera){
        if model.mesh.vertices.len()==2{
            self.draw_line(model, shader, camera);
            return
        }
        unsafe {
            gl::UseProgram(shader.program);
            let viewMat=shader.uniforms.get("viewMatrix").unwrap();
            let modelMat=shader.uniforms.get("modelMatrix").unwrap();
            let projMat=shader.uniforms.get("projMatrix").unwrap();
            let viewPos=shader.uniforms.get("viewPos").unwrap();
            gl::UniformMatrix4fv(*viewMat,1, gl::FALSE, camera.get_view_matrix().as_ref().as_ptr());
            gl::UniformMatrix4fv(*modelMat,1, gl::FALSE, model.transform.get_model_matrix().as_ref().as_ptr());
            gl::UniformMatrix4fv(*projMat,1, gl::FALSE, camera.get_projection_matrix().as_ref().as_ptr());
            gl::Uniform3f(*viewPos,camera.position[0],camera.position[1],camera.position[2]);
            shader.set_material(model.material.clone());
            gl::BindVertexArray(model.mesh.get_vao());


            gl::DrawElements(gl::TRIANGLES, model.mesh.indices.len() as i32,gl::UNSIGNED_INT, std::ptr::null());
        }
    }
    fn draw_model_wrt_object(&mut self, model:&Model, shader: &mut Shader, camera: &Camera, obj_trans:&Transform){
        if model.mesh.vertices.len()==2{
            self.draw_line(model, shader, camera);
            return
        }
        unsafe {
            gl::UseProgram(shader.program);
            let viewMat=shader.uniforms.get("viewMatrix").unwrap();
            let modelMat=shader.uniforms.get("modelMatrix").unwrap();
            let projMat=shader.uniforms.get("projMatrix").unwrap();
            let viewPos=shader.uniforms.get("viewPos").unwrap();

            let m=obj_trans.get_model_matrix()*model.transform.get_model_matrix();
            gl::UniformMatrix4fv(*viewMat,1, gl::FALSE, camera.get_view_matrix().as_ref().as_ptr());
            gl::UniformMatrix4fv(*modelMat,1, gl::FALSE, m.as_ref().as_ptr());
            gl::UniformMatrix4fv(*projMat,1, gl::FALSE, camera.get_projection_matrix().as_ref().as_ptr());
            gl::Uniform3f(*viewPos,camera.position[0],camera.position[1],camera.position[2]);
            shader.set_material(model.material.clone());
            gl::BindVertexArray(model.mesh.get_vao());


            gl::DrawElements(gl::TRIANGLES, model.mesh.indices.len() as i32,gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn draw_lights(&mut self, lights: &[LightSource], shader:&mut Shader,camera:&Camera){

        unsafe {
            let mut light_shader = Shader::new(ShaderType::VertexColor);

            shader.set_lights(lights);
            gl::UseProgram(light_shader.program);

            for light in lights {
                if light.model.mesh.vertices.len() == 2 {
                    self.draw_line(light.model, &mut light_shader, camera);

                    continue;
                }
                if light.light_specific_coloring==true {
                    light_shader.set_interpolation(false);
                    light_shader.set_shader_color(light.diffuse[0], light.diffuse[1], light.diffuse[2]);
                }
                else{
                    light_shader.set_interpolation(true);

                }


                let viewMat = light_shader.uniforms.get("viewMatrix").unwrap();
                let modelMat = light_shader.uniforms.get("modelMatrix").unwrap();
                let projMat = light_shader.uniforms.get("projMatrix").unwrap();
                let view = camera.get_view_matrix();
                let m = light.model.transform.get_model_matrix();
                let pj = camera.get_projection_matrix();
                gl::UniformMatrix4fv(*viewMat, 1, gl::FALSE, camera.get_view_matrix().as_ref().as_ptr());
                gl::UniformMatrix4fv(*modelMat, 1, gl::FALSE, light.model.transform.get_model_matrix().as_ref().as_ptr());
                gl::UniformMatrix4fv(*projMat, 1, gl::FALSE, camera.get_projection_matrix().as_ref().as_ptr());
                gl::BindVertexArray(light.model.mesh.get_vao());
                gl::DrawElements(gl::TRIANGLES, light.model.mesh.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
                light_shader.set_interpolation(false);
            }
        }
    }
    pub fn draw_line(&mut self, model:&Model, shader:&mut Shader, camera:&Camera){
        unsafe {
            gl::BindVertexArray(model.mesh.get_vao());
            gl::UseProgram(shader.program);
            gl::DrawArrays(gl::LINES, 0, 2);
            gl::BindVertexArray(0);
        }


    }

    pub fn set_wireframe(&mut self){
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }
    pub fn set_fill(&mut self){
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }
    pub fn set_depth_test(&mut self, flag:bool){
        unsafe{
            if flag==true {
                gl::Enable(gl::DEPTH_TEST);
            }
            else{
                gl::Disable(gl::DEPTH_TEST);
            }

        }
    }
    pub fn set_stencil_test(&mut self, flag:bool){
        unsafe{
            if flag==true{
                gl::Enable(gl::STENCIL_TEST);

            }
            else{
                gl::Disable(gl::STENCIL_TEST);
            }
        }
    }

    pub fn set_depth_func(&mut self, f: FuncType) {
        let depth_func = match f {
            FuncType::Always => gl::ALWAYS,
            FuncType::Never => gl::NEVER,
            FuncType::Equal => gl::EQUAL,
            FuncType::Notequal => gl::NOTEQUAL,
            FuncType::Less => gl::LESS,
            FuncType::Greater => gl::GREATER,
            FuncType::Lequal => gl::LEQUAL,
            FuncType::Gequal => gl::GEQUAL,
        };

        unsafe {
            gl::DepthFunc(depth_func);
        }
    }
    pub fn set_stencil_mask(&mut self, mask: StencilMask) {
        let mask = match mask {
            StencilMask::All => u32::MAX,
            StencilMask::None => 0,
            StencilMask::Custom(bits) => bits,
        };

        unsafe {
            gl::StencilMask(mask);
        }
    }
    pub fn set_stencil_func(&mut self, func: FuncType, reference: i32, mask: u32) {
        let func = match func {
            FuncType::Never => gl::NEVER,
            FuncType::Less => gl::LESS,
            FuncType::Lequal => gl::LEQUAL,
            FuncType::Greater => gl::GREATER,
            FuncType::Gequal => gl::GEQUAL,
            FuncType::Equal => gl::EQUAL,
            FuncType::Notequal => gl::NOTEQUAL,
            FuncType::Always => gl::ALWAYS,
        };

        unsafe {
            gl::StencilFunc(func, reference, mask);
        }
    }
    pub fn set_stencil_op(&mut self, sfail: StencilOp, dfail: StencilOp, dpas: StencilOp, ) {

        unsafe {
            gl::StencilOp(stencil_to_gl(sfail), stencil_to_gl(dfail), stencil_to_gl(dpas));
        }
    }
    pub fn set_blend(&mut self, enabled: bool) {
        unsafe {
            if enabled {
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }
        }
    }

    pub fn set_blend_func(&mut self, source: BlendFactor, destination: BlendFactor) {
        unsafe {
            gl::BlendFunc(blend_factor_to_gl(source), blend_factor_to_gl(destination));
        }
    }

    pub fn set_blend_func_separate(&mut self, source_rgb: BlendFactor, destination_rgb: BlendFactor, source_alpha: BlendFactor, destination_alpha: BlendFactor, ) {
        unsafe {
            gl::BlendFuncSeparate(
                blend_factor_to_gl(source_rgb),
                blend_factor_to_gl(destination_rgb),
                blend_factor_to_gl(source_alpha),
                blend_factor_to_gl(destination_alpha),
            );
        }
    }

    pub fn set_blend_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::BlendColor(red, green, blue, alpha);
        }
    }



}
fn stencil_to_gl(sop:StencilOp) -> gl::types::GLenum{
    let gl = match sop {
        StencilOp::Keep => gl::KEEP,
        StencilOp::Zero => gl::ZERO,
        StencilOp::Replace => gl::REPLACE,
        StencilOp::Increment => gl::INCR,
        StencilOp::Incrementwrap => gl::INCR_WRAP,
        StencilOp::Decrement => gl::DECR,
        StencilOp::Decrementwrap => gl::DECR_WRAP,
        StencilOp::Invert => gl::INVERT,
    };
    gl
}
fn blend_factor_to_gl(factor: BlendFactor) -> gl::types::GLenum {
    match factor {
        BlendFactor::Zero => gl::ZERO,
        BlendFactor::One => gl::ONE,
        BlendFactor::SourceColor => gl::SRC_COLOR,
        BlendFactor::OneMinusSourceColor => gl::ONE_MINUS_SRC_COLOR,
        BlendFactor::DestinationColor => gl::DST_COLOR,
        BlendFactor::OneMinusDestinationColor => gl::ONE_MINUS_DST_COLOR,
        BlendFactor::SourceAlpha => gl::SRC_ALPHA,
        BlendFactor::OneMinusSourceAlpha => gl::ONE_MINUS_SRC_ALPHA,
        BlendFactor::DestinationAlpha => gl::DST_ALPHA,
        BlendFactor::OneMinusDestinationAlpha => gl::ONE_MINUS_DST_ALPHA,
        BlendFactor::ConstantColor => gl::CONSTANT_COLOR,
        BlendFactor::OneMinusConstantColor => gl::ONE_MINUS_CONSTANT_COLOR,
        BlendFactor::ConstantAlpha => gl::CONSTANT_ALPHA,
        BlendFactor::OneMinusConstantAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,
    }
}