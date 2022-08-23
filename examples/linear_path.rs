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
        RoughGenerator::polygon(
            vec![
                pt2(-400.0, -400.0),
                pt2(0.0, 100.0),
                pt2(-450.0, 200.0),
                pt2(330.0, 220.0),
                pt2(450.0, 100.0),
                pt2(300.0, -390.0),
            ],
            options,
        )
        .draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
