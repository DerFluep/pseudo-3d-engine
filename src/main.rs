mod vec2;

use core::f32;

use macroquad::prelude::*;
use vec2::Vec2;

use crate::vec2::rot_to_vec;

pub fn intersection_distance(origin: &Vec2, vector: &Vec2, line: &Line) -> f32 {
    let origin_x = origin.x;
    let origin_y = origin.y;
    let vector_x = vector.x;
    let vector_y = vector.y;
    let point1_x = line.start.x;
    let point1_y = line.start.y;
    let point2_x = line.end.x;
    let point2_y = line.end.y;

    // Line direction
    let dx = point2_x - point1_x;
    let dy = point2_y - point1_y;

    let denom = vector_x * dy - vector_y * dx;

    /* if denom == 0.0 {
        println!("Lines are parallel");
    } */

    let t = ((point1_x - origin_x) * dy - (point1_y - origin_y) * dx) / denom;
    let s = ((point1_x - origin_x) * vector_y - (point1_y - origin_y) * vector_x) / denom;

    let mut distance = f32::MAX;
    if t >= 0.0 && (0.0..=1.0).contains(&s) {
        distance = t;
    }
    distance
}

pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
}

pub struct Engine {
    pub position: Vec2,
    pub rotation: f32,
    pub fov: f32,
    pub walls: Vec<Line>,
}

impl Engine {
    pub fn new(walls: Vec<Line>) -> Self {
        Engine {
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            fov: 90.0,
            walls,
        }
    }

    pub fn draw_walls(&self) {
        let pixel_off = self.fov / screen_width();
        let first_deg = (self.fov / 2.0) + self.rotation;
        let pixels = screen_width();
        let height_middle = screen_height() / 2.0;
        for pixel in 0..pixels as i32 {
            let direction = rot_to_vec(first_deg - pixel as f32 * pixel_off);

            let mut min_dist = f32::MAX;
            self.walls.iter().for_each(|wall| {
                let distance = intersection_distance(&self.position, &direction, wall);
                if distance < min_dist {
                    min_dist = distance;
                }
            });

            if min_dist < f32::MAX {
                let line_height = screen_height() / min_dist;
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

    'running: loop {
        if is_key_down(KeyCode::Left) {
            engine.rotation += 2.0;
        }
        if is_key_down(KeyCode::Right) {
            engine.rotation -= 2.0;
        }
        if is_key_down(KeyCode::W) {
            engine.position += rot_to_vec(engine.rotation) / 10.0;
        }
        if is_key_down(KeyCode::A) {
            engine.position += rot_to_vec(engine.rotation + 90.0) / 10.0;
        }
        if is_key_down(KeyCode::S) {
            engine.position -= rot_to_vec(engine.rotation) / 10.0;
        }
        if is_key_down(KeyCode::D) {
            engine.position += rot_to_vec(engine.rotation - 90.0) / 10.0;
        }
        clear_background(BLACK);
        engine.draw_walls();
        next_frame().await
    }
}
