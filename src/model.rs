use nannou::prelude::*;

pub struct Model {
    pub window_id: window::Id,
    // Add other fields as necessary
}

impl Model {
    pub fn new(window_id: window::Id) -> Self {
        Self { window_id }
    }

    pub fn update(&mut self, _app: &App, _update: Update) {
        // Add your update logic here
    }

    pub fn view(&self, _app: &App, _draw: &Draw, _frame: Frame) {
        // Add your drawing logic here
        _draw.ellipse().color(STEELBLUE).w_h(100.0, 100.0).xy(vec2(0.0, 0.0));
    }
}