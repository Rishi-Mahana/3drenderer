mod model;
mod rendering;

use std::{env, path::PathBuf};

use glam::{Quat, Vec3};
use glfw::{Action, CursorMode, Key, WindowEvent};

use model::loading::object::Object;
use model::model::Model;
use rendering::camera::{Camera, PerspectiveMode};
use rendering::lighting::lightsrc::LightSource;
use rendering::renderer::{BlendFactor, Renderer};
use rendering::shaders::{Shader, ShaderType};
use rendering::window::{Window, WindowMode};

const MOUSE_SENSITIVITY: f32 = 0.0025;
const CAMERA_MOVE_SPEED: f32 = 4.0;
const MODEL_ROTATION_SPEED: f32 = 0.45;

fn main() {
    let model_path = env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/gltfmodels/skull_salazar_downloadable.glb")
    });

    let mut window = Window::new(720, 960, "furry dog kisser", WindowMode::Windowed);
    let mut renderer = Renderer::new(&mut window);
    renderer.set_depth_test(true);
    renderer.set_blend_func(BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha);

    let mut scene = match Object::load_gltf(&model_path) {
        Ok(scene) => scene,
        Err(error) => {
            eprintln!("Could not load '{}': {error:?}", model_path.display());
            return;
        }
    };

    println!("Loaded '{}' ({} mesh primitives)", model_path.display(), scene.models.len());

    let mut camera = Camera::new()
        .with_position(Vec3::new(0.0, 0.0, 5.0))
        .with_perspective(PerspectiveMode::Perspective(
            45.0_f32.to_radians(),
            window.get_aspect_ratio(),
            0.1,
            1000.0,
        ));

    window.set_mouse_polling(true, CursorMode::Disabled);

    let mut key_model = Model::new();
    let key = LightSource::directional(&mut key_model)
        .with_direction(-0.5, -0.6, 1.0)
        .with_ambient(0.08, 0.07, 0.06)
        .with_diffuse(0.95, 0.82, 0.68)
        .with_specular(0.8, 0.75, 0.7);

    let mut fill_model = Model::new();
    let fill = LightSource::directional(&mut fill_model)
        .with_direction(0.8, 0.1, 0.6)
        .with_ambient(0.04, 0.05, 0.07)
        .with_diffuse(0.32, 0.42, 0.58)
        .with_specular(0.25, 0.3, 0.4);

    let mut rim_model = Model::new();
    let rim = LightSource::directional(&mut rim_model)
        .with_direction(0.1, 0.7, -1.0)
        .with_ambient(0.02, 0.02, 0.025)
        .with_diffuse(0.28, 0.32, 0.42)
        .with_specular(0.35, 0.4, 0.5);

    let lights = [key, fill, rim];
    let mut shader = Shader::new(ShaderType::Lighting);
    shader.set_interpolation(false);
    shader.set_lights(&lights);

    let mut last_time = window.get_time();
    let mut last_cursor: Option<(f64, f64)> = None;

    while window.is_open() {
        let now = window.get_time();
        let dt = (now - last_time).clamp(0.0, 0.1) as f32;
        last_time = now;

        for (_, event) in window.poll_events() {
            match event {
                WindowEvent::Close
                | WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.window.set_should_close(true);
                }
                WindowEvent::CursorPos(x, y) => {
                    if let Some((last_x, last_y)) = last_cursor {
                        camera.follow_cursor(
                            (x - last_x) as f32,
                            (y - last_y) as f32,
                            MOUSE_SENSITIVITY,
                        );
                    }
                    last_cursor = Some((x, y));
                }
                _ => {}
            }
        }

        let step = CAMERA_MOVE_SPEED * dt;
        if window.window.get_key(Key::W) == Action::Press { camera.translate(0.0, 0.0, step); }
        if window.window.get_key(Key::S) == Action::Press { camera.translate(0.0, 0.0, -step); }
        if window.window.get_key(Key::A) == Action::Press { camera.translate(-step, 0.0, 0.0); }
        if window.window.get_key(Key::D) == Action::Press { camera.translate(step, 0.0, 0.0); }
        if window.window.get_key(Key::Q) == Action::Press { camera.translate(0.0, -step, 0.0); }
        if window.window.get_key(Key::E) == Action::Press { camera.translate(0.0, step, 0.0); }

        //scene.obj_rotate(Quat::from_rotation_y(MODEL_ROTATION_SPEED * dt));

        renderer.begin_frame(0.22, 0.22, 0.22, 1.0);
        renderer.draw_object(&scene, &mut shader, &camera);
        window.update();
    }
}