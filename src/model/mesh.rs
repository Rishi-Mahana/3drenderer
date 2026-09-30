use std::sync::Arc;
use gl::types::GLuint;
use glam::{Vec3, Vec2};
use crate::rendering::textures::texture::Texture;
use image::GenericImageView;
#[repr(C)]              //basically makes a predictable memory structure so that C/C++ FFIs can take it in
#[derive(Clone,Copy)]
#[derive(Debug)]//doing this so that we can simply pass in the vec<vertex> stored in the mesh instead of pre-processing
pub struct Vertex{
    pub pos: Vec3,
    pub color: Vec3,
    pub normal: Vec3,
    pub text:Vec2,

}



pub enum TextMode{
    Repeat,
    MirroredRepeat,
    ClampEdge,
    ClampBorder,
}
impl Vertex{
    pub fn new(pos:Vec3, col:Vec3, text:Vec2) -> Vertex{
        Vertex{pos:pos, color:col, normal:Vec3::ZERO, text}
    }
    pub fn translate_vertex(&mut self, delta:&Vec3){
        self.pos=self.pos+delta;

    }
    pub fn set_normal(&mut self, normal:Vec3){
        self.normal=normal;
    }

}
pub struct Mesh{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vao: GLuint,
    vbo:GLuint,
    ebo:GLuint,
}

