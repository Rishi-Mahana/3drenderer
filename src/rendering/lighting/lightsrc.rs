use gl::ZERO;
use crate::model::model::Model;
use glam::Vec3;
pub enum LightMode {
    Directional {
        direction: Vec3,
    },
    Point {
        constant: f32,
        linear: f32,
        quadratic: f32,
    },
    SpotLight{
        inner_cutoff: f32,
        outer_cutoff:f32,
        direction: Vec3,
        constant: f32,
        linear: f32,
        quadratic: f32,
        //IMPLEMENT BEING ABLE TO CHANGE THESE LATER
    }
}
pub struct LightSource<'a>{
    pub diffuse: Vec3,
    pub ambient: Vec3,
    pub specular: Vec3,
    pub model: &'a mut Model,
    pub lightmode: LightMode,
    pub light_specific_coloring: bool,

    //lightingmode
}
impl<'a> LightSource<'a>{
    pub fn directional(model: &'a mut Model) -> Self {
        Self {
            ambient: Vec3::splat(0.2),
            diffuse: Vec3::ONE,
            specular: Vec3::ONE,
            model,
            lightmode: LightMode::Directional {
                direction: Vec3::ZERO,
            },
            light_specific_coloring: true,
        }
    }

    pub fn spotlight(model: &'a mut Model)->Self{
        Self {
            ambient: Vec3::splat(0.2),
            diffuse: Vec3::ONE,
            specular: Vec3::ONE,
            model,
            lightmode: LightMode::SpotLight {
                inner_cutoff: 25.0,
                outer_cutoff: 35.0,
                direction: Vec3::ZERO,
                constant:1.0,
                linear:0.0,
                quadratic:0.0,
            },
            light_specific_coloring: true,
        }
    }

    pub fn with_cutoff(mut self, angle1:f32, angle2:f32)->Self{
        match self. lightmode{
            LightMode::SpotLight {outer_cutoff,inner_cutoff, direction, constant,linear, quadratic} => {
                self.lightmode=LightMode::SpotLight {inner_cutoff:angle1,direction, outer_cutoff:angle2, constant,linear,quadratic};
            }
            _=>{}
        }
        self
    }
    pub fn with_direction(mut self, x:f32,y:f32,z:f32)->Self{
        let dir=Vec3::new(x,y,z);
        match self. lightmode{
            LightMode::SpotLight {outer_cutoff, inner_cutoff, direction,constant,linear,quadratic} => {
                self.lightmode=LightMode::SpotLight {inner_cutoff, outer_cutoff,direction: dir.normalize(),constant,linear,quadratic};
            }
            LightMode::Directional{direction} => {
                self.lightmode=LightMode::Directional {direction:dir.normalize()}
            }
            _=>{}
        }
        self
    }
    pub fn point(model: &'a mut Model) -> Self {
        Self {
            ambient: Vec3::splat(0.2),
            diffuse: Vec3::ONE,
            specular: Vec3::ONE,
            model,
            lightmode: LightMode::Point {
                constant: 1.0,
                linear: 0.007,
                quadratic: 0.0002,
            },
            light_specific_coloring: true,
        }
    }

    pub fn with_range(mut self, range: f32) -> Self {
        match self.lightmode {
            LightMode::Point{constant, linear, quadratic,  } => {
                let (linear, quadratic) = attenuation_from_range(range);

                self.lightmode=LightMode::Point {constant:1.0,linear,quadratic};

            }
            LightMode::SpotLight {inner_cutoff,outer_cutoff,direction,constant,quadratic,linear}=>{
                let (linear, quadratic) = attenuation_from_range(range);
                self.lightmode=LightMode::SpotLight{inner_cutoff,outer_cutoff,direction,constant:1.0,linear,quadratic};

                
            }
            _=>{}
        }


        self
    }

