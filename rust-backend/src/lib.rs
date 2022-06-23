use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use vecmath::Vector2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn vec2_normalized_safe(a: Vector2<f32>) -> Vector2<f32> {
    if a[0] == 0.0 && a[1] == 0.0 {
        return [0.0, 0.0];
    }
    return vecmath::vec2_normalized(a);
}

//---------

thread_local!(static FLOCK: RefCell<Vec<Boid>> = RefCell::new(Vec::new()));

//-----

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Boid {
    position: vecmath::Vector2<f32>,
    direction: vecmath::Vector2<f32>,

    speed: f32,
    weight: f32,

    radius_perception: f32,
    force_seperation: f32,
    force_alignment: f32,
    force_cohesion: f32,
}
impl Boid {
    pub fn log(&self) {
        console_log!(
            "pos: ({}, {}), vel: ({}, {}) \nspd: {}, wht: {} \nprc: {} \nsep: {}, aln: {}, coh: {}",
            self.position[0],
            self.position[1],
            self.direction[0],
            self.direction[1],
            self.speed,
            self.weight,
            self.radius_perception,
            self.force_seperation,
            self.force_alignment,
            self.force_cohesion
        );
    }

    pub fn update(&mut self, flock: &Vec<Boid>) {
        let mut acceleration: vecmath::Vector2<f32> = [0.0, 0.0];

        let seperation_steer = self.calculate_seperation_steer(flock);
        acceleration = vecmath::vec2_add(acceleration, seperation_steer);

        let alignment_steer = self.calculate_alignment_steer(flock);
        acceleration = vecmath::vec2_add(acceleration, alignment_steer);

        let cohesion_steer = self.calculate_cohesion_steer(flock);
        acceleration = vecmath::vec2_add(acceleration, cohesion_steer);

        acceleration = vec2_normalized_safe(acceleration);

        self.direction = vecmath::vec2_add(self.direction, acceleration);
        self.direction = vec2_normalized_safe(self.direction);

        let velocity = vecmath::vec2_scale(self.direction, self.speed);
        self.position = vecmath::vec2_add(self.position, velocity);
    }

    fn calculate_seperation_steer(&self, flock: &Vec<Boid>) -> vecmath::Vector2<f32> {
        return [0.0, 0.0];
    }
    fn calculate_alignment_steer(&self, flock: &Vec<Boid>) -> vecmath::Vector2<f32> {
        return [0.0, 0.0];
    }
    fn calculate_cohesion_steer(&self, flock: &Vec<Boid>) -> vecmath::Vector2<f32> {
        let mut desired_position: vecmath::Vector2<f32> = [0.0, 0.0];
        for boid in flock {
            let diff_vec = vecmath::vec2_sub(boid.position, self.position);
            let dist = vecmath::vec2_len(diff_vec);
            if !(dist <= self.radius_perception) {
                continue;
            };

            desired_position = vecmath::vec2_add(desired_position, boid.position);
        }

        let diff_vec = vecmath::vec2_sub(desired_position, self.position);
        let desired_direction = vec2_normalized_safe(diff_vec);

        return desired_direction;
    }
}

//-----

#[wasm_bindgen]
pub fn flock_update() {
    FLOCK.with(|f| {
        let mut flock = f.borrow_mut();
        let flock_org = flock.clone();
        for boid in flock.iter_mut() {
            boid.update(&flock_org);
        }
    });
}

#[wasm_bindgen]
pub fn flock_add_boid(js_boid: &JsValue) {
    FLOCK.with(|f| {
        let mut flock = f.borrow_mut();
        flock.push(js_boid.into_serde().unwrap());
        console_log!("Boid added. Flock now contains {} Boids.", flock.len());
    });
}
#[wasm_bindgen]
pub fn flock_add_boids(js_boids: &JsValue) {
    FLOCK.with(|f| {
        let mut flock = f.borrow_mut();
        flock.append(&mut js_boids.into_serde().unwrap());
        console_log!("Boids added. Flock now contains {} Boids.", flock.len());
    });
}

#[wasm_bindgen]
pub fn flock_remove_boids(amount: u32) {
    FLOCK.with(|f| {
        let mut flock = f.borrow_mut();
        for _i in 0..amount {
            flock.pop();
        }
        console_log!(
            "{} Boids removed. Flock now contains {} Boids.",
            amount,
            flock.len()
        );
    });
}

#[wasm_bindgen]
pub fn flock_get() -> JsValue {
    let mut flock: Vec<Boid> = Vec::new();
    FLOCK.with(|f| {
        flock = f.borrow().clone();
    });
    return JsValue::from_serde(&flock).unwrap();
}
