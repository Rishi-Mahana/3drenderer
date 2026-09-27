use image::GenericImageView;
#[derive(Clone)]
pub struct Texture{
    id:u32,
    width:u32,
    height:u32,

}
pub enum WrapMode{
    Repeat,
    MirroredRepeat,
    EdgeClamp,
    BorderClamp
}
pub enum FilteringMode{
    Nearest,
    Linear,
    Linear_MipMap_Linear,
    Linear_MipMap_Nearest,
    Nearest_MipMap_Nearest,
    Nearest_MipMap_Linear,
}
impl Texture{
    pub fn from_file(path:&str)->Self{
        let img=image::open(path).expect("Failed to load texture").flipv();
        let (w,h)=img.dimensions();
        let data=img.into_rgba8();
        let mut id=0;

        unsafe{
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);


            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);




            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,

            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Self{
                id , width:w,height:h
            }
        }

    }
    pub fn bind(&self, unit:u32){
        unsafe{
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn set_wrapmode_s(&mut self, texmode: WrapMode){
        let spec=match texmode{
            WrapMode::Repeat =>{gl::REPEAT},
            WrapMode::MirroredRepeat =>{gl::MIRRORED_REPEAT},
            WrapMode::EdgeClamp =>{gl::CLAMP_TO_EDGE},
            WrapMode::BorderClamp =>{gl::CLAMP_TO_BORDER},
        };
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, spec as i32);
        }
    }
    pub fn set_wrapmode_t(&mut self, texmode: WrapMode){
        let spec=match texmode{
            WrapMode::Repeat =>{gl::REPEAT},
            WrapMode::MirroredRepeat =>{gl::MIRRORED_REPEAT},
            WrapMode::EdgeClamp =>{gl::CLAMP_TO_EDGE},
            WrapMode::BorderClamp =>{gl::CLAMP_TO_BORDER},
        };
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, spec as i32);
        }
    }
    pub fn set_min_filter(&mut self, filter:FilteringMode){
        let spec = match filter{
            FilteringMode::Nearest =>{gl::NEAREST},
            FilteringMode::Linear =>{gl::LINEAR},
            FilteringMode::Linear_MipMap_Linear =>{gl::LINEAR_MIPMAP_LINEAR},
            FilteringMode::Linear_MipMap_Nearest=>{gl::LINEAR_MIPMAP_NEAREST},
            FilteringMode::Nearest_MipMap_Linear=>{gl::NEAREST_MIPMAP_LINEAR},
            FilteringMode::Nearest_MipMap_Nearest=>{gl::NEAREST_MIPMAP_NEAREST},
        };
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.id );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter as i32);
        }
    }
    pub fn set_mag_filter(&mut self, filter:FilteringMode){
        let spec = match filter{
            FilteringMode::Nearest =>{gl::NEAREST},
            FilteringMode::Linear =>{gl::LINEAR},
            FilteringMode::Linear_MipMap_Linear =>{gl::LINEAR_MIPMAP_LINEAR},
            FilteringMode::Linear_MipMap_Nearest=>{gl::LINEAR_MIPMAP_NEAREST},
            FilteringMode::Nearest_MipMap_Linear=>{gl::NEAREST_MIPMAP_LINEAR},
            FilteringMode::Nearest_MipMap_Nearest=>{gl::NEAREST_MIPMAP_NEAREST},
        };
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D, self.id );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as i32);
        }
    }
    pub fn default()->Self{
        Self{
            id:0,
            width:0,
            height:0,
        }
    }
    // decoded RGBA8 pxels, for gltf images
    pub fn from_rgba8(width: u32, height: u32, pixels: &[u8]) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32, );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32, );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self { id, width, height }
    }
}