    pub fn with_attenuation(mut self, c: f32, l: f32, q: f32, ) -> Self {
        match self.lightmode {
            LightMode::Point{constant, linear, quadratic} => {
                self.lightmode=LightMode::Point {constant:c,linear:l,quadratic:q};

            }
            LightMode::SpotLight {inner_cutoff,outer_cutoff,direction,constant,linear,quadratic} => {
                self.lightmode=LightMode::SpotLight{constant:c,linear:l,quadratic:q,inner_cutoff,outer_cutoff,direction};

            }
            _=>{}
        }


        self
    }
    pub fn with_constant(mut self, c: f32 ) -> Self {
        match self.lightmode {
            LightMode::SpotLight {inner_cutoff,outer_cutoff,direction,constant,linear,quadratic} => {
                self.lightmode=LightMode::SpotLight{constant:c,linear,quadratic,inner_cutoff,outer_cutoff,direction};

            }
            _=>{}
        }
        self
    }
    pub fn with_linear(mut self, l: f32 ) -> Self {
        match self.lightmode {
            LightMode::SpotLight {inner_cutoff,outer_cutoff,direction,constant,linear,quadratic} => {
                self.lightmode=LightMode::SpotLight{constant,linear:l,quadratic,inner_cutoff,outer_cutoff,direction};

            }
            _=>{}
        }
        self
    }
    pub fn with_quadratic(mut self, q: f32 ) -> Self {
        match self.lightmode {
            LightMode::SpotLight {inner_cutoff,outer_cutoff,direction,constant,linear,quadratic} => {
                self.lightmode=LightMode::SpotLight{constant,linear,quadratic:q,inner_cutoff,outer_cutoff,direction};

            }
            _=>{}
        }
        self
    }
    pub fn set_light_specific_coloring(&mut self, light_specific_coloring:bool){
        self.light_specific_coloring = light_specific_coloring;
    } 
    



    pub fn with_diffuse(mut self, r:f32, g:f32,b:f32)->Self{
        self.diffuse= Vec3::new(r,g,b);
        self
    }
    pub fn with_specular(mut self, r:f32, g:f32,b:f32)->Self{
        self.specular=Vec3::new(r,g,b);
        self
    }
    pub fn with_ambient(mut self, r:f32, g:f32, b:f32)->Self{
        self.ambient=Vec3::new(r,g,b);
        self
    }
    pub fn set_diffuse(&mut self, r:f32, g:f32, b:f32){
        self.diffuse=Vec3::new(r,g,b);
    }
    pub fn set_specular(&mut self, r:f32, g:f32, b:f32){
        self.specular=Vec3::new(r,g,b);
    }
    pub fn set_ambient(&mut self, r:f32, g:f32, b:f32){
        self.ambient=Vec3::new(r,g,b);
    }
    pub fn white(mut self) -> Self {
        self.ambient = Vec3::splat(0.2);
        self.diffuse = Vec3::ONE;
        self.specular = Vec3::ONE;
        self
    }

    pub fn sunlight(mut self) -> Self {
        self.ambient = Vec3::new(0.30, 0.30, 0.28);
        self.diffuse = Vec3::new(1.00, 0.98, 0.90);
        self.specular = Vec3::new(1.00, 1.00, 0.95);
        self
    }

    pub fn candle(mut self) -> Self {
        self.ambient = Vec3::new(0.10, 0.05, 0.02);
        self.diffuse = Vec3::new(1.00, 0.60, 0.20);
        self.specular = Vec3::new(1.00, 0.75, 0.35);
        self
    }

}
fn attenuation_from_range(range: f32) -> (f32, f32) {
    if range <= 7.0 {
        (0.7, 1.8)
    } else if range <= 13.0 {
        (0.35, 0.44)
    } else if range <= 20.0 {
        (0.22, 0.20)
    } else if range <= 32.0 {
        (0.14, 0.07)
    } else if range <= 50.0 {
        (0.09, 0.032)
    } else if range <= 65.0 {
        (0.07, 0.017)
    } else if range <= 100.0 {
        (0.045, 0.0075)
    } else if range <= 160.0 {
        (0.027, 0.0028)
    } else if range <= 200.0 {
        (0.022, 0.0019)
    } else if range <= 325.0 {
        (0.014, 0.0007)
    } else if range <= 600.0 {
        (0.007, 0.0002)
    } else {
        (0.0014, 0.000007)
    }
}