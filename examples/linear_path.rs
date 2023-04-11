use nannou::prelude::*;

use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let mut rg = RoughGenerator::new(&draw);
        let n = 5;
        let theta = 2.0 * f32::PI() / (n as f32);
        let points = Vec::from_iter((0..=n).map(|i| {
            let x = 400.0 * ((2 * i % n) as f32 * theta).cos();
            let y = 400.0 * ((2 * i % n) as f32 * theta).sin();
            pt2(x, y)
        }));
        rg.fill()
            .fill_style("Hachure")
            .color(GREEN)
            .fill_color(ORANGE)
            .polygon(points)
            .draw();
    }
    draw.to_frame(app, &frame).unwrap();
}
