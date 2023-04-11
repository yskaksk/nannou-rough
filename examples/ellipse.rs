use nannou::prelude::*;

use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() % 10 == 0 {
        draw.background().color(BEIGE);
        let d = 250.0;
        let r = 400.0;
        let mut rg = RoughGenerator::new(&draw);
        rg.fill().fill_style("ZigZag");
        rg.ellipse(-d, d, r, r).draw();
        rg.ellipse(-d, -d, r, r).draw();
        rg.ellipse(d, -d, r, r).draw();
        rg.ellipse(d, d, r, r).draw();
    }
    draw.to_frame(app, &frame).unwrap();
}
