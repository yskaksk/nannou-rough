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
        let options = Options::new();
        for i in 0..10 {
            let x = map_range(i, 0, 9, -450.0, 450.0);
            let y = map_range(i, 0, 9, -450.0, 450.0);
            RoughGenerator::line(x, -450.0, x, 450.0, options).draw(&draw);
            RoughGenerator::line(-450.0, y, 450.0, y, options).draw(&draw);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
