use nannou::color::{self, DefaultScalar, LinSrgba};
use nannou::prelude::*;

use nannou::lyon::tessellation::FillRule;

pub type ColorScalar = DefaultScalar;

#[derive(Copy, Clone, Debug)]
pub enum OpType {
    Move,
    BcurveTo,
    LineTo,
}

#[derive(Copy, Clone)]
pub enum OpSetType {
    Path,
    FillPath,
    FillSketch,
}

#[derive(Copy, Clone, Debug)]
pub enum FillStyle {
    Solid,
    Zigzag,
    CrossHatch,
    Dots,
    Dashed,
    ZigzagLine,
    Hachure,
}

use FillStyle::*;
use OpSetType::*;
use OpType::*;

#[derive(Copy, Clone)]
pub struct Options {
    pub roughness: f32,
    pub bowing: f32,
    pub disable_multi_stroke: bool,
    pub max_randomness_offset: f32,
    pub preserve_vertices: bool,
    pub fill: bool,
    pub fill_style: FillStyle,
    pub hachure_angle: f32,
    pub hachure_gap: f32,
    pub stroke_width: f32,
    pub fill_weight: f32,
    pub curve_step_count: usize,
    pub curve_fitting: f32,
    pub curve_tightness: f32,
    pub color: LinSrgba<ColorScalar>,
    pub fill_color: LinSrgba<ColorScalar>,
    pub dash_offset: f32,
    pub dash_gap: f32,
}

impl Options {
    pub fn new() -> Self {
        Options::default()
    }

    pub fn default() -> Self {
        return Options {
            roughness: 1.0,
            bowing: 1.5,
            disable_multi_stroke: false,
            max_randomness_offset: 3.0,
            preserve_vertices: false,
            fill: false,
            fill_style: Solid,
            hachure_angle: 45.0,
            hachure_gap: 10.0,
            stroke_width: 2.0,
            fill_weight: 3.0,
            curve_step_count: 9,
            curve_fitting: 0.95,
            curve_tightness: 0.0,
            color: color::lin_srgba(0.0, 0.0, 0.0, 1.0),
            fill_color: color::lin_srgba(0.0, 0.0, 0.0, 1.0),
            dash_offset: -1.0,
            dash_gap: -1.0,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Op {
    pub op: OpType,
    pub data: Vec<f32>,
}

#[derive(Clone)]
pub struct OpSet {
    pub ops_type: OpSetType,
    pub ops: Vec<Op>,
}

#[derive(Clone)]
pub struct Drawable<'a> {
    pub shape: String,
    pub options: Options,
    pub sets: Vec<OpSet>,
    draw: &'a Draw,
}

impl<'a> Drawable<'a> {
    pub fn new(shape: &str, options: Options, sets: Vec<OpSet>, draw: &'a Draw) -> Self {
        return Drawable {
            shape: shape.to_string(),
            options,
            sets,
            draw,
        };
    }
    pub fn draw(&self) {
        let sets = self.sets.clone();
        for drawing in sets.iter() {
            match drawing.ops_type {
                Path => {
                    let mut builder = nannou::geom::path::Builder::new().with_svg();
                    for item in drawing.ops.iter() {
                        let data = item.data.clone();
                        match item.op {
                            Move => {
                                builder.move_to([data[0], data[1]].into());
                            }
                            BcurveTo => {
                                builder.cubic_bezier_to(
                                    [data[0], data[1]].into(),
                                    [data[2], data[3]].into(),
                                    [data[4], data[5]].into(),
                                );
                            }
                            LineTo => {
                                builder.line_to([data[0], data[1]].into());
                            }
                        }
                    }
                    let path = builder.build();
                    let weight = self.options.stroke_width;
                    self.draw
                        .path()
                        .stroke()
                        .weight(weight)
                        .color(self.options.color)
                        .events(path.iter());
                }
                FillPath => {
                    let mut builder = nannou::geom::path::Builder::new().with_svg();
                    for item in drawing.ops.iter() {
                        let data = item.data.clone();
                        match item.op {
                            Move => {
                                builder.move_to([data[0], data[1]].into());
                            }
                            BcurveTo => {
                                builder.cubic_bezier_to(
                                    [data[0], data[1]].into(),
                                    [data[2], data[3]].into(),
                                    [data[4], data[5]].into(),
                                );
                            }
                            LineTo => {
                                builder.line_to([data[0], data[1]].into());
                            }
                        }
                    }
                    let path = builder.build();
                    //let weight = self.options.fill_weight;
                    let fill_rule = match self.shape.as_str() {
                        "curve" | "polygon" | "path" => FillRule::EvenOdd,
                        _ => FillRule::NonZero,
                    };
                    self.draw
                        .path()
                        .fill()
                        .fill_rule(fill_rule)
                        .color(self.options.fill_color)
                        .events(path.iter());
                }
                FillSketch => {
                    let mut builder = nannou::geom::path::Builder::new().with_svg();
                    for item in drawing.ops.iter() {
                        let data = item.data.clone();
                        match item.op {
                            Move => {
                                builder.move_to([data[0], data[1]].into());
                            }
                            BcurveTo => {
                                builder.cubic_bezier_to(
                                    [data[0], data[1]].into(),
                                    [data[2], data[3]].into(),
                                    [data[4], data[5]].into(),
                                );
                            }
                            LineTo => {
                                builder.line_to([data[0], data[1]].into());
                            }
                        }
                    }
                    let path = builder.build();
                    self.draw
                        .path()
                        .stroke()
                        .weight(self.options.fill_weight)
                        .color(self.options.fill_color)
                        .events(path.iter());
                }
            }
        }
    }
}
