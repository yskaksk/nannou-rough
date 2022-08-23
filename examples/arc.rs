use nannou::prelude::*;

use nannou_rough::core::Options;
use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let mut options = Options::new();
        options.set_fill().set_fill_style("Hachure");
        RoughGenerator::arc(0.0, 0.0, 800.0, 500.0, 0.0, f32::PI() * 1.8, true, options)
            .draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
