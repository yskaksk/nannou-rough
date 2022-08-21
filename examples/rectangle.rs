use nannou::prelude::*;

use nannou_rough::core::Options;
use nannou_rough::generator::RoughGenerator;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    if frame.nth() % 10 == 0 {
        draw.background().color(BEIGE);
        let mut options = Options::new();
        options.set_fill().set_fill_style("Hachure");
        let n = 8;
        for i in 0..n {
            for j in 0..n {
                if (i + j) % 2 == 1 {
                    let mut x = map_range(i, 0, n, win.left(), win.right());
                    let mut y = map_range(j, 0, n, win.bottom(), win.top());
                    if x > win.right() {
                        x -= win.w();
                    }
                    if x < win.left() {
                        x += win.w();
                    }
                    if y > win.top() {
                        y -= win.h();
                    }
                    if y < win.bottom() {
                        y += win.h();
                    }
                    RoughGenerator::rectangle(
                        x,
                        y,
                        0.9 * win.w() / n as f32,
                        0.9 * win.h() / n as f32,
                        options,
                    )
                    .draw(&draw);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
