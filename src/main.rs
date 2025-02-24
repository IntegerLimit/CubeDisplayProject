mod objects;
mod icons;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Add;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use crate::icons::*;
use crate::objects::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Cube M N Demo"),
        window_width: 1200,
        window_height: 600,
        icon: Some(Icon {
            small: SMALL,
            medium: MEDIUM,
            big: BIG,
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let near: f32 = 95.0;
    let far: f32 = 105.0;
    let fov: f32 = PI / 2.0;

    let rot_spd: f32 = 1.0;
    let mut rot_x: f32 = PI / 6.0;
    let mut rot_y: f32 = PI / 6.0;

    let trs_spd: f32 = 0.2;
    let mut m: f32 = 0.5;
    let mut n: f32 = 0.5;

    let mut show_info: bool = true;

    loop {
        clear_background(BLACK);

        // Key handling
        if is_key_down(KeyCode::Left) {
            rot_y -= rot_spd * get_frame_time();
        }
        if is_key_down(KeyCode::Right) {
            rot_y += rot_spd * get_frame_time();
        }
        if is_key_down(KeyCode::Down) {
            rot_x -= rot_spd * get_frame_time();
        }
        if is_key_down(KeyCode::Up) {
            rot_x += rot_spd * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            m -= trs_spd * get_frame_time();
        }
        if is_key_down(KeyCode::D) {
            m += trs_spd * get_frame_time();
        }
        if is_key_down(KeyCode::J) {
            n -= trs_spd * get_frame_time();
        }
        if is_key_down(KeyCode::L) {
            n += trs_spd * get_frame_time();
        }
        if is_key_pressed(KeyCode::H) {
            show_info = !show_info;
        }

        // Clamping
        m = m.clamp(0.0, 1.0);
        n = n.clamp(0.0, 1.0);

        let mut pts = Vec::new();
        let mut lns = Vec::new();

        // Setup Cube Vertices
        let mut a = create_btm_point(from_v3(vec3(-1.0, 1.0, -1.0)), String::from("A"), RED);
        let mut b = create_btm_point(from_v3(vec3(1.0, 1.0, -1.0)), String::from("B"), BLUE);
        let mut c = create_btm_point(from_v3(vec3(1.0, 1.0, 1.0)), String::from("C"), GREEN);
        let mut d = create_btm_point(from_v3(vec3(-1.0, 1.0, 1.0)), String::from("D"), YELLOW);
        let mut a1 = create_tp_point(from_v3(vec3(-1.0, -1.0, -1.0)), String::from("A'"), RED);
        let mut b1 = create_tp_point(from_v3(vec3(1.0, -1.0, -1.0)), String::from("B'"), BLUE);
        let mut c1 = create_tp_point(from_v3(vec3(1.0, -1.0, 1.0)), String::from("C'"), GREEN);
        let mut d1 = create_tp_point(from_v3(vec3(-1.0, -1.0, 1.0)), String::from("D'"), YELLOW);

        // Setup M and N Points
        let mut m_vec = create_btm_point(a1.vec + (b.vec - a1.vec) * m, String::from("M"), VIOLET);
        let mut n_vec = create_tp_point(d1.vec + (b1.vec - d1.vec) * n, String::from("N"), MAGENTA);

        // Add Points (Transfer Ownership)
        pts.push(&mut a);
        pts.push(&mut b);
        pts.push(&mut c);
        pts.push(&mut d);
        pts.push(&mut a1);
        pts.push(&mut b1);
        pts.push(&mut c1);
        pts.push(&mut d1);
        pts.push(&mut m_vec);
        pts.push(&mut n_vec);

        // Matrices
        let quat = Quat::from_rotation_x(rot_x) * Quat::from_rotation_y(rot_y);
        let to_world_mat = Mat4::from_scale_rotation_translation(Vec3::splat(1.0), quat, vec3(0.0, 0.0, 100.0));

        let aspect = screen_width() / screen_height();
        let fov_tan = (fov / 2.0).tan();
        let project_mat = mat4(Vec4::splat(0.0).with_x(1.0 / (aspect * fov_tan)),
                               Vec4::splat(0.0).with_y(1.0 / fov_tan),
                               vec4(0.0, 0.0, -(near + far) / (near - far), 1.0),
                               Vec4::splat(0.0).with_z((2.0 * near * far) / (near - far)));

        let result_mat = project_mat * to_world_mat;

        let mut to_draw_pts = Vec::new();

        // Translate Points
        for pt in &mut pts {
            pt.screen_point = Some(to_screen(&pt.vec, &result_mat));
            to_draw_pts.push(pt.clone())
        }

        // Setup Cube Lines

        // Main Edges
        lns.push(ln_white(a.clone(), b.clone()));
        lns.push(ln_white(b.clone(), c.clone()));
        lns.push(ln_white(c.clone(), d.clone()));
        lns.push(ln_white(d.clone(), a.clone()));

        lns.push(ln(LIME, a1.clone(), b1.clone()));
        lns.push(ln(LIME, b1.clone(), c1.clone()));
        lns.push(ln(LIME, c1.clone(), d1.clone()));
        lns.push(ln(LIME, d1.clone(), a1.clone()));

        lns.push(ln(GOLD, a.clone(), a1.clone()));
        lns.push(ln(GOLD, b.clone(), b1.clone()));
        lns.push(ln(GOLD, c.clone(), c1.clone()));
        lns.push(ln(GOLD, d.clone(), d1.clone()));

        // M and N Translatable Lines
        lns.push(ln(PINK, a1.clone(), m_vec.clone()));
        lns.push(ln(PINK, m_vec.clone(), b.clone()));
        lns.push(ln(PURPLE, b1.clone(), n_vec.clone()));
        lns.push(ln(PURPLE, n_vec.clone(), d1.clone()));

        // MN Line
        lns.push(ln(BEIGE, m_vec.clone(), n_vec.clone()));

        let mut to_draw_lns = Vec::new();

        // Add Lines to Draw
        for ln in lns {
            to_draw_lns.push(ln);
        }

        // Sort To Draw
        to_draw_lns.sort_by(|d1, d2| -> Ordering { ln_cmp(d1, d2) });

        // Draw Points after the Third Line wants to
        // Note: All vertices (even M and N) have three lines connecting to them.
        // This is the only reason this works.
        let mut pt_draw_counter = HashMap::new();

        // Draw
        for ln in to_draw_lns {
            draw_line(ln.pt_a.screen_point.unwrap().x, ln.pt_a.screen_point.unwrap().y,
                      ln.pt_b.screen_point.unwrap().x, ln.pt_b.screen_point.unwrap().y,
                      15.0, ln.color);

            update_pt_counter(ln.pt_a, &mut pt_draw_counter, &show_info);
            update_pt_counter(ln.pt_b, &mut pt_draw_counter, &show_info);
        }

        if show_info {
            draw_text("Controls:", 25.0, 50.0, 50.0, WHITE);
            draw_text("Rotate: Arrows", 25.0, 100.0, 40.0, WHITE);
            draw_text("M: A + D", 25.0, 140.0, 40.0, WHITE);
            draw_text("N: J + L", 25.0, 180.0, 40.0, WHITE);
            draw_text("Toggle Info: H", 25.0, 220.0, 40.0, WHITE);

            draw_text("Data:", screen_width() - 175.0, 50.0, 50.0, WHITE);
            draw_text(&*(String::from("MN: ")
                .add(&format!("{:.3}", m_vec.vec.distance(n_vec.vec) / 2.0))),
                      screen_width() - 175.0, 100.0, 40.0, WHITE);
            draw_text(&*(String::from("m: ").add(&format!("{:.2}", m))),
                      screen_width() - 175.0, 140.0, 40.0, WHITE);
            draw_text(&*(String::from("n: ").add(&format!("{:.2}", n))),
                      screen_width() - 175.0, 180.0, 40.0, WHITE);
        }

        next_frame().await
    }
}

fn update_pt_counter(pt: Point, map: &mut HashMap<String, i32>, show_info: &bool) {
    let curr = map.get(&pt.name).unwrap_or(&0) + 1;
    if curr >= 3 {
        draw_circle(pt.screen_point.unwrap().x, pt.screen_point.unwrap().y, 15.0, pt.color);

        if *show_info {
            draw_text(&pt.name, pt.screen_point.unwrap().x - 15.0,
                      pt.screen_point.unwrap().y + pt.name_offset_y, 50.0, pt.color);
        }
    }
    map.insert(pt.name, curr);
}

fn to_screen(vec4: &Vec4, mat4: &Mat4) -> Vec3 {
    let result = *mat4 * *vec4;
    let nx = result.x / result.w * 50.0;
    let ny = result.y / result.w * 50.0;

    // Dividing by 2 as original nx/ny is between -1 and 1, we need it between 0 and screen dimensions
    let screen_x = nx * screen_width() / 2.0 + screen_width() / 2.0;
    let screen_y = ny * screen_height() / 2.0 + screen_height() / 2.0;

    vec3(screen_x, screen_y, result.z / result.w)
}

fn from_v3(vec3: Vec3) -> Vec4 {
    Vec4::from((vec3, 1.0))
}
