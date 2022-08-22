use nannou::prelude::*;

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
}

impl Options {
    pub fn new() -> Self {
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
        };
    }

    pub fn set_fill_style(&mut self, style: &str) -> &mut Self {
        match style {
            "Solid" => self.fill_style = Solid,
            "Hachure" => self.fill_style = Hachure,
            _ => unimplemented!(),
        }
        return self;
    }

    pub fn set_fill(&mut self) -> &mut Self {
        self.fill = true;
        return self;
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
pub struct Drawable {
    pub shape: String,
    pub options: Options,
    pub sets: Vec<OpSet>,
}

impl Drawable {
    pub fn new(shape: &str, options: Options, sets: Vec<OpSet>) -> Self {
        return Drawable {
            shape: shape.to_string(),
            options,
            sets,
        };
    }
    pub fn draw(&self, draw: &Draw) {
        let sets = self.sets.clone();
        let mut builder = nannou::geom::path::Builder::new().with_svg();
        let mut weight = 3.0;
        for drawing in sets.iter() {
            match drawing.ops_type {
                Path => {
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
                }
                FillPath => {
                    // TODO: fill 対応
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
                }
                FillSketch => {
                    weight = self.options.fill_weight;
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
                }
            }
        }
        let path = builder.build();
        draw.path().stroke().weight(weight).events(path.iter());
    }
}
