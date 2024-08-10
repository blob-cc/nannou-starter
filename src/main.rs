mod model;
mod utils;

use nannou::prelude::*;
use model::Model;
use utils::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(800, 600).view(view).build().unwrap();
    Model::new(window_id)
}

fn update(app: &App, model: &mut Model, update: Update) {
    // Update logic
    model.update(app, update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // Call the view logic from the model
    model.view(app, &draw, frame);

    // Write to the frame
    draw.to_frame(app, &frame).unwrap();
}