use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model;

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.ellipse().color(STEELBLUE).w_h(200.0, 200.0).xy(app.mouse.position());
    draw.to_frame(app, &frame).unwrap();
}