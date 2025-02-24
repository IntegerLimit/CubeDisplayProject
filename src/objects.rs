use std::cmp::Ordering;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Point {
    pub vec: Vec4,
    pub color: Color,
    pub name: String,
    pub name_offset_y: f32,
    pub screen_point: Option<Vec3>
}

pub struct Line {
    pub color: Color,
    pub pt_a: Point,
    pub pt_b: Point,
}

pub fn ln_cmp(a: &Line, b: &Line) -> Ordering {
    ln_draw_z(b).partial_cmp(&ln_draw_z(a)).unwrap_or(Ordering::Equal)
}

pub fn create_btm_point(vec: Vec4, name: String, color: Color) -> Point {
    Point { vec, name, color, name_offset_y: 50.0, screen_point: None }
}

pub fn create_tp_point(vec: Vec4, name: String, color: Color) -> Point {
    Point { vec, name, color, name_offset_y: -25.0, screen_point: None }
}

pub fn ln_white(pt_a: Point, pt_b: Point) -> Line {
    Line { color: WHITE, pt_a, pt_b }
}

pub fn ln(color: Color, pt_a: Point, pt_b: Point) -> Line {
    Line { color, pt_a, pt_b }
}

pub fn ln_draw_z(ln: &Line) -> f32 {
    (pt_draw_z(&ln.pt_a) + pt_draw_z(&ln.pt_b)) / 2.0
}

pub fn pt_draw_z(pt: &Point) -> f32 {
    pt.screen_point.unwrap().z
}