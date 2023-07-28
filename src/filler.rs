use std::cmp::Ordering;

use nannou::prelude::*;

use crate::core::{FillStyle, OpSet, OpSetType, Options};
use crate::renderer::_double_line;

use Ordering::*;

pub trait PatternFiller {
    fn fill_polygons(&self, polygon_list: Vec<Vec<Point2>>, o: &Options) -> OpSet;
}

pub struct HachureFiller {}
pub struct ZigzagFiller {}
pub struct DashedFiller {}

struct Line {
    start: Point2,
    end: Point2,
}

#[derive(Clone, Copy)]
struct EdgeEntry {
    ymin: f32,
    ymax: f32,
    x: f32,
    islope: f32,
}

fn cmp_edge(e1: &EdgeEntry, e2: &EdgeEntry) -> Ordering {
    if e1.ymin < e2.ymin {
        return Less;
    }
    if e1.ymin > e2.ymin {
        return Greater;
    }
    if e1.x < e2.x {
        return Less;
    }
    if e1.x > e2.x {
        return Greater;
    }
    if e1.ymax < e2.ymax {
        return Less;
    }
    if e1.ymax > e2.ymax {
        return Greater;
    }
    return Equal;
}

impl Line {
    fn rotate(&self, center: Point2, degrees: f32) -> Self {
        let angle = (f32::PI() / 180.0) * degrees;
        return Line {
            start: (self.start - center).rotate(angle) + center,
            end: (self.end - center).rotate(angle) + center,
        };
    }

    fn len(&self) -> f32 {
        return (self.end - self.start).length_squared();
    }
}

fn rotate_points(points: Vec<Point2>, center: Point2, degrees: f32) -> Vec<Point2> {
    Vec::from_iter(
        points
            .iter()
            .map(|p| (*p - center).rotate((f32::PI() / 180.0) * degrees) + center),
    )
}

pub fn get_filler(options: &Options) -> Box<dyn PatternFiller> {
    match options.fill_style {
        FillStyle::Hachure => Box::new(HachureFiller {}),
        FillStyle::Zigzag => Box::new(ZigzagFiller {}),
        FillStyle::Dashed => Box::new(DashedFiller {}),
        FillStyle::Solid => unreachable!(),
        _ => unimplemented!(),
    }
}

impl PatternFiller for HachureFiller {
    fn fill_polygons(&self, polygon_list: Vec<Vec<Point2>>, o: &Options) -> OpSet {
        let lines = polygon_hachure_lines(polygon_list, o);
        let ops = Vec::from_iter(
            lines
                .iter()
                .map(|l| _double_line(l.start.x, l.start.y, l.end.x, l.end.y, o)),
        )
        .into_iter()
        .flatten()
        .collect();
        return OpSet {
            ops_type: OpSetType::FillSketch,
            ops,
        };
    }
}

fn polygon_hachure_lines(polygon_list: Vec<Vec<Point2>>, o: &Options) -> Vec<Line> {
    let angle = o.hachure_angle + 90.0;
    let mut gap = if o.hachure_gap < 0.0 {
        o.stroke_width * 4.0
    } else {
        o.hachure_gap
    };
    gap = gap.max(0.1);

    let rotation_center = pt2(0.0, 0.0);
    let rotate_polygon_list = Vec::from_iter(
        polygon_list
            .iter()
            .map(|pg| rotate_points(pg.clone(), rotation_center, angle)),
    );
    let lines = straight_hachure_lines(rotate_polygon_list, gap);
    return Vec::from_iter(lines.iter().map(|l| l.rotate(rotation_center, -angle)));
}

