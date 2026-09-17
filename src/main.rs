mod vec2;

use core::f32;
use std::f32::consts::PI;

use macroquad::prelude::*;
use vec2::Vec2;

pub fn degree_to_radians(degree: f32) -> f32 {
    degree * PI / 180.0
}

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
        let pixel_width = screen_width();
        let height_middle = screen_height() / 2.0;
        for pixel in 0..pixel_width as i32 {
            let mut direction = Vec2::new(
                (pixel as f32 - pixel_width / 2.0) / (pixel_width / 2.0),
                1.0,
            );
            direction.rotate(degree_to_radians(self.rotation));

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
        if is_key_pressed(KeyCode::Left) {
            engine.rotation += 5.0;
        }
        if is_key_pressed(KeyCode::Right) {
            engine.rotation -= 5.0;
        }
        if is_key_pressed(KeyCode::Up) {
            engine.position.y += 1.0;
        }
        if is_key_pressed(KeyCode::Down) {
            engine.position.y -= 1.0;
        }
        clear_background(BLACK);
        engine.draw_walls();
        next_frame().await
    }
}
