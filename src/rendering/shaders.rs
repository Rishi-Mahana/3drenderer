use std::collections::HashMap;
use std::ffi::CString;
use gl::types::{GLint, GLuint};
use glam::{Mat4, Vec3,};
use crate::rendering::textures::texture::Texture;
use crate::model::material::{AlphaMode, Material};
use std::sync::Arc;
use gltf::Glb;
use crate::rendering::lighting::lightsrc::LightMode;
use crate::rendering::lighting::lightsrc::LightSource;


pub struct Shader{
    pub program: GLuint,
    pub shadertype: ShaderType,
    pub uniforms: HashMap<String, GLint>,
    pub numlights: u8,
}
pub enum ShaderType{
    VertexColor,
    Lighting,

}



impl Shader {
    pub fn new(shader_type: ShaderType) -> Self {
        match shader_type {
            ShaderType::VertexColor => {
                let default_vertex_src = CString::new(include_str!("../../assets/shaders/vc_text_vert.txt")).unwrap();
                let default_frag_src = CString::new(include_str!("../../assets/shaders/vc_text_frag.txt")).unwrap();
                unsafe {
                    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
                    gl::ShaderSource(vertex_shader, 1, &default_vertex_src.as_ptr(), std::ptr::null());
                    gl::CompileShader(vertex_shader);
                    let mut success: gl::types::GLint = 1;
                    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
                    if success == 0 {
                        panic!("vertex shader compilation failed");
                    }
                    success = 1;


                    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
                    gl::ShaderSource(fragment_shader, 1, &default_frag_src.as_ptr(), std::ptr::null());
                    gl::CompileShader(fragment_shader);

                    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
                    if success == 0 {
                        panic!("fragment shader compilation failed");
                    }


                    success = 1;


                    let program1 = gl::CreateProgram();

                    gl::AttachShader(program1, vertex_shader);
                    gl::AttachShader(program1, fragment_shader);

                    gl::LinkProgram(program1);
                    gl::GetProgramiv(program1, gl::LINK_STATUS, &mut success);
                    if success == 0 {
                        panic!("linking program failed");
                    }

                    gl::DeleteShader(fragment_shader);
                    gl::DeleteShader(vertex_shader);
                    gl::UseProgram(program1);


                    let mut opacitylocation = gl::GetUniformLocation(program1, CString::new("opacity").unwrap().as_ptr());
                    gl::Uniform1f(opacitylocation, 1.0);
                    let mut tintlocation = gl::GetUniformLocation(program1, CString::new("tint").unwrap().as_ptr());
                    gl::Uniform3f(tintlocation, 1.0, 1.0, 1.0);
                    let mut modelMatrixlocation = gl::GetUniformLocation(program1, CString::new("modelMatrix").unwrap().as_ptr());
                    let default_mat = Mat4::IDENTITY;
                    gl::UniformMatrix4fv(
                        modelMatrixlocation,
                        1,
                        gl::FALSE,
                        default_mat.as_ref().as_ptr(),
                    );
                    let mut viewMatrixlocation = gl::GetUniformLocation(program1, CString::new("viewMatrix").unwrap().as_ptr());
                    gl::UniformMatrix4fv(
                        viewMatrixlocation,
                        1,
                        gl::FALSE,
                        default_mat.as_ref().as_ptr(),
                    );
                    let mut projMatrixlocation = gl::GetUniformLocation(program1, CString::new("projMatrix").unwrap().as_ptr());
                    gl::UniformMatrix4fv(
                        projMatrixlocation,
                        1,
                        gl::FALSE,
                        default_mat.as_ref().as_ptr(),
                    );
                    let mut interpollocation = gl::GetUniformLocation(program1, CString::new("interpolation").unwrap().as_ptr());
                    gl::Uniform1i(interpollocation, 1);

                    let mut viewposlocation = gl::GetUniformLocation(program1, CString::new("viewPos").unwrap().as_ptr());
                    gl::Uniform3f(viewposlocation, 0.0, 0.0, 0.0);

                    let mut numlightlocation = gl::GetUniformLocation(program1, CString::new("numlight").unwrap().as_ptr());
                    gl::Uniform1i(numlightlocation, 0);

                    let mut scolorlocation = gl::GetUniformLocation(program1, CString::new("shaderColor").unwrap().as_ptr());
                    gl::Uniform3f(scolorlocation, 1.0,1.0,1.0);
                    let mut ambientlocation = gl::GetUniformLocation(program1, CString::new("material.ambient").unwrap().as_ptr());
                    let mut diffuselocation = gl::GetUniformLocation(program1, CString::new("material.diffuse").unwrap().as_ptr());
                    let mut specularlocation = gl::GetUniformLocation(program1, CString::new("material.specular").unwrap().as_ptr());
                    let mut shininesslocation = gl::GetUniformLocation(program1, CString::new("material.shininess").unwrap().as_ptr());
                    let mut hasdiffuselocation = gl::GetUniformLocation(program1, CString::new("material.hasdiffuse").unwrap().as_ptr());
                    let mut hasspecularlocation = gl::GetUniformLocation(program1, CString::new("material.hasspecular").unwrap().as_ptr());
                    let mut diffusemaplocation = gl::GetUniformLocation(program1, CString::new("diffusemap").unwrap().as_ptr());
                    let mut specularmaplocation = gl::GetUniformLocation(program1, CString::new("specularmap").unwrap().as_ptr());

                    gl::Uniform1i(hasdiffuselocation, 0);
                    gl::Uniform1i(hasspecularlocation, 0);
                    gl::Uniform1i(diffusemaplocation, 0);
                    gl::Uniform1i(specularmaplocation, 1);


                    gl::Uniform3f(ambientlocation, 0.2, 0.2, 0.2);
                    gl::Uniform4f(diffuselocation, 0.8, 0.8, 0.8,1.0);
                    gl::Uniform3f(specularlocation, 0.5, 0.5, 0.5);
                    gl::Uniform1f(shininesslocation, 32.0);
                    let mut alphamodelocation = gl::GetUniformLocation(program1, CString::new("material.alphamode").unwrap().as_ptr());
                    let mut alphacutofflocation = gl::GetUniformLocation(program1, CString::new("material.alphacutoff").unwrap().as_ptr());

                    gl::Uniform1i(alphamodelocation, 0);
                    gl::Uniform1f(alphacutofflocation, 0.5);

                    let mut uniforms = HashMap::new();
                    uniforms.insert(String::from("opacity"), opacitylocation);
                    uniforms.insert(String::from("tint"), tintlocation);
                    uniforms.insert(String::from("interpolation"), interpollocation);
                    uniforms.insert(String::from("modelMatrix"), modelMatrixlocation);
                    uniforms.insert(String::from("viewMatrix"), viewMatrixlocation);
                    uniforms.insert(String::from("projMatrix"), projMatrixlocation);
                    uniforms.insert(String::from("viewPos"), viewposlocation);
                    uniforms.insert(String::from("numlight"), numlightlocation);
                    uniforms.insert(String::from("shaderColor"), scolorlocation);
                    uniforms.insert(String::from("material.ambient"), ambientlocation);
                    uniforms.insert(String::from("material.diffuse"), diffuselocation);
                    uniforms.insert(String::from("material.specular"), specularlocation);
                    uniforms.insert(String::from("material.shininess"), shininesslocation);
                    uniforms.insert(String::from("material.hasdiffuse"), hasdiffuselocation);
                    uniforms.insert(String::from("material.hasspecular"), hasspecularlocation);

                    uniforms.insert(String::from("material.alphamode"), alphamodelocation);
                    uniforms.insert(String::from("material.alphacutoff"), alphacutofflocation);
                    uniforms.insert(String::from("diffusemap"), diffusemaplocation);
                    uniforms.insert(String::from("specularmap"), specularmaplocation);


                    Shader { program: program1, shadertype: shader_type, uniforms: uniforms, numlights:0 }
                }
            }
            ShaderType::Lighting => {
                let default_vertex_src = CString::new(include_str!("../../assets/shaders/light_text_combined_vert.txt")).unwrap();
                let default_frag_src = CString::new(include_str!("../../assets/shaders/light_text_combined_frag.txt")).unwrap();
                unsafe {
                    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
                    gl::ShaderSource(vertex_shader, 1, &default_vertex_src.as_ptr(), std::ptr::null());
                    gl::CompileShader(vertex_shader);
                    let mut success: gl::types::GLint = 1;
                    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
                    if success == 0 {
                        panic!("vertex shader compilation failed");
                    }
                    success = 1;


                    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
                    gl::ShaderSource(fragment_shader, 1, &default_frag_src.as_ptr(), std::ptr::null());
                    gl::CompileShader(fragment_shader);

                    gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
                    if success == 0 {
                        panic!("fragment shader compilation failed");
                    }


                    success = 1;


                    let program1 = gl::CreateProgram();

                    gl::AttachShader(program1, vertex_shader);
                    gl::AttachShader(program1, fragment_shader);

                    gl::LinkProgram(program1);
                    gl::GetProgramiv(program1, gl::LINK_STATUS, &mut success);
                    if success == 0 {
                        panic!("linking program failed");
                    }

                    gl::DeleteShader(fragment_shader);
                    gl::DeleteShader(vertex_shader);
                    gl::UseProgram(program1);

                    let mut modelMatrixlocation = gl::GetUniformLocation(program1, CString::new("modelMatrix").unwrap().as_ptr());
                    let default_mat = Mat4::IDENTITY;
                    gl::UniformMatrix4fv(modelMatrixlocation, 1, gl::FALSE, default_mat.as_ref().as_ptr());

                    let mut viewMatrixlocation = gl::GetUniformLocation(program1, CString::new("viewMatrix").unwrap().as_ptr());
                    gl::UniformMatrix4fv(viewMatrixlocation, 1, gl::FALSE, default_mat.as_ref().as_ptr());

                    let mut projMatrixlocation = gl::GetUniformLocation(program1, CString::new("projMatrix").unwrap().as_ptr());
                    gl::UniformMatrix4fv(projMatrixlocation, 1, gl::FALSE, default_mat.as_ref().as_ptr());

                    let mut interpollocation = gl::GetUniformLocation(program1, CString::new("interpolation").unwrap().as_ptr());
                    gl::Uniform1i(interpollocation, 1);

                    let mut ambientlocation = gl::GetUniformLocation(program1, CString::new("material.ambient").unwrap().as_ptr());
                    let mut diffuselocation = gl::GetUniformLocation(program1, CString::new("material.diffuse").unwrap().as_ptr());
                    let mut specularlocation = gl::GetUniformLocation(program1, CString::new("material.specular").unwrap().as_ptr());
                    let mut shininesslocation = gl::GetUniformLocation(program1, CString::new("material.shininess").unwrap().as_ptr());

                    let mut hasdiffuselocation = gl::GetUniformLocation(program1, CString::new("material.hasdiffuse").unwrap().as_ptr());
                    let mut hasspecularlocation = gl::GetUniformLocation(program1, CString::new("material.hasspecular").unwrap().as_ptr());

                    let mut diffusemaplocation = gl::GetUniformLocation(program1, CString::new("diffusemap").unwrap().as_ptr());
                    let mut specularmaplocation = gl::GetUniformLocation(program1, CString::new("specularmap").unwrap().as_ptr());

                    gl::Uniform1i(hasdiffuselocation, 0);
                    gl::Uniform1i(hasspecularlocation, 0);
                    gl::Uniform1i(diffusemaplocation, 0);
                    gl::Uniform1i(specularmaplocation, 1);

                    let mut alphamodelocation = gl::GetUniformLocation(program1, CString::new("material.alphamode").unwrap().as_ptr());
                    let mut alphacutofflocation = gl::GetUniformLocation(program1, CString::new("material.alphacutoff").unwrap().as_ptr());

                    gl::Uniform1i(alphamodelocation, 0);
                    gl::Uniform1f(alphacutofflocation, 0.5);


                    gl::Uniform3f(ambientlocation, 0.2, 0.2, 0.2);
                    gl::Uniform3f(diffuselocation, 0.8, 0.8, 0.8);
                    gl::Uniform3f(specularlocation, 0.5, 0.5, 0.5);
                    gl::Uniform1f(shininesslocation, 32.0);

                    let mut viewposlocation = gl::GetUniformLocation(program1, CString::new("viewPos").unwrap().as_ptr());
                    gl::Uniform3f(viewposlocation, 0.0, 0.0, 0.0);

                    let mut opacitylocation = gl::GetUniformLocation(program1, CString::new("opacity").unwrap().as_ptr());
                    gl::Uniform1f(opacitylocation, 1.0);
                    let mut numlightlocation = gl::GetUniformLocation(program1, CString::new("numlight").unwrap().as_ptr());
                    gl::Uniform1i(numlightlocation, 0);

                    let mut uniforms = HashMap::new();

                    uniforms.insert(String::from("modelMatrix"), modelMatrixlocation);
                    uniforms.insert(String::from("viewMatrix"), viewMatrixlocation);
                    uniforms.insert(String::from("projMatrix"), projMatrixlocation);
                    uniforms.insert(String::from("interpolation"), interpollocation);
                    uniforms.insert(String::from("viewPos"), viewposlocation);
                    uniforms.insert(String::from("opacity"), opacitylocation);
                    uniforms.insert(String::from("material.ambient"), ambientlocation);
                    uniforms.insert(String::from("material.diffuse"), diffuselocation);
                    uniforms.insert(String::from("material.specular"), specularlocation);
                    uniforms.insert(String::from("material.shininess"), shininesslocation);
                    uniforms.insert(String::from("numlight"), numlightlocation);

                    uniforms.insert(String::from("material.hasdiffuse"), hasdiffuselocation);
                    uniforms.insert(String::from("material.hasspecular"), hasspecularlocation);
                    uniforms.insert(String::from("material.alphamode"), alphamodelocation);
                    uniforms.insert(String::from("material.alphacutoff"), alphacutofflocation);

                    uniforms.insert(String::from("diffusemap"), diffusemaplocation);
                    uniforms.insert(String::from("specularmap"), specularmaplocation);

                    Shader { program: program1, shadertype: shader_type, uniforms: uniforms, numlights: 0 }
                }
            }
            // }
    // ShaderType::TextureLighting => {
    //     let default_vertex_src = CString::new(include_str!("../assets/shaders/light_text_vert.txt")).unwrap();
    //     let default_frag_src = CString::new(include_str!("../assets/shaders/light_text_frag.txt")).unwrap();
    //
    //                 unsafe {
    //                     let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    //                     gl::ShaderSource(vertex_shader, 1, &default_vertex_src.as_ptr(), std::ptr::null());
    //                     gl::CompileShader(vertex_shader);
    //                     let mut success: gl::types::GLint = 1;
    //                     gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    //                     if success == 0 {
    //                         panic!("vertex shader compilation failed");
    //                     }
    //                     success = 1;
    //
    //
    //                     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    //                     gl::ShaderSource(fragment_shader, 1, &default_frag_src.as_ptr(), std::ptr::null());
    //                     gl::CompileShader(fragment_shader);
    //
    //                     gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    //                     if success == 0 {
    //                         panic!("fragment shader compilation failed");
    //                     }
    //
    //
    //                     success = 1;
    //
    //
    //                     let program1 = gl::CreateProgram();
    //
    //                     gl::AttachShader(program1, vertex_shader);
    //                     gl::AttachShader(program1, fragment_shader);
    //
    //                     gl::LinkProgram(program1);
    //                     gl::GetProgramiv(program1, gl::LINK_STATUS, &mut success);
    //                     if success == 0 {
    //                         panic!("linking program failed");
    //                     }
    //
    //                     gl::DeleteShader(fragment_shader);
    //                     gl::DeleteShader(vertex_shader);
    //                     gl::UseProgram(program1);
    //
    //
    //                     let mut opacitylocation = gl::GetUniformLocation(program1, CString::new("opacity").unwrap().as_ptr());
    //                     gl::Uniform1f(opacitylocation, 1.0);
    //                     let mut modelMatrixlocation = gl::GetUniformLocation(program1, CString::new("modelMatrix").unwrap().as_ptr());
    //                     let default_mat = Mat4::IDENTITY;
    //                     gl::UniformMatrix4fv(
    //                         modelMatrixlocation,
    //                         1,
    //                         gl::FALSE,
    //                         default_mat.as_ref().as_ptr(),
    //                     );
    //                     let mut viewMatrixlocation = gl::GetUniformLocation(program1, CString::new("viewMatrix").unwrap().as_ptr());
    //                     gl::UniformMatrix4fv(
    //                         viewMatrixlocation,
    //                         1,
    //                         gl::FALSE,
    //                         default_mat.as_ref().as_ptr(),
    //                     );
    //                     let mut projMatrixlocation = gl::GetUniformLocation(program1, CString::new("projMatrix").unwrap().as_ptr());
    //                     gl::UniformMatrix4fv(
    //                         projMatrixlocation,
    //                         1,
    //                         gl::FALSE,
    //                         default_mat.as_ref().as_ptr(),
    //                     );
    //                     let mut interpollocation = gl::GetUniformLocation(program1, CString::new("interpolation").unwrap().as_ptr());
    //                     gl::Uniform1i(interpollocation, 0);
    //
    //
    //                     let mut viewposlocation = gl::GetUniformLocation(program1, CString::new("viewPos").unwrap().as_ptr());
    //                     gl::Uniform3f(viewposlocation, 0.0, 0.0, 0.0);
    //
    //                     let mut ambientlocation = gl::GetUniformLocation(program1, CString::new("material.ambient").unwrap().as_ptr());
    //                     let mut diffuselocation = gl::GetUniformLocation(program1, CString::new("material.diffuse").unwrap().as_ptr());
    //                     let mut specularlocation = gl::GetUniformLocation(program1, CString::new("material.specular").unwrap().as_ptr());
    //                     let mut shininesslocation = gl::GetUniformLocation(program1, CString::new("material.shininess").unwrap().as_ptr());
    //                     gl::Uniform3f(ambientlocation, 0.2, 0.2, 0.2);
    //                     gl::Uniform3f(diffuselocation, 0.8, 0.8, 0.8);
    //                     gl::Uniform3f(specularlocation, 0.5, 0.5, 0.5);
    //                     gl::Uniform1f(shininesslocation, 32.0);
    //
    //
    //                     let mut uniforms = HashMap::new();
    //
    //                     let mut textureLocation = gl::GetUniformLocation(program1, CString::new("ourTexture").unwrap().as_ptr());
    //                     gl::Uniform1i(textureLocation, 0);
    //
    //                     let mut numlightlocation =gl::GetUniformLocation(program1, CString::new("numlight").unwrap().as_ptr());
    //                     gl::Uniform1i(numlightlocation,0);
    //
    //
    //                     uniforms.insert(String::from("interpolation"), interpollocation);
    //                     uniforms.insert(String::from("modelMatrix"), modelMatrixlocation);
    //                     uniforms.insert(String::from("viewMatrix"), viewMatrixlocation);
    //                     uniforms.insert(String::from("projMatrix"), projMatrixlocation);
    //                     uniforms.insert(String::from("ourTexture"), textureLocation);
    //                     uniforms.insert(String::from("viewPos"), viewposlocation);
    //                     uniforms.insert(String::from("material.ambient"), ambientlocation);
    //                     uniforms.insert(String::from("material.diffuse"), diffuselocation);
    //                     uniforms.insert(String::from("material.shininess"), shininesslocation);
    //                     uniforms.insert(String::from("material.specular"), specularlocation);
    //                     uniforms.insert(String::from("numlight"), numlightlocation);
    //                     uniforms.insert(String::from("opacity"), opacitylocation);
    //                     Shader { program: program1, shadertype: shader_type, uniforms: uniforms , numlights:0}
    //                 }
    //
    //         }
    //         ShaderType::Texture => {
    //             let default_vertex_src = CString::new(include_str!("../assets/shaders/texture_vert.txt")).unwrap();
    //             let default_frag_src = CString::new(include_str!("../assets/shaders/texture_frag.txt")).unwrap();
    //             unsafe {
    //                 let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    //                 gl::ShaderSource(vertex_shader, 1, &default_vertex_src.as_ptr(), std::ptr::null());
    //                 gl::CompileShader(vertex_shader);
    //                 let mut success: gl::types::GLint = 1;
    //                 gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    //                 if success == 0 {
    //                     panic!("vertex shader compilation failed");
    //                 }
    //                 success = 1;
    //
    //
    //                 let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    //                 gl::ShaderSource(fragment_shader, 1, &default_frag_src.as_ptr(), std::ptr::null());
    //                 gl::CompileShader(fragment_shader);
    //
    //                 gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
    //                 if success == 0 {
    //                     panic!("fragment shader compilation failed");
    //                 }
    //
    //
    //                 success = 1;
    //
    //
    //                 let program1 = gl::CreateProgram();
    //
    //                 gl::AttachShader(program1, vertex_shader);
    //                 gl::AttachShader(program1, fragment_shader);
    //
    //                 gl::LinkProgram(program1);
    //                 gl::GetProgramiv(program1, gl::LINK_STATUS, &mut success);
    //                 if success == 0 {
    //                     panic!("linking program failed");
    //                 }
    //
    //                 gl::DeleteShader(fragment_shader);
    //                 gl::DeleteShader(vertex_shader);
    //                 gl::UseProgram(program1);
    //
    //
    //                 let mut modelMatrixlocation = gl::GetUniformLocation(program1, CString::new("modelMatrix").unwrap().as_ptr());
    //                 let default_mat = Mat4::IDENTITY;
    //                 gl::UniformMatrix4fv(
    //                     modelMatrixlocation,
    //                     1,
    //                     gl::FALSE,
    //                     default_mat.as_ref().as_ptr(),
    //                 );
    //                 let mut viewMatrixlocation = gl::GetUniformLocation(program1, CString::new("viewMatrix").unwrap().as_ptr());
    //                 gl::UniformMatrix4fv(
    //                     viewMatrixlocation,
    //                     1,
    //                     gl::FALSE,
    //                     default_mat.as_ref().as_ptr(),
    //                 );
    //                 let mut projMatrixlocation = gl::GetUniformLocation(program1, CString::new("projMatrix").unwrap().as_ptr());
    //                 gl::UniformMatrix4fv(
    //                     projMatrixlocation,
    //                     1,
    //                     gl::FALSE,
    //                     default_mat.as_ref().as_ptr(),
    //                 );
    //                 let mut numlightlocation =gl::GetUniformLocation(program1, CString::new("numlight").unwrap().as_ptr());
    //                 gl::Uniform1i(numlightlocation,0);
    //                 //DEFINE YOUR NLIGHT wALA UNIFORM
    //                 let mut textureLocation = gl::GetUniformLocation(program1, CString::new("ourTexture").unwrap().as_ptr());
    //                 gl::Uniform1i(textureLocation, 0);
    //
    //                 let mut viewposlocation = gl::GetUniformLocation(program1, CString::new("viewPos").unwrap().as_ptr());
    //                 gl::Uniform3f(viewposlocation, 0.0, 0.0, 0.0);
    //
    //                 let mut tintlocation = gl::GetUniformLocation(program1, CString::new("tint").unwrap().as_ptr());
    //                 gl::Uniform3f(tintlocation, 1.0, 1.0, 1.0);
    //
    //                 let mut uniforms = HashMap::new();
    //                 uniforms.insert(String::from("modelMatrix"), modelMatrixlocation);
    //                 uniforms.insert(String::from("viewMatrix"), viewMatrixlocation);
    //                 uniforms.insert(String::from("projMatrix"), projMatrixlocation);
    //                 uniforms.insert(String::from("ourTexture"), textureLocation);
    //                 uniforms.insert(String::from("viewPos"), viewposlocation);
    //                 uniforms.insert(String::from("numlight"), numlightlocation);
    //                 uniforms.insert(String::from("tint"), tintlocation);
    //
    //                 Shader { program: program1, shadertype: shader_type, uniforms: uniforms , numlights:0}
    //             }
    //         }

            _ => { Shader { program: 0, shadertype: shader_type, uniforms: HashMap::new() , numlights:0 } }
        }
    }

