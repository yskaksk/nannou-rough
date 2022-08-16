use nannou::prelude::*;

use crate::core::{Op, OpSet, OpSetType, OpType, Options};
use OpSetType::*;
use OpType::*;

pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, o: &Options) -> OpSet {
    OpSet {
        ops_type: Path,
        ops: _double_line(x1, y1, x2, y2, o),
    }
}

pub fn _double_line(x1: f32, y1: f32, x2: f32, y2: f32, o: &Options) -> Vec<Op> {
    let single_stroke = o.disable_multi_stroke;
    let o1 = _line(x1, y1, x2, y2, o, true, false);
    if single_stroke {
        return o1;
    }
    let o2 = _line(x1, y1, x2, y2, o, true, true);
    return vec![o1, o2].concat();
}

fn _line(x1: f32, y1: f32, x2: f32, y2: f32, o: &Options, ismove: bool, overlay: bool) -> Vec<Op> {
    let length_sq = pt2(x1, y1).distance_squared(pt2(x2, y2));
    let length = length_sq.sqrt();
    let roughness_gain = if length < 200.0 {
        1.0
    } else if length > 500.0 {
        0.4
    } else {
        (-0.0016668) * length + 1.233334
    };
    let mut offset = o.max_randomness_offset;
    if (offset.powi(2) * 100.0) > length_sq {
        offset = length / 100.0;
    }
    let half_offset = offset * 0.5;
    let diverge_point = random_range::<f32>(0.2, 0.4);
    let mut mid_disp_x = o.bowing * o.max_randomness_offset * (y2 - y1) / 200.0;
    mid_disp_x = _offset_opt(mid_disp_x, o, roughness_gain);
    let mut mid_disp_y = o.bowing * o.max_randomness_offset * (x1 - x2) / 200.0;
    mid_disp_y = _offset_opt(mid_disp_y, o, roughness_gain);

    let random_half = || -> f32 { _offset_opt(half_offset, o, roughness_gain) };
    let random_full = || -> f32 { _offset_opt(offset, o, roughness_gain) };
    let pv = o.preserve_vertices;

    let mut ops: Vec<Op> = vec![];
    if ismove {
        if overlay {
            ops.push(Op {
                op: Move,
                data: vec![
                    x1 + if pv { 0.0 } else { random_half() },
                    y1 + if pv { 0.0 } else { random_half() },
                ],
            });
        } else {
            ops.push(Op {
                op: Move,
                data: vec![
                    x1 + if pv { 0.0 } else { random_full() },
                    y1 + if pv { 0.0 } else { random_full() },
                ],
            });
        }
    }
    if overlay {
        ops.push(Op {
            op: BcurveTo,
            data: vec![
                mid_disp_x + x1 + (x2 - x1) * diverge_point + random_half(),
                mid_disp_y + y1 + (y2 - y1) * diverge_point + random_half(),
                mid_disp_x + x1 + 2.0 * (x2 - x1) * diverge_point + random_half(),
                mid_disp_y + y1 + 2.0 * (y2 - y1) * diverge_point + random_half(),
                x2 + if pv { 0.0 } else { random_half() },
                y2 + if pv { 0.0 } else { random_half() },
            ],
        });
    } else {
        ops.push(Op {
            op: BcurveTo,
            data: vec![
                mid_disp_x + x1 + (x2 - x1) * diverge_point + random_full(),
                mid_disp_y + y1 + (y2 - y1) * diverge_point + random_full(),
                mid_disp_x + x1 + 2.0 * (x2 - x1) * diverge_point + random_full(),
                mid_disp_y + y1 + 2.0 * (y2 - y1) * diverge_point + random_full(),
                x2 + if pv { 0.0 } else { random_full() },
                y2 + if pv { 0.0 } else { random_full() },
            ],
        });
    }
    ops
}

fn _offset(min: f32, max: f32, ops: &Options, roughness_gain: f32) -> f32 {
    ops.roughness * roughness_gain * (random_f32() * (max - min) + min)
}

fn _offset_opt(x: f32, ops: &Options, roughness_gain: f32) -> f32 {
    _offset(-x, x, ops, roughness_gain)
}
