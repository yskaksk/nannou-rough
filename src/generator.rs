use nannou::prelude::*;

use crate::core::{Drawable, FillStyle, OpSet, Options};
use crate::renderer::{line, pattern_fill_polygon, rectangle, solid_fill_polygon};

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
}
