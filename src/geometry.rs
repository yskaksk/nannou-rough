use nannou::prelude::*;

pub struct Line {
    start: Point2,
    end: Point2,
}

impl Line {
    fn rotate(&self, center: Point2, degrees: f32) -> Self {
        let angle = (f32::PI() / 180.0) * degrees;
        Line {
            start: (self.start - center).rotate(angle) + center,
            end: (self.end - center).rotate(angle) + center,
        }
    }

    fn length(&self) -> f32 {
        (self.start - self.end).length()
    }
}

pub fn rotate_points(points: Vec<Point2>, center: Point2, degrees: f32) -> Vec<Point2> {
    let angle = (f32::PI() / 180.0) * degrees;
    return Vec::from_iter(points.iter().map(|p| (*p - center).rotate(angle) + center));
}

pub fn rotate_lines(lines: Vec<Line>, center: Point2, degrees: f32) -> Vec<Line> {
    return Vec::from_iter(lines.iter().map(|l| l.rotate(center, degrees)));
}
