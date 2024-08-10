use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    angle: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    Model { angle: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += 0.02;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.ellipse()
        .color(PLUM)
        .w_h(100.0, 100.0)
        .x_y(200.0 * model.angle.cos(), 200.0 * model.angle.sin());
    draw.to_frame(app, &frame).unwrap();
}