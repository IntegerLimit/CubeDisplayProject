use std::cmp::Ordering;
use macroquad::prelude::*;

#[derive (Clone)]
pub struct Point {
    pub vec: Vec4,
    pub color: Color,
    pub name: String,
    pub screen_point: Option<Vec3>
}

pub struct Line {
    pub color: Color,
    pub pt_a: Point,
    pub pt_b: Point,
}

pub enum Drawable {
    Point(Point),
    Line(Line),
}

pub enum DrawOrder {
    Point,
    Line
}

pub trait DrawComp {
    fn draw_comp(&self, other: &Drawable) -> Ordering;
}

impl DrawComp for Point {
    fn draw_comp(&self, other: &Drawable) -> Ordering {
        match other {
            Drawable::Point(pt) => {
                // Highest Z Value First
                pt_draw_z(pt).partial_cmp(&pt_draw_z(self)).unwrap_or(Ordering::Equal)
            }
            Drawable::Line(ln) => {
                match comp_ln_pt(self, ln) {
                    DrawOrder::Point => { Ordering::Greater }
                    DrawOrder::Line => { Ordering::Less }
                }
            }
        }
    }
}

impl DrawComp for Line {
    fn draw_comp(&self, other: &Drawable) -> Ordering {
        match other {
            Drawable::Point(pt) => {
                match comp_ln_pt(pt, self) {
                    DrawOrder::Point => { Ordering::Less }
                    DrawOrder::Line => { Ordering::Greater }
                }
            }
            Drawable::Line(ln) => {
                // Highest Z Value First
                ln_draw_z(ln).partial_cmp(&ln_draw_z(self)).unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl DrawComp for Drawable {
    fn draw_comp(&self, other: &Drawable) -> Ordering {
        match self {
            Drawable::Point(pt) => { pt.draw_comp(other) }
            Drawable::Line(ln) => { ln.draw_comp(other) }
        }
    }
}

pub fn create_point(vec: Vec4, name: String, color: Color) -> Point {
    Point { vec, name, color, screen_point: None }
}

pub fn ln_white(pt_a: Point, pt_b: Point) -> Line {
    Line { color: WHITE, pt_a, pt_b }
}

pub fn ln(color: Color, pt_a: Point, pt_b: Point) -> Line {
    Line { color, pt_a, pt_b }
}

// Returns which (point or line) should be on top (behind in ordering)
pub fn comp_ln_pt(pt: &Point, ln: &Line) -> DrawOrder {
    return DrawOrder::Point;

    // TODO Get this to work, need to deal with the problem that a < b & b < c doesn't mean a < c
    if pt_draw_z(&ln.pt_a) == pt_draw_z(pt) || pt_draw_z(&ln.pt_b) == pt_draw_z(pt) {
        return DrawOrder::Point;
    }

    if ln_draw_z(ln) > pt_draw_z(pt) {
        return DrawOrder::Line;
    }

    DrawOrder::Point
}

pub fn ln_draw_z(ln: &Line) -> f32 {
    (pt_draw_z(&ln.pt_a) + pt_draw_z(&ln.pt_b)) / 2.0
}

pub fn pt_draw_z(pt: &Point) -> f32 {
    pt.screen_point.unwrap().z
}