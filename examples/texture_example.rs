use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    let assets = app.assets_path().unwrap();
    let texture_path = assets.join("textures").join("texture.png");
    let texture = wgpu::Texture::from_path(app, texture_path).unwrap();
    Model { texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.texture(&model.texture).wh(frame.rect().wh());
    draw.to_frame(app, &frame).unwrap();
}