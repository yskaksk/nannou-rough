use nannou::prelude::*;

use crate::core::{Drawable, FillStyle, OpSet, Options};
use crate::renderer::{
    ellipse_with_params, generate_ellipse_params, line, pattern_fill_polygon, rectangle,
    solid_fill_polygon,
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
}
