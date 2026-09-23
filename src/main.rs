use core::f32;

use macroquad::prelude::*;
use vec2::Vec2;

fn rot_to_vec(radians: f32) -> Vec2<f32> {
    Vec2::new(radians.cos(), radians.sin())
}

pub fn intersection_distance(origin: &Vec2<f32>, vector: &mut Vec2<f32>, line: &Line) -> f32 {
    vector.normalize();
    // store deltaX and deltaY of the line in a Vec2
    let line_delta = Vec2::new(line.end.x - line.start.x, line.end.y - line.start.y);

    let cross_product = vector.cross(line_delta);

    // Lines are parallel so no rendering of it is possible
    if cross_product == 0.0 {
        return f32::MAX;
    }

    let t = ((line.start.x - origin.x) * line_delta.y - (line.start.y - origin.y) * line_delta.x)
        / cross_product;
    let s = ((line.start.x - origin.x) * vector.y - (line.start.y - origin.y) * vector.x)
        / cross_product;

    if t >= 0.0 && (0.0..=1.0).contains(&s) {
        return t;
    }
    f32::MAX
}

pub struct Line {
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
}

pub struct Engine {
    pub position: Vec2<f32>,
    pub rotation: f32,
    pub fov: f32,
    pub walls: Vec<Line>,
}

impl Engine {
    pub fn new(walls: Vec<Line>) -> Self {
        Engine {
            position: Vec2::new(0.0, 0.0),
            rotation: 90.0_f32.to_radians(),
            fov: 90.0_f32.to_radians(),
            walls,
        }
    }

    pub fn draw_walls(&self) {
        let pixel_off = self.fov / screen_width();
        let first_deg = self.fov / 2.0;
        let pixels = screen_width();
        let height_middle = screen_height() / 2.0;
        for pixel in 0..pixels as i32 {
            let angle = first_deg - pixel as f32 * pixel_off;
            let mut direction = rot_to_vec(angle + self.rotation);

            let mut min_dist = f32::MAX;
            self.walls.iter().for_each(|wall| {
                let distance = intersection_distance(&self.position, &mut direction, wall);
                if distance < min_dist {
                    min_dist = distance;
                }
            });

            // FIXME: still some distortion in the wall height
            if min_dist < f32::MAX {
                let dist_corrected = min_dist * angle.cos();
                let line_height = screen_height() / dist_corrected;
                draw_line(
                    pixel as f32,
                    height_middle - line_height / 2.0,
                    pixel as f32,
                    height_middle + line_height / 2.0,
                    1.0,
                    WHITE,
                );
            }
        }
    }
}

#[macroquad::main("Gravity Particle Sim")]
async fn main() {
    let mut walls = vec![Line {
        start: Vec2::new(-2.0, 6.0),
        end: Vec2::new(2.0, 6.0),
    }];
    walls.push(Line {
        start: Vec2::new(2.0, 6.0),
        end: Vec2::new(2.0, 0.0),
    });

    let mut engine = Engine::new(walls);
    let ninty_deg_rad = 90.0_f32.to_radians();
    let rotation_change = 2.0_f32.to_radians();

    'running: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'running;
        }
        if is_key_down(KeyCode::Left) {
            engine.rotation += rotation_change;
        }
        if is_key_down(KeyCode::Right) {
            engine.rotation -= rotation_change;
        }
        if is_key_down(KeyCode::W) {
            engine.position += rot_to_vec(engine.rotation) / 10.0;
        }
        if is_key_down(KeyCode::A) {
            engine.position += rot_to_vec(engine.rotation + ninty_deg_rad) / 10.0;
        }
        if is_key_down(KeyCode::S) {
            engine.position -= rot_to_vec(engine.rotation) / 10.0;
        }
        if is_key_down(KeyCode::D) {
            engine.position += rot_to_vec(engine.rotation - ninty_deg_rad) / 10.0;
        }
        clear_background(BLACK);
        engine.draw_walls();
        next_frame().await
    }
}
