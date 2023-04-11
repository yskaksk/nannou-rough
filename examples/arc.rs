use nannou::prelude::*;

use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let t = app.time * 100.0;
    if frame.nth() % 10 == 0 {
        let r = t % 300.0;
        let r2 = t % 150.0;
        let d = if r == r2 { r } else { 300.0 - r };
        let th = f32::PI() * 0.25 * map_range(d, 0.0, 150.0, 0.0, 1.0);
        draw.background().color(FLORALWHITE);
        let mut rg = RoughGenerator::new(&draw);
        rg.fill()
            .fill_style("Hachure")
            .curve_step_count(15)
            .color(GOLD)
            .fill_color(GOLD)
            .arc(0.0, 0.0, 600.0, 600.0, th, 2.0 * f32::PI() - th, true)
            .draw();
    }
    draw.to_frame(app, &frame).unwrap();
}
