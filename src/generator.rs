use nannou::color::IntoLinSrgba;
use nannou::prelude::*;

use crate::core::{ColorScalar, Drawable, FillStyle, OpSet, OpSetType, Options};
use crate::renderer::{
    arc, ellipse_with_params, generate_ellipse_params, line, linear_path, pattern_fill_arc,
    pattern_fill_polygon, rectangle, solid_fill_polygon,
};

pub struct RoughGenerator<'a> {
    options: Options,
    draw: &'a Draw,
}

impl<'a> RoughGenerator<'a> {
    pub fn new(draw: &'a Draw) -> Self {
        return Self {
            options: Options::default(),
            draw,
        };
    }
    pub fn fill_style(&mut self, style: &str) -> &mut Self {
        match style {
            "Solid" => self.options.fill_style = FillStyle::Solid,
            "Hachure" => self.options.fill_style = FillStyle::Hachure,
            "ZigZag" => self.options.fill_style = FillStyle::Zigzag,
            _ => unimplemented!(),
        }
        return self;
    }

    pub fn stroke_width(&mut self, width: f32) -> &mut Self {
        self.options.stroke_width = width;
        return self;
    }

    pub fn fill_weight(&mut self, weight: f32) -> &mut Self {
        self.options.fill_weight = weight;
        return self;
    }

    pub fn fill(&mut self) -> &mut Self {
        self.options.fill = true;
        return self;
    }

    pub fn curve_step_count(&mut self, curve_step_count: usize) -> &mut Self {
        self.options.curve_step_count = curve_step_count;
        return self;
    }

    pub fn color<C>(&mut self, color: C) -> &mut Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.options.color = color.into_lin_srgba();
        return self;
    }

    pub fn fill_color<C>(&mut self, color: C) -> &mut Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.options.fill_color = color.into_lin_srgba();
        return self;
    }

    pub fn line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Drawable {
        Drawable::new(
            "line",
            self.options,
            vec![line(x1, y1, x2, y2, &self.options)],
            self.draw,
        )
    }

    pub fn rectangle(&self, x: f32, y: f32, width: f32, height: f32) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = rectangle(x, y, width, height, &self.options);
        if self.options.fill {
            let points = vec![
                pt2(x, y),
                pt2(x + width, y),
                pt2(x + width, y + height),
                pt2(x, y + height),
            ];
            match self.options.fill_style {
                FillStyle::Solid => {
                    paths.push(solid_fill_polygon(vec![points], &self.options));
                }
                _ => {
                    paths.push(pattern_fill_polygon(vec![points], &self.options));
                }
            }
        }
        paths.push(outline);
        Drawable::new("rectangle", self.options, paths, self.draw)
    }

    pub fn ellipse(&self, x: f32, y: f32, width: f32, height: f32) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let ellipse_params = generate_ellipse_params(width, height, &self.options);
        let ellipse_response = ellipse_with_params(x, y, &self.options, ellipse_params);
        if self.options.fill {
            match self.options.fill_style {
                FillStyle::Solid => {
                    let mut shape = ellipse_with_params(x, y, &self.options, ellipse_params).opset;
                    shape.ops_type = OpSetType::FillPath;
                    paths.push(shape);
                }
                _ => {
                    let shape = pattern_fill_polygon(
                        vec![ellipse_response.estimated_points],
                        &self.options,
                    );
                    paths.push(shape);
                }
            }
        }
        paths.push(ellipse_response.opset);
        return Drawable::new("ellipse", self.options, paths, self.draw);
    }

    pub fn linear_path(&self, points: Vec<Point2>) -> Drawable {
        let path = linear_path(points, false, &self.options);
        return Drawable::new("linear_path", self.options, vec![path], self.draw);
    }

    pub fn polygon(&self, points: Vec<Point2>) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = linear_path(points.clone(), true, &self.options);
        if self.options.fill {
            match self.options.fill_style {
                FillStyle::Solid => {
                    paths.push(solid_fill_polygon(vec![points.clone()], &self.options))
                }
                _ => paths.push(pattern_fill_polygon(vec![points.clone()], &self.options)),
            }
        }
        // TODO: add options.stroke
        paths.push(outline);
        return Drawable::new("polygon", self.options, paths, self.draw);
    }

    pub fn arc(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        start: f32,
        stop: f32,
        closed: bool,
    ) -> Drawable {
        let mut paths: Vec<OpSet> = vec![];
        let outline = arc(
            x,
            y,
            width,
            height,
            start,
            stop,
            closed,
            true,
            &self.options,
        );
        if closed && self.options.fill {
            match self.options.fill_style {
                FillStyle::Solid => {
                    unimplemented!()
                }
                _ => {
                    paths.push(pattern_fill_arc(
                        x,
                        y,
                        width,
                        height,
                        start,
                        stop,
                        &self.options,
                    ));
                }
            }
        }
        // TODO: stroke
        paths.push(outline);
        return Drawable::new("arc", self.options, paths, self.draw);
    }
}
