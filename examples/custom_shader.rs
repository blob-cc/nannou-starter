use nannou::prelude::*;
use nannou::wgpu::ShaderModule;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    shader: ShaderModule,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    let vs = app.assets_path().unwrap().join("shaders/basic.vert");
    let fs = app.assets_path().unwrap().join("shaders/basic.frag");
    let shader = app
        .shader_path()
        .with_vertex(vs)
        .with_fragment(fs)
        .build()
        .unwrap();

    Model { shader }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    // Example of using the shader in your drawing
    draw.quad()
        .shader(&model.shader)
        .x_y(0.0, 0.0)
        .w_h(200.0, 200.0);

    draw.to_frame(_app, &frame).unwrap();
}