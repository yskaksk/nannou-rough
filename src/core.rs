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

#[derive(Copy, Clone)]
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
            hachure_gap: 0.0,
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
                    unimplemented!()
                }
                FillSketch => {
                    unimplemented!()
                }
            }
        }
        let path = builder.build();
        draw.path().stroke().events(path.iter());
    }
}
