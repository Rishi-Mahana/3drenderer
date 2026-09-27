use std::{collections::HashMap, path::Path};
use glam::{Mat4, Vec2, Vec3, Vec4};
use crate::{
    model::{material::{AlphaMode, Material}, mesh::{Mesh, Vertex}, model::Model, transform::Transform},
    rendering::textures::texture::Texture,
};

#[derive(Debug)]
pub enum ObjectError {
    Gltf(gltf::Error),
    NoScene,
    MissingPositions { mesh: usize, primitive: usize },
    UnsupportedPrimitiveMode { mesh: usize, primitive: usize, mode: gltf::mesh::Mode },
    UnsupportedTextureCoordinateSets { mesh: usize, primitive: usize },
    InvalidImageData { image: usize },
}

impl From<gltf::Error> for ObjectError {
    fn from(error: gltf::Error) -> Self { Self::Gltf(error) }
}

pub struct Object {
    pub models: Vec<Model>,
    pub transform: Transform,
}

impl Object {
    pub fn new() -> Self { Self { models: Vec::new(), transform: Transform::new() } }
    pub fn obj_rotate(&mut self, rotation: glam::Quat) {
        self.transform.rotate(rotation);
    }
    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
    pub fn obj_translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.translate(Vec3::new(x, y, z));
    }

    pub fn load_gltf<P: AsRef<Path>>(path: P) -> Result<Self, ObjectError> {
        let (document, buffers, images) = gltf::import(path)?;
        let scene = document.default_scene().or_else(|| document.scenes().next()).ok_or(ObjectError::NoScene)?;
        let scene = match document.default_scene(){
            Some(scene)=>{scene}
            None=>{match document.scenes().next(){
                Some(a)=>{a},
                None=>{return Err(ObjectError::NoScene)}
            }}
        };
        let mut object = Self::new();
        let mut textures = HashMap::new();
        for node in scene.nodes() {
            Self::process_node(node, Mat4::IDENTITY, &buffers, &images, &mut textures, &mut object)?;
        }
        Ok(object)
    }


    pub fn process_node(node: gltf::Node, parent: Mat4, buf: &[gltf::buffer::Data], images: &[gltf::image::Data], textures: &mut HashMap<usize, Texture>, obj: &mut Object, ) -> Result<(), ObjectError> {
        let world = parent * Self::gltf_mat_transform(&node);
        if let Some(mesh) = node.mesh() {
            Self::process_mesh(mesh, world, buf, images, textures, obj)?;
        }
        for child in node.children() {
            Self::process_node(child, world, buf, images, textures, obj)?;
        }
        Ok(())
    }

    fn gltf_mat_transform(node: &gltf::Node) -> Mat4 {
        Mat4::from_cols_array_2d(&node.transform().matrix())
    }

    pub fn process_mesh(mesh: gltf::Mesh, transform: Mat4, buf: &[gltf::buffer::Data], images: &[gltf::image::Data], textures: &mut HashMap<usize, Texture>, obj: &mut Object, ) -> Result<(), ObjectError> {
        for primitive in mesh.primitives() {
            Self::process_primitive(mesh.index(), primitive, transform, buf, images, textures, obj)?;
        }
        Ok(())
    }

    pub fn process_primitive(meshi: usize, prim: gltf::Primitive, transform: Mat4, buf: &[gltf::buffer::Data], images: &[gltf::image::Data], textures: &mut HashMap<usize, Texture>, obj: &mut Object, ) -> Result<(), ObjectError> {
        if prim.mode() != gltf::mesh::Mode::Triangles {
            return Err(ObjectError::UnsupportedPrimitiveMode {
                mesh: meshi, primitive: prim.index(), mode: prim.mode(),
            });
        }

        let material = prim.material();
        let pbr = material.pbr_metallic_roughness();
        let spec_gloss = material.pbr_specular_glossiness();
        let diff_info = match spec_gloss.as_ref() {
            Some(spg) => spg.diffuse_texture().or_else(|| pbr.base_color_texture()),  //if spec gloss exists try to get diff text or js give base color
            None => pbr.base_color_texture(), // if spec gloss doesnt exist
        };
        let spec_info = match spec_gloss.as_ref() {
            None => None,
            Some(spg) => { spg.specular_glossiness_texture() }
        } ;
        let tex_coord_set = diff_info.as_ref().map(|i| i.tex_coord()).or_else(|| spec_info.as_ref().map(|i| i.tex_coord())).unwrap_or(0);

        if diff_info.as_ref().is_some_and(|i| i.tex_coord() != tex_coord_set) //if diffinfo exists and isnt supported by uv set, or spec isnt supported throw error
            || spec_info.as_ref().is_some_and(|i| i.tex_coord() != tex_coord_set)
        {
            return Err(ObjectError::UnsupportedTextureCoordinateSets { mesh: meshi, primitive: prim.index() });
        }

        let reader = prim.reader(|buffer| Some(&buf[buffer.index()].0));
        let positions: Vec<[f32; 3]> = reader.read_positions()
            .ok_or(ObjectError::MissingPositions { mesh: meshi, primitive: prim.index() })?
            .collect();
        let normals: Option<Vec<[f32; 3]>> = reader.read_normals().map(Iterator::collect);
        let texcoords: Option<Vec<[f32; 2]>> = reader.read_tex_coords(tex_coord_set).map(|c| c.into_f32().collect());
        let colors: Option<Vec<[f32; 3]>> = reader.read_colors(0).map(|c| c.into_rgb_f32().collect());
        let indices: Vec<u32> = reader.read_indices().map(|i| i.into_u32().collect()).unwrap_or_else(|| (0..positions.len() as u32).collect());




        let mut vertices= Vec::new();
        for i in (0..positions.len()){
            let pos=Vec3::from_array(positions[i]);
            let normal= normals.as_ref().and_then(|v| v.get(i)).map(|n| Vec3::from_array(*n)).unwrap_or(Vec3::ZERO); //get ith normal from normals list, from_Array it, options style :^
            let text=texcoords.as_ref().and_then(|v| v.get(i)).map(|uv| Vec2::from_array(*uv)).unwrap_or(Vec2::ZERO);
            let color = colors.as_ref().and_then(|v| v.get(i)).map(|c| Vec3::from_array(*c)).unwrap_or(Vec3::ONE);
            let newvert= Vertex{pos, normal, text, color};
            vertices.push(newvert);
        }

        let mut mesh = Mesh::new().with_vertices(vertices).with_indices(indices);
        if normals.is_none() { mesh.gen_normals(); }
        mesh.compile(false);

        let mut model_material = Self::process_material(&prim);
        if let Some(info) = diff_info {
            let texture = Self::load_texture(info.texture().source().index(), images, textures)?;
            model_material = model_material.with_diffusemap(texture);
        }
        if let Some(info) = spec_info {
            let texture = Self::load_texture(info.texture().source().index(), images, textures)?;
            model_material = model_material.with_specularmap(texture);
        }

        obj.add_model(Model::new()
            .with_mesh(mesh)
            .with_material(model_material)
            .with_transform(Self::mat4_to_transform(transform)));
        Ok(())
    }

    fn process_material(prim: &gltf::Primitive) -> Material {
        let material = prim.material();
        let alpha_mode = match material.alpha_mode() {
            gltf::material::AlphaMode::Opaque => AlphaMode::Opaque,
            gltf::material::AlphaMode::Mask => AlphaMode::Mask(material.alpha_cutoff().unwrap_or(0.5)),
            gltf::material::AlphaMode::Blend => AlphaMode::Blend,
        };

        if let Some(spec_gloss) = material.pbr_specular_glossiness() {
            let diffuse = Vec4::from_array(spec_gloss.diffuse_factor());
            return Material::new()
                .with_ambient(diffuse.truncate() * 0.2)
                .with_diffuse(diffuse)
                .with_specular(Vec3::from_array(spec_gloss.specular_factor()))
                .with_shininess(2.0 + spec_gloss.glossiness_factor() * 126.0)
                .with_alpha_mode(alpha_mode);
        }

        let pbr = material.pbr_metallic_roughness();
        let diffuse = Vec4::from_array(pbr.base_color_factor());
        Material::new()
            .with_ambient(diffuse.truncate() * 0.2)
            .with_diffuse(diffuse)
            .with_specular(Vec3::splat(0.04).lerp(diffuse.truncate(), pbr.metallic_factor()))
            .with_shininess(2.0 + (1.0 - pbr.roughness_factor()) * 126.0)
            .with_alpha_mode(alpha_mode)
    }

    fn load_texture(imagei: usize, images: &[gltf::image::Data], texts: &mut HashMap<usize, Texture>, ) -> Result<Texture, ObjectError> {
        if let Some(texture) = texts.get(&imagei) { return Ok(texture.clone()); }
        let image = images.get(imagei).ok_or(ObjectError::InvalidImageData { image: imagei})?;
        let texture = Texture::from_rgba8(image.width, image.height, &Self::image_to_rgba8(image, imagei)?);
        texts.insert(imagei, texture.clone());
        Ok(texture)
    }

    fn image_to_rgba8(image: &gltf::image::Data, image_index: usize) -> Result<Vec<u8>, ObjectError> {
        use image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb, Rgba};

        let (width, height) = (image.width, image.height);
        let invalid = || ObjectError::InvalidImageData { image: image_index };
        let decoded = match image.format {
            gltf::image::Format::R8 => DynamicImage::ImageLuma8(ImageBuffer::<Luma<u8>, _>::from_raw(width, height, image.pixels.clone()).ok_or_else(invalid)?),
            gltf::image::Format::R8G8 => DynamicImage::ImageLumaA8(ImageBuffer::<LumaA<u8>, _>::from_raw(width, height, image.pixels.clone()).ok_or_else(invalid)?),
            gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, image.pixels.clone()).ok_or_else(invalid)?),
            gltf::image::Format::R8G8B8A8 => DynamicImage::ImageRgba8(ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, image.pixels.clone()).ok_or_else(invalid)?),
            gltf::image::Format::R16 => DynamicImage::ImageLuma16(ImageBuffer::<Luma<u16>, _>::from_raw(width, height, Self::bytes_to_u16(&image.pixels)).ok_or_else(invalid)?),
            gltf::image::Format::R16G16 => DynamicImage::ImageLumaA16(ImageBuffer::<LumaA<u16>, _>::from_raw(width, height, Self::bytes_to_u16(&image.pixels)).ok_or_else(invalid)?),
            gltf::image::Format::R16G16B16 => DynamicImage::ImageRgb16(ImageBuffer::<Rgb<u16>, _>::from_raw(width, height, Self::bytes_to_u16(&image.pixels)).ok_or_else(invalid)?),
            gltf::image::Format::R16G16B16A16 => DynamicImage::ImageRgba16(ImageBuffer::<Rgba<u16>, _>::from_raw(width, height, Self::bytes_to_u16(&image.pixels)).ok_or_else(invalid)?),
            gltf::image::Format::R32G32B32FLOAT => DynamicImage::ImageRgb32F(ImageBuffer::<Rgb<f32>, _>::from_raw(width, height, Self::bytes_to_f32(&image.pixels)).ok_or_else(invalid)?),
            gltf::image::Format::R32G32B32A32FLOAT => DynamicImage::ImageRgba32F(ImageBuffer::<Rgba<f32>, _>::from_raw(width, height, Self::bytes_to_f32(&image.pixels)).ok_or_else(invalid)?),
        };
        Ok(decoded.to_rgba8().into_raw())
    }

    fn bytes_to_u16(bytes: &[u8]) -> Vec<u16> {
        bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect()
    }

    fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
        bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
    }

    fn mat4_to_transform(matrix: Mat4) -> Transform {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Transform { position: translation, rotation, scale }
    }
}