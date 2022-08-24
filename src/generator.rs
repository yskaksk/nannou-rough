use nannou::prelude::*;

use crate::core::{Drawable, FillStyle, OpSet, Options};
use crate::renderer::{
    arc, ellipse_with_params, generate_ellipse_params, line, linear_path, pattern_fill_arc,
    pattern_fill_polygon, rectangle, solid_fill_polygon,
};

pub struct RoughGenerator {}

impl RoughGenerator {
    pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, options: Options) -> Drawable {
        Drawable::new("line", options, vec![line(x1, y1, x2, y2, &options)])
    }

    pub fn rectangle(x: f32, y: f32, width: f32, height: f32, options: Options) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = rectangle(x, y, width, height, &options);
        if options.fill {
            let points = vec![
                pt2(x, y),
                pt2(x + width, y),
                pt2(x + width, y + height),
                pt2(x, y + height),
            ];
            match options.fill_style {
                FillStyle::Solid => {
                    paths.push(solid_fill_polygon(vec![points], &options));
                }
                _ => {
                    paths.push(pattern_fill_polygon(vec![points], &options));
                }
            }
        }
        paths.push(outline);
        Drawable::new("rectangle", options, paths)
    }

    pub fn ellipse(x: f32, y: f32, width: f32, height: f32, options: Options) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let ellipse_params = generate_ellipse_params(width, height, &options);
        let ellipse_response = ellipse_with_params(x, y, &options, ellipse_params);
        if options.fill {
            match options.fill_style {
                FillStyle::Solid => {
                    let shape = ellipse_with_params(x, y, &options, ellipse_params).opset;
                    paths.push(shape);
                }
                _ => {
                    let shape =
                        pattern_fill_polygon(vec![ellipse_response.estimated_points], &options);
                    paths.push(shape);
                }
            }
        }
        paths.push(ellipse_response.opset);
        return Drawable::new("ellipse", options, paths);
    }

    pub fn linear_path(points: Vec<Point2>, options: Options) -> Drawable {
        let path = linear_path(points, false, &options);
        return Drawable::new("linear_path", options, vec![path]);
    }

    pub fn polygon(points: Vec<Point2>, options: Options) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = linear_path(points.clone(), true, &options);
        if options.fill {
            match options.fill_style {
                FillStyle::Solid => paths.push(solid_fill_polygon(vec![points.clone()], &options)),
                _ => paths.push(pattern_fill_polygon(vec![points.clone()], &options)),
            }
        }
        // TODO: add options.stroke
        paths.push(outline);
        return Drawable::new("polygon", options, paths);
    }

    pub fn arc(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        start: f32,
        stop: f32,
        closed: bool,
        options: Options,
    ) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = arc(x, y, width, height, start, stop, closed, true, &options);
        if closed && options.fill {
            match options.fill_style {
                FillStyle::Solid => {
                    unimplemented!()
                }
                _ => {
                    paths.push(pattern_fill_arc(x, y, width, height, start, stop, &options));
                }
            }
        }
        // TODO: stroke
        paths.push(outline);
        return Drawable::new("arc", options, paths);
    }
}
