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
        let theta = 2.0 * f32::PI() / 5.0;
        let points = Vec::from_iter((0..=5).map(|i| {
            let x = 400.0 * ((2 * i + 1) as f32 * theta).cos();
            let y = 400.0 * ((2 * i + 1) as f32 * theta).sin();
            pt2(x, y)
        }));
        rg.fill()
            .fill_style("Solid")
            .color(BLACK)
            .fill_color(ORANGE)
            .polygon(points)
            .draw();
    }
    draw.to_frame(app, &frame).unwrap();
}
