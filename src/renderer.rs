use nannou::prelude::*;

use crate::core::{FillStyle, Op, OpSet, OpSetType, OpType, Options};
use crate::filler::get_filler;
use OpSetType::*;
use OpType::*;

pub fn line(x1: f32, y1: f32, x2: f32, y2: f32, o: &Options) -> OpSet {
    OpSet {
        ops_type: Path,
        ops: _double_line(x1, y1, x2, y2, o),
    }
}

pub fn rectangle(x: f32, y: f32, width: f32, height: f32, options: &Options) -> OpSet {
    let points = vec![
        pt2(x, y),
        pt2(x + width, y),
        pt2(x + width, y + height),
        pt2(x, y + height),
    ];
    return polygon(points, options);
}

fn polygon(points: Vec<Point2>, options: &Options) -> OpSet {
    return linear_path(points, true, options);
}

pub fn solid_fill_polygon(polygon_list: Vec<Vec<Point2>>, options: &Options) -> OpSet {
    let mut ops = vec![];
    for points in polygon_list.iter() {
        if points.len() > 2 {
            let offset = options.max_randomness_offset;
            ops.push(Op {
                op: Move,
                data: vec![
                    points[0].x + _offset_opt(offset, options, 1.0),
                    points[0].y + _offset_opt(offset, options, 1.0),
                ],
            });
            for i in 1..points.len() {
                ops.push(Op {
                    op: LineTo,
                    data: vec![
                        points[i].x + _offset_opt(offset, options, 1.0),
                        points[i].y + _offset_opt(offset, options, 1.0),
                    ],
                })
            }
        }
    }
    OpSet {
        ops_type: FillPath,
        ops,
    }
}

pub fn pattern_fill_polygon(polygon_list: Vec<Vec<Point2>>, options: &Options) -> OpSet {
    return get_filler(options).fill_polygons(polygon_list, options);
}

pub fn linear_path(points: Vec<Point2>, close: bool, options: &Options) -> OpSet {
    let len = points.len();
    if len > 2 {
        let mut ops = vec![];
        for i in 0..(len - 1) {
            ops.extend(_double_line(
                points[i].x,
                points[i].y,
                points[i + 1].x,
                points[i + 1].y,
                options,
            ));
        }
        if close {
            ops.extend(_double_line(
                points[len - 1].x,
                points[len - 1].y,
                points[0].x,
                points[0].y,
                options,
            ));
        }
        return OpSet {
            ops_type: Path,
            ops,
        };
    } else if len == 2 {
        return line(points[0].x, points[0].y, points[1].x, points[1].y, options);
    }
    return OpSet {
        ops_type: Path,
        ops: vec![],
    };
}

pub fn arc(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    start: f32,
    stop: f32,
    closed: bool,
    rough_closure: bool,
    options: &Options,
) -> OpSet {
    let cx = x;
    let cy = y;
    let mut rx = (width / 2.0).abs();
    let mut ry = (height / 2.0).abs();
    rx += _offset_opt(rx * 0.01, options, 1.0);
    ry += _offset_opt(ry * 0.01, options, 1.0);
    let mut start = start;
    let mut stop = stop;
    while start < 0.0 {
        start += f32::PI() * 2.0;
        stop += f32::PI() * 2.0;
    }
    if (stop - start) > f32::PI() * 2.0 {
        start = 0.0;
        stop = f32::PI() * 2.0;
    }
    let ellipse_inc = f32::PI() * 2.0 / options.curve_step_count as f32;
    let arc_inc = (ellipse_inc / 2.0).min((stop - start) / 2.0);
    let mut ops = _arc(arc_inc, cx, cy, rx, ry, start, stop, 1.0, options);
    if !options.disable_multi_stroke {
        let ops2 = _arc(arc_inc, cx, cy, rx, ry, start, stop, 1.5, options);
        ops.extend(ops2);
    }
    if closed {
        if rough_closure {
            ops.extend(_double_line(
                cx,
                cy,
                cx + rx * start.cos(),
                cy + ry * start.sin(),
                options,
            ));
            ops.extend(_double_line(
                cx,
                cy,
                cx + rx * stop.cos(),
                cy + ry * stop.sin(),
                options,
            ));
        } else {
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![cx, cy],
            });
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![cx + rx * start.cos(), cy + ry * start.sin()],
            });
        }
    }
    return OpSet {
        ops_type: OpSetType::Path,
        ops,
    };
}

fn _arc(
    increment: f32,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    start: f32,
    stop: f32,
    offset: f32,
    options: &Options,
) -> Vec<Op> {
    let rad_offset = start + _offset_opt(0.1, options, 1.0);
    let mut points: Vec<Point2> = vec![];
    points.push(pt2(
        _offset_opt(offset, options, 1.0) + cx + 0.9 * rx * (rad_offset - increment).cos(),
        _offset_opt(offset, options, 1.0) + cy + 0.9 * ry * (rad_offset - increment).sin(),
    ));
    let mut angle = rad_offset;
    assert!(increment > 0.0);
    while angle <= stop {
        points.push(pt2(
            _offset_opt(offset, options, 1.0) + cx + rx * angle.cos(),
            _offset_opt(offset, options, 1.0) + cy + ry * angle.sin(),
        ));
        angle += increment;
    }
    points.push(pt2(cx + rx * stop.cos(), cy + ry * stop.sin()));
    points.push(pt2(cx + rx * stop.cos(), cy + ry * stop.sin()));
    return _curve(points, None, options);
}

pub fn pattern_fill_arc(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    start: f32,
    stop: f32,
    options: &Options,
) -> OpSet {
    let cx = x;
    let cy = y;
    let mut rx = (width / 2.0).abs();
    let mut ry = (height / 2.0).abs();
    rx += _offset_opt(rx * 0.01, options, 1.0);
    ry += _offset_opt(ry * 0.01, options, 1.0);
    let mut start = start;
    let mut stop = stop;
    while start < 0.0 {
        start += f32::PI() * 2.0;
        stop += f32::PI() * 2.0;
    }
    if (stop - start) > f32::PI() * 2.0 {
        start = 0.0;
        stop = f32::PI();
    }
    let increment = (stop - start) / options.curve_step_count as f32;
    let mut points: Vec<Point2> = vec![];
    let mut angle = start;
    while angle <= stop {
        points.push(pt2(cx + rx * angle.cos(), cy + ry * angle.sin()));
        angle += increment;
    }
    points.push(pt2(cx + rx * stop.cos(), cy + ry * stop.sin()));
    points.push(pt2(cx, cy));
    return pattern_fill_polygon(vec![points], options);
}

#[derive(Clone)]
pub struct EllipseResult {
    pub opset: OpSet,
    pub estimated_points: Vec<Point2>,
}

#[derive(Copy, Clone)]
pub struct EllipseParams {
    rx: f32,
    ry: f32,
    increment: f32,
}

pub fn generate_ellipse_params(width: f32, height: f32, options: &Options) -> EllipseParams {
    let psq =
        (f32::PI() * 2.0 * (((width * 0.5).powi(2) + (height * 0.5).powi(2)) * 0.5).sqrt()).sqrt();
    let step_count = (options.curve_step_count as f32)
        .max(psq * options.curve_step_count as f32 / 200.0.sqrt())
        .ceil();
    let increment = 2.0 * f32::PI() / step_count;
    let mut rx = (width / 2.0).abs();
    let mut ry = (height / 2.0).abs();
    let curve_fit_randomness = 1.0 - options.curve_fitting;
    rx += _offset_opt(rx * curve_fit_randomness, options, 1.0);
    ry += _offset_opt(ry * curve_fit_randomness, options, 1.0);
    return EllipseParams { increment, rx, ry };
}

pub fn ellipse_with_params(
    x: f32,
    y: f32,
    options: &Options,
    ellipse_params: EllipseParams,
) -> EllipseResult {
    let overlap =
        ellipse_params.increment * _offset(0.1, _offset(0.4, 1.0, options, 1.0), options, 1.0);
    let (ap1, cp1) = _compute_ellipse_points(
        ellipse_params.increment,
        x,
        y,
        ellipse_params.rx,
        ellipse_params.ry,
        1.0,
        overlap,
        options,
    );
    let mut o1 = _curve(ap1, None, options);
    if !options.disable_multi_stroke && (options.roughness != 0.0) {
        let (ap2, _) = _compute_ellipse_points(
            ellipse_params.increment,
            x,
            y,
            ellipse_params.rx,
            ellipse_params.ry,
            1.5,
            0.0,
            options,
        );
        o1.extend(_curve(ap2, None, options));
    }
    let ops_type = match options.fill_style {
        FillStyle::Solid => OpSetType::FillPath,
        _ => OpSetType::Path,
    };
    return EllipseResult {
        estimated_points: cp1,
        opset: OpSet { ops_type, ops: o1 },
    };
}

fn _compute_ellipse_points(
    increment: f32,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    offset: f32,
    overlap: f32,
    options: &Options,
) -> (Vec<Point2>, Vec<Point2>) {
    let core_only = options.roughness == 0.0;
    let mut core_points: Vec<Point2> = vec![];
    let mut all_points: Vec<Point2> = vec![];

    if core_only {
        let increment = increment / 4.0;
        all_points.push(pt2(
            cx + rx * (-increment).cos(),
            cy + ry * (-increment).sin(),
        ));
        let mut angle = 0.0;
        while angle <= f32::PI() * 2.0 {
            let p = pt2(cx + rx * angle.cos(), cy + ry * angle.sin());
            core_points.push(p);
            all_points.push(p);
            angle += increment;
        }
        // angle = 0
        all_points.push(pt2(cx + rx, cy));
        all_points.push(pt2(cx + rx * increment.cos(), cy + ry * increment.sin()));
    } else {
        let rad_offset = _offset_opt(0.5, options, 1.0) - 0.5 * f32::PI();
        all_points.push(pt2(
            _offset_opt(offset, options, 1.0) + cx + 0.9 * rx * (rad_offset - increment).cos(),
            _offset_opt(offset, options, 1.0) + cy + 0.9 * ry * (rad_offset - increment).sin(),
        ));
        let end_angle = f32::PI() * 2.0 + rad_offset - 0.01;
        let mut angle = rad_offset;
        while angle < end_angle {
            let p = pt2(
                _offset_opt(offset, options, 1.0) + cx + rx * angle.cos(),
                _offset_opt(offset, options, 1.0) + cy + ry * angle.sin(),
            );
            core_points.push(p);
            all_points.push(p);
            angle += increment;
        }
        all_points.push(pt2(
            _offset_opt(offset, options, 1.0) + cx + rx * (rad_offset + overlap * 0.5).cos(),
            _offset_opt(offset, options, 1.0) + cy + ry * (rad_offset + overlap * 0.5).sin(),
        ));
        all_points.push(pt2(
            _offset_opt(offset, options, 1.0) + cx + 0.98 * rx * (rad_offset + overlap).cos(),
            _offset_opt(offset, options, 1.0) + cy + 0.98 * ry * (rad_offset + overlap).sin(),
        ));
        all_points.push(pt2(
            _offset_opt(offset, options, 1.0) + cx + 0.9 * rx * (rad_offset + overlap * 0.5).cos(),
            _offset_opt(offset, options, 1.0) + cy + 0.9 * ry * (rad_offset + overlap * 0.5).sin(),
        ));
    }
    return (all_points, core_points);
}

fn _curve(points: Vec<Point2>, close_point: Option<Point2>, options: &Options) -> Vec<Op> {
    let len = points.len();
    let mut ops: Vec<Op> = vec![];
    if len > 3 {
        let s = 1.0 - options.curve_tightness;
        ops.push(Op {
            op: OpType::Move,
            data: vec![points[1].x, points[1].y],
        });
        for i in 1..(len - 2) {
            let cached_vert_array = points[i];
            ops.push(Op {
                op: OpType::BcurveTo,
                data: vec![
                    cached_vert_array.x + (s * points[i + 1].x - s * points[i - 1].x) / 6.0,
                    cached_vert_array.y + (s * points[i + 1].y - s * points[i - 1].y) / 6.0,
                    points[i + 1].x + (s * points[i].x - s * points[i + 2].x) / 6.0,
                    points[i + 1].y + (s * points[i].y - s * points[i + 2].y) / 6.0,
                    points[i + 1].x,
                    points[i + 1].y,
                ],
            });
        }
        if let Some(clp) = close_point {
            let ro = options.max_randomness_offset;
            ops.push(Op {
                op: OpType::LineTo,
                data: vec![
                    clp.x + _offset_opt(ro, options, 1.0),
                    clp.y + _offset_opt(ro, options, 1.0),
                ],
            })
        }
    } else if len == 3 {
        ops.push(Op {
            op: OpType::Move,
            data: vec![points[1].x, points[1].y],
        });
        ops.push(Op {
            op: OpType::BcurveTo,
            data: vec![
                points[1].x,
                points[1].y,
                points[2].x,
                points[2].y,
                points[2].x,
                points[2].y,
            ],
        });
    } else if len == 2 {
        ops.extend(
            _double_line(points[0].x, points[0].y, points[1].x, points[1].y, options).into_iter(),
        );
    }
    return ops;
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