fn straight_hachure_lines(polygon_list: Vec<Vec<Point2>>, gap: f32) -> Vec<Line> {
    let mut vertex_array: Vec<Vec<Point2>> = vec![];
    for polygon in polygon_list.iter() {
        let mut vertices = polygon.clone();
        if vertices[0] != vertices[vertices.len() - 1] {
            vertices.push(vertices[0].clone());
        }
        if vertices.len() > 2 {
            vertex_array.push(vertices);
        }
    }

    let mut edges: Vec<EdgeEntry> = vec![];

    for vertices in vertex_array.iter() {
        for i in 0..(vertices.len() - 1) {
            let p1 = vertices[i];
            let p2 = vertices[i + 1];
            if p1.y != p2.y {
                let ymin = p1.y.min(p2.y);
                let ymax = p1.y.max(p2.y);
                let x = if ymin == p1.y { p1.x } else { p2.x };
                let islope = (p2.x - p1.x) / (p2.y - p1.y);
                edges.push(EdgeEntry {
                    ymin,
                    ymax,
                    x,
                    islope,
                })
            }
        }
    }

    edges.sort_by(|e1, e2| cmp_edge(e1, e2));

    let mut active_edges: Vec<EdgeEntry> = vec![];
    let mut y = edges[0].ymin;

    let mut lines: Vec<Line> = vec![];
    let gap = gap.max(0.1);

    while active_edges.len() > 0 || edges.len() > 0 {
        if edges.len() > 0 {
            let mut ix = 0;
            for i in 0..edges.len() {
                if edges[i].ymin > y {
                    break;
                }
                ix = i + 1;
            }
            for r in edges.splice(0..ix, []) {
                active_edges.push(r);
            }
        }
        active_edges = Vec::from_iter(active_edges.into_iter().filter(|e| e.ymax > y));
        active_edges.sort_by(|e1, e2| {
            if e1.x == e2.x {
                return Equal;
            }
            if e1.x < e2.x {
                return Less;
            }
            return Greater;
        });

        if active_edges.len() > 1 {
            for i in (0..active_edges.len()).step_by(2) {
                let nexti = i + 1;
                if nexti >= active_edges.len() {
                    break;
                }
                let ce = active_edges[i];
                let ne = active_edges[nexti];
                lines.push(Line {
                    start: pt2(ce.x, y),
                    end: pt2(ne.x, y),
                });
            }
        }

        y += gap;
        for e in active_edges.iter_mut() {
            e.x += gap * e.islope;
        }
    }
    return lines;
}

impl PatternFiller for ZigzagFiller {
    fn fill_polygons(&self, polygon_list: Vec<Vec<Point2>>, o: &Options) -> OpSet {
        let mut gap = if o.hachure_gap < 0.0 {
            o.stroke_width * 4.0
        } else {
            o.hachure_gap
        };
        gap = gap.max(0.1);
        let lines = polygon_hachure_lines(polygon_list, o);
        let zigzag_angle = (PI / 180.0) * o.hachure_angle;
        let mut zigzag_lines: Vec<Line> = vec![];
        let dgx = gap * 0.5 * zigzag_angle.cos();
        let dgy = gap * 0.5 * zigzag_angle.sin();

        for l in lines.iter() {
            if l.len() > 0.0 {
                zigzag_lines.push(Line {
                    start: pt2(l.start.x - dgx, l.start.y + dgy),
                    end: l.end,
                });
                zigzag_lines.push(Line {
                    start: pt2(l.start.x + dgx, l.start.y - dgy),
                    end: l.end,
                });
            }
        }
        let ops = Vec::from_iter(
            zigzag_lines
                .iter()
                .map(|l| _double_line(l.start.x, l.start.y, l.end.x, l.end.y, o)),
        )
        .into_iter()
        .flatten()
        .collect();
        return OpSet {
            ops_type: OpSetType::FillSketch,
            ops,
        };
    }
}

impl PatternFiller for DashedFiller {
    fn fill_polygons(&self, polygon_list: Vec<Vec<Point2>>, o: &Options) -> OpSet {
        let offset = if o.dash_offset < 0.0 {
            if o.hachure_gap < 0.0 {
                o.stroke_width * 4.0
            } else {
                o.hachure_gap
            }
        } else {
            o.dash_offset
        };
        let gap = if o.dash_gap < 0.0 {
            if o.hachure_gap < 0.0 {
                o.stroke_width * 4.0
            } else {
                o.hachure_gap
            }
        } else {
            o.dash_gap
        };
        let ops = Vec::from_iter(
            polygon_list.iter().map(|l| {
                let length = l.len();
            })
        );
    }
}