    pub fn set_tint(&mut self, r: f32, g: f32, b: f32) {
        match self.shadertype{


            ShaderType::VertexColor => {
                unsafe {
                    gl::UseProgram(self.program);
                    gl::Uniform3f(*self.uniforms.get("tint").unwrap(), r, g, b);
                }
            }
            _=>{}

        }
    }
    pub fn set_shader_color(&mut self, r:f32,g:f32,b:f32){
        match self.shadertype{
            ShaderType::VertexColor => {
                unsafe {
                    gl::UseProgram(self.program);
                    gl::Uniform3f(*self.uniforms.get("shaderColor").unwrap(), r, g, b);
                }
            }
            _=>{}
        }
    }
    pub fn set_opacity(&mut self, o:f32){
        match self.shadertype {
            _ => {
                unsafe {
                    gl::UseProgram(self.program);
                    gl::Uniform1f(*self.uniforms.get("opacity").unwrap(), o);
                }
            }
        }
    }
    pub fn set_interpolation(&mut self, o:bool){
        match self.shadertype {
            _ => {
                unsafe {
                    gl::UseProgram(self.program);
                    gl::Uniform1i(*self.uniforms.get("interpolation").unwrap(), o as i32);
                }
            }
        }
    }


    pub fn set_numlights(&mut self, numlights: u8){
        if numlights>32{
            panic!("Only 32 lights allowed at a time for a particular shader sorry, SSBO functionality coming soon hopefully");

        }

        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1i(*self.uniforms.get("numlight").unwrap(), numlights as i32);
        }
    }


    pub fn set_tex_unit(&mut self, unit: u8) {
        unsafe {
            match self.shadertype {
                ShaderType::VertexColor => {
                    return
                }
                ShaderType::Lighting => {
                    return
                }
                _ => {
                    gl::UseProgram(self.program);
                    gl::Uniform1i(*self.uniforms.get("ourTexture").unwrap(), unit as i32)
                }
            }
        }
    }
    pub fn set_material(&mut self, mat: Arc<Material>) {
        unsafe {
            gl::UseProgram(self.program);

            if let Some(&location) = self.uniforms.get("material.ambient") {
                gl::Uniform3f(location, mat.ambient.x, mat.ambient.y, mat.ambient.z);
            }
            if let Some(&location) = self.uniforms.get("material.diffuse") {
                gl::Uniform4f(location, mat.diffuse.x, mat.diffuse.y, mat.diffuse.z, mat.diffuse.w);
            }
            if let Some(&location) = self.uniforms.get("material.specular") {
                gl::Uniform3f(location, mat.specular.x, mat.specular.y, mat.specular.z);
            }
            if let Some(&location) = self.uniforms.get("material.shininess") {
                gl::Uniform1f(location, mat.shininess);
            }

            let (alpha_mode, alpha_cutoff) = match mat.alphamode {
                AlphaMode::Opaque => (0, 0.5),
                AlphaMode::Mask(cutoff) => (1, cutoff),
                AlphaMode::Blend => (2, 0.5),
            };

            if let Some(&location) = self.uniforms.get("material.alphamode") {
                gl::Uniform1i(location, alpha_mode);
            }
            if let Some(&location) = self.uniforms.get("material.alphacutoff") {
                gl::Uniform1f(location, alpha_cutoff);
            }

            if let Some(&location) = self.uniforms.get("material.hasdiffuse") {
                gl::Uniform1i(location, mat.diffusemap.is_some() as i32);
            }
            if let Some(&location) = self.uniforms.get("material.hasspecular") {
                gl::Uniform1i(location, mat.specularmap.is_some() as i32);
            }

            if let Some(&location) = self.uniforms.get("diffusemap") {
                gl::Uniform1i(location, 0);
            }
            if let Some(&location) = self.uniforms.get("specularmap") {
                gl::Uniform1i(location, 1);
            }

            if let Some(texture) = &mat.diffusemap {
                texture.bind(0);
            } else {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }

            if let Some(texture) = &mat.specularmap {
                texture.bind(1);
            } else {
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }
    }
    pub fn set_index_light(&mut self, light:&LightSource, i: u8) {
        if i>=32{panic!("Only 32 lights allowed at a time for a particular shader sorry, SSBO functionality coming soon hopefully");}
        if i==self.numlights{self.numlights += 1;} //basically if #1 #2 #3 ... #nlight #___, if idx wants to fill the next openslot we just add one to numlight
        else if i>self.numlights{ panic!("The next open lighting slot is {}. lights follow a queue-based structure", self.numlights);}
        else{
            unsafe {
                gl::UseProgram(self.program);
                let ambientloc = self.get_ambient_loc(i as u8);
                let diffuseloc = self.get_diffuse_loc(i as u8);
                let specularloc = self.get_specular_loc(i as u8);
                gl::Uniform3f(ambientloc, light.ambient[0], light.ambient[1], light.ambient[2]);
                gl::Uniform3f(diffuseloc, light.diffuse[0], light.diffuse[1], light.diffuse[2]);
                gl::Uniform3f(specularloc, light.specular[0], light.specular[1], light.specular[2]);
                let modeloc = self.get_mode_loc(i as u8);
                match light.lightmode {
                    LightMode::Directional { direction } => {
                        let directionloc = self.get_direction_loc(i as u8);
                        gl::Uniform3f(directionloc, direction[0], direction[1], direction[2]);
                        gl::Uniform1i(modeloc, 1);
                    },
                    LightMode::Point { constant, linear, quadratic } => {
                        let constantloc = self.get_constant_loc(i as u8);
                        let linearloc = self.get_linear_loc(i as u8);
                        let quadraticloc = self.get_quadratic_loc(i as u8);
                        let positionloc = self.get_position_loc(i as u8);
                        gl::Uniform1f(constantloc, constant);
                        gl::Uniform1f(linearloc, linear);
                        gl::Uniform1f(quadraticloc, quadratic);
                        gl::Uniform3f(positionloc, light.model.transform.position[0], light.model.transform.position[1], light.model.transform.position[2]);
                        gl::Uniform1i(modeloc, 2);
                    }
                    LightMode::SpotLight { inner_cutoff, outer_cutoff, direction, constant, linear, quadratic } => {
                        let incutoffloc = self.get_incutoff_loc(i as u8);
                        let outcutoffloc = self.get_outcutoff_loc(i as u8);
                        let directionloc = self.get_direction_loc(i as u8);
                        let constantloc = self.get_constant_loc(i as u8);
                        let linearloc = self.get_linear_loc(i as u8);
                        let quadraticloc = self.get_quadratic_loc(i as u8);
                        let positionloc = self.get_position_loc(i as u8);
                        gl::Uniform1f(constantloc, constant);
                        gl::Uniform1f(linearloc, linear);
                        gl::Uniform1f(quadraticloc, quadratic);
                        gl::Uniform3f(positionloc, light.model.transform.position[0], light.model.transform.position[1], light.model.transform.position[2]);
                        gl::Uniform1f(incutoffloc, inner_cutoff);
                        gl::Uniform1f(outcutoffloc, outer_cutoff);
                        gl::Uniform3f(directionloc, direction[0], direction[1], direction[2]);
                        gl::Uniform1i(modeloc, 3);
                    }

                }
            }
        }

    }
    pub fn set_lights(&mut self, lights: &[LightSource]) {
        if lights.len() > 32 {
            panic!("Only 32 lights allowed at a time for a particular shader sorry, SSBO functionality coming soon hopefully");

        }
        let nlights=lights.len() as u8;

        self.set_numlights(nlights);

        self.numlights=nlights ;
        match self.shadertype {
            ShaderType::VertexColor => { return }
            _ => {}
        }

        unsafe {
            gl::UseProgram(self.program);
            for (i, light) in lights.iter().enumerate() {
                let ambientloc=self.get_ambient_loc(i as u8);
                let diffuseloc=self.get_diffuse_loc(i as u8);
                let specularloc=self.get_specular_loc(i as u8);
                gl::Uniform3f(ambientloc,light.ambient[0], light.ambient[1], light.ambient[2]);
                gl::Uniform3f(diffuseloc,light.diffuse[0], light.diffuse[1], light.diffuse[2]);
                gl::Uniform3f(specularloc,light.specular[0], light.specular[1], light.specular[2]);
                let modeloc=self.get_mode_loc(i as u8);
                match light.lightmode {
                    LightMode ::Directional{direction} => {
                        //println!("dirc");
                        let directionloc=self.get_direction_loc(i as u8);
                        gl::Uniform3f(directionloc, direction[0], direction[1], direction[2]);
                        gl::Uniform1i(modeloc, 1);
                    },
                    LightMode::Point{constant, linear, quadratic} => {
                        //println!("pnt");

                        let constantloc=self.get_constant_loc(i as u8);
                        let linearloc=self.get_linear_loc(i as u8);
                        let quadraticloc=self.get_quadratic_loc(i as u8);
                        let positionloc=self.get_position_loc(i as u8);
                        gl::Uniform1f(constantloc, constant);
                        gl::Uniform1f(linearloc, linear);
                        gl::Uniform1f(quadraticloc, quadratic);
                        gl::Uniform3f(positionloc,light.model.transform.position[0], light.model.transform.position[1], light.model.transform.position[2]);
                        gl::Uniform1i(modeloc, 2);
                    }
                    LightMode::SpotLight {inner_cutoff, outer_cutoff, direction,constant, linear, quadratic} => {
                        //println!("sptl");

                        let incutoffloc=self.get_incutoff_loc(i as u8);
                        let outcutoffloc=self.get_outcutoff_loc(i as u8);
                        let directionloc=self.get_direction_loc(i as u8);
                        let constantloc=self.get_constant_loc(i as u8);
                        let linearloc=self.get_linear_loc(i as u8);
                        let quadraticloc=self.get_quadratic_loc(i as u8);
                        let positionloc=self.get_position_loc(i as u8);
                        gl::Uniform1f(constantloc, constant);
                        gl::Uniform1f(linearloc, linear);
                        gl::Uniform1f(quadraticloc, quadratic);
                        gl::Uniform3f(positionloc,light.model.transform.position[0], light.model.transform.position[1], light.model.transform.position[2]);
                        gl::Uniform1f(incutoffloc, inner_cutoff.to_radians().cos());
                        gl::Uniform1f(outcutoffloc, outer_cutoff.to_radians().cos());
                        gl::Uniform3f(directionloc, direction[0], direction[1], direction[2]);
                        gl::Uniform1i(modeloc,3);
                    }
                }
            }
        }
    }

    fn get_position_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].position", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_constant_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].constant", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }


    fn get_linear_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].linear", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_quadratic_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].quadratic", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_ambient_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].ambient", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_diffuse_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].diffuse", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_specular_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].specular", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_direction_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].direction", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_incutoff_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].incutoff", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_outcutoff_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].outcutoff", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }

    fn get_mode_loc(&mut self, index: u8) -> GLint {
        let name = format!("lights[{}].mode", index);
        let name2 = name.clone();

        if let Some(&location) = self.uniforms.get(&name) { return location; }
        let location = unsafe {
            gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr(), )
        };
        self.uniforms.insert(name2, location);

        location
    }
}

//MAKE FUNCTON TO CHANGE ONE PARTICULAR LIGHT