impl Mesh{
    pub fn new()->Self{
        Mesh{vertices: Vec::new(), indices: Vec::new(), vao:0 ,vbo:0, ebo:0}
    }
    pub fn with_vertices(mut self, vertices: Vec<Vertex>)->Self{
        self.vertices = vertices;
        self
    }
    pub fn with_indices(mut self, indices: Vec<u32>)->Self{
        self.indices = indices;
        self
    }
    pub fn new_cube()->Self {
        let color=Vec3::new(0.5,0.5,0.5);
        Mesh {
            vertices: vec![
                // Front Face (z = 1)
                Vertex::new(Vec3::new(0.0, 0.0, 1.0), color, Vec2::new(0.0, 0.0)), // 0
                Vertex::new(Vec3::new(1.0, 0.0, 1.0), color, Vec2::new(1.0, 0.0)), // 1
                Vertex::new(Vec3::new(1.0, 1.0, 1.0), color, Vec2::new(1.0, 1.0)), // 2
                Vertex::new(Vec3::new(0.0, 1.0, 1.0), color, Vec2::new(0.0, 1.0)), // 3

                // Back Face (z = 0)
                Vertex::new(Vec3::new(1.0, 0.0, 0.0), color, Vec2::new(0.0, 0.0)), // 4
                Vertex::new(Vec3::new(0.0, 0.0, 0.0), color, Vec2::new(1.0, 0.0)), // 5
                Vertex::new(Vec3::new(0.0, 1.0, 0.0), color, Vec2::new(1.0, 1.0)), // 6
                Vertex::new(Vec3::new(1.0, 1.0, 0.0), color, Vec2::new(0.0, 1.0)), // 7

                // Left Face (x = 0)
                Vertex::new(Vec3::new(0.0, 0.0, 0.0), color, Vec2::new(0.0, 0.0)), // 8
                Vertex::new(Vec3::new(0.0, 0.0, 1.0), color, Vec2::new(1.0, 0.0)), // 9
                Vertex::new(Vec3::new(0.0, 1.0, 1.0), color, Vec2::new(1.0, 1.0)), //10
                Vertex::new(Vec3::new(0.0, 1.0, 0.0), color, Vec2::new(0.0, 1.0)), //11

                // Right Face (x = 1)
                Vertex::new(Vec3::new(1.0, 0.0, 1.0), color, Vec2::new(0.0, 0.0)), //12
                Vertex::new(Vec3::new(1.0, 0.0, 0.0), color, Vec2::new(1.0, 0.0)), //13
                Vertex::new(Vec3::new(1.0, 1.0, 0.0), color, Vec2::new(1.0, 1.0)), //14
                Vertex::new(Vec3::new(1.0, 1.0, 1.0), color, Vec2::new(0.0, 1.0)), //15

                // Bottom Face (y = 0)
                Vertex::new(Vec3::new(0.0, 0.0, 0.0), color, Vec2::new(0.0, 0.0)), //16
                Vertex::new(Vec3::new(1.0, 0.0, 0.0), color, Vec2::new(1.0, 0.0)), //17
                Vertex::new(Vec3::new(1.0, 0.0, 1.0), color, Vec2::new(1.0, 1.0)), //18
                Vertex::new(Vec3::new(0.0, 0.0, 1.0), color, Vec2::new(0.0, 1.0)), //19

                // Top Face (y = 1)
                Vertex::new(Vec3::new(0.0, 1.0, 1.0), color, Vec2::new(0.0, 0.0)), //20
                Vertex::new(Vec3::new(1.0, 1.0, 1.0), color, Vec2::new(1.0, 0.0)), //21
                Vertex::new(Vec3::new(1.0, 1.0, 0.0), color, Vec2::new(1.0, 1.0)), //22
                Vertex::new(Vec3::new(0.0, 1.0, 0.0), color, Vec2::new(0.0, 1.0)), //23
            ],

            indices: vec![
                // Front
                0,  1,  2,
                2,  3,  0,

                // Back
                4,  5,  6,
                6,  7,  4,

                // Left
                8,  9, 10,
                10, 11,  8,

                // Right
                12, 13, 14,
                14, 15, 12,

                // Bottom
                16, 17, 18,
                18, 19, 16,

                // Top
                20, 21, 22,
                22, 23, 20,
            ],

            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    pub fn new_triangle()->Self{
        let color=Vec3::new(0.5,0.5,0.5);
        let root_3 = 3.0f32.sqrt();
        Mesh {
            vertices: vec![
                Vertex::new(Vec3::new(0.0, root_3, 0.0), color, Vec2::new(0.5,0.5)), // 0
                Vertex::new(Vec3::new(-1.0, -1.0/root_3, 0.0), color, Vec2::new(0.0,0.0)), // 1
                Vertex::new(Vec3::new(1.0, -1.0/root_3, 0.0), color, Vec2::new(1.0,1.0)), // 1
            ],

            indices: vec![
                0,1,2,
            ],

            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }
    pub fn gen_perm_vert(n:u8){

    }
    pub fn shared_reference(self)->Arc<Mesh>{
        Arc::new(self)
    }
    pub fn add_vertex(&mut self, vertex: Vertex){
        let n=self.vertices.len();
        self.vertices.push(vertex);
        if n>=2 {
            self.indices.push((n - 1) as u32);
            self.indices.push(n as u32);
            self.indices.push(0);
        }
    }
    pub fn gen_normals(&mut self){
        for v in &mut self.vertices{
            v.normal=Vec3::ZERO;
        }

        for i in (0..self.indices.len()-2).step_by(3){
            let i1=self.indices[i ];
            let i2=self.indices[i+1 ];
            let i3=self.indices[i +2];

            let p1=&self.vertices[i1 as usize].pos;
            let p2=&self.vertices[i2 as usize].pos;
            let p3=&self.vertices[i3  as usize].pos;

            let edge1 = p2 - p1;
            let edge2 = p3 - p1;
            let normal = edge1.cross(edge2);
            self.vertices[i1 as usize].normal+=normal;
            self.vertices[i2 as usize].normal+=normal;
            self.vertices[i3 as usize].normal+=normal;
        }

        for v in &mut self.vertices{
            if v.normal.length_squared() > 0.0 {
                v.normal = v.normal.normalize();
            }
            println!("{:?}", v.normal);
        }
    }
    pub fn compile(&mut self, gen_normals:bool){
        unsafe {
            if gen_normals{
                self.gen_normals();
            }


            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::TRUE,
                size_of::<Vertex>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::TRUE,
                size_of::<Vertex>() as i32,
                std::mem::offset_of!(Vertex, color) as *const _,
            );
            gl::EnableVertexAttribArray(1);


            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::TRUE,
                size_of::<Vertex>() as i32,
                std::mem::offset_of!(Vertex, normal) as *const _,
            );
            gl::EnableVertexAttribArray(2);


            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::TRUE,
                size_of::<Vertex>() as i32,
                std::mem::offset_of!(Vertex,text) as *const _,
            );
            gl::EnableVertexAttribArray(3);


        }
    }
    pub fn get_vao(&self)->GLuint{self.vao}
    pub fn get_vbo(&self)->GLuint{self.vbo}
    pub fn get_ebo(&self)->GLuint{self.ebo}
    
}