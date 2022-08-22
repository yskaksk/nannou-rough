use nannou::prelude::*;

use nannou_rough::core::Options;
use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() % 10 == 0 {
        draw.background().color(BEIGE);
        let mut options = Options::new();
        options.set_fill().set_fill_style("Hachure");
        let d = 250.0;
        let r = 400.0;
        RoughGenerator::ellipse(-d, d, r, r, options).draw(&draw);
        RoughGenerator::ellipse(-d, -d, r, r, options).draw(&draw);
        RoughGenerator::ellipse(d, -d, r, r, options).draw(&draw);
        RoughGenerator::ellipse(d, d, r, r, options).draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
