use nannou::prelude::*;
use nannou::wgpu::{ShaderModule, Device};

pub fn load_shader(app: &App, device: &Device, vertex_path: &str, fragment_path: &str) -> ShaderModule {
    let vs_path = app.assets_path().unwrap().join("shaders").join(vertex_path);
    let fs_path = app.assets_path().unwrap().join("shaders").join(fragment_path);

    let shader = ShaderModule::from_files(device, vs_path, fs_path).unwrap();
    shader
}