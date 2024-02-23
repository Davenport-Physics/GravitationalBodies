
use rand::Rng;
use rand::thread_rng;

use crate::position::Position2;
use crate::vector2::Vector2;
use crate::settings::{WIDTH, HEIGHT};

#[derive(Clone, Copy, Debug)]
pub struct Velocity2 {
    pub x: f64,
    pub y: f64   
}

#[derive(Debug)]
pub struct GravitionalBody {

    pub color: [u8; 4],
    pub position: Position2,
    pub velocity: Velocity2,
    pub mass: f64

}

#[derive(Debug)]
pub struct PhysicsSimulation {
    gravitational_bodies: Vec<GravitionalBody>
}

const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
const TIME_STEP: f64 = 1.0e-3;
const DISTANCE_MULTIPLIER: f64 = 1.0e6;

impl PhysicsSimulation {

    pub fn new() -> PhysicsSimulation {

        PhysicsSimulation {
            gravitational_bodies: PhysicsSimulation::init_gravitational_bodies(3)
        }

    }

    fn init_gravitational_bodies(amount: i32) -> Vec<GravitionalBody> {

        let mut bodies = Vec::new();
        let mut rng = thread_rng();

        for _ in 0..amount {

            bodies.push(GravitionalBody {
                color: [rng.gen_range(100..200), rng.gen_range(100..200), rng.gen_range(100..200), 255],
                position: Position2 { x: rng.gen_range(20.0..(WIDTH as f64)-20.0), y: rng.gen_range(20.0..(HEIGHT as f64)-20.0) },
                velocity: Velocity2 { x: 0.0, y: 0.0 },
                mass: rng.gen_range(1.0e22..1.0e26)
            });

        }
        println!("{:?}", bodies);

        bodies

    }

    pub fn tick(&mut self) {

        for i in 0..self.gravitational_bodies.len() {

            for j in 0..self.gravitational_bodies.len() {

                if i == j {
                    continue;
                }

                let pos_vector = self.gravitational_bodies[i].position.vector(&self.gravitational_bodies[j].position);
                let distance_squared = pos_vector.magnitude().powi(2) * DISTANCE_MULTIPLIER;

                if distance_squared <= DISTANCE_MULTIPLIER {
                    println!("Distance too small");
                    continue;
                }

                let angle = pos_vector.angle_from_normal();

                let force = GRAVITATIONAL_CONSTANT * (self.gravitational_bodies[i].mass * self.gravitational_bodies[j].mass) / (distance_squared);

                let force_vector = Vector2 {
                    x: force * angle.cos(),
                    y: force * angle.sin()
                };

                let accleration = Vector2 {
                    x: (force_vector.x / self.gravitational_bodies[i].mass),
                    y: (force_vector.y / self.gravitational_bodies[i].mass)
                };

                self.gravitational_bodies[i].velocity.x += accleration.x * TIME_STEP;
                self.gravitational_bodies[i].velocity.y += accleration.y * TIME_STEP;

                self.gravitational_bodies[i].position.x += self.gravitational_bodies[i].velocity.x * TIME_STEP;
                self.gravitational_bodies[i].position.y += self.gravitational_bodies[i].velocity.y * TIME_STEP;

            }

        }

    }

    pub fn draw(&self, frame: &mut [u8]) {

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let pos = Position2::from(x as f64, y as f64);

            let mut color: Option<[u8; 4]> = None;
            self.gravitational_bodies.iter().for_each(|body| {

                if pos.vector(&body.position).magnitude() as i16 <= 10 {
                    color = Some(body.color);
                }

            });

            if let Some(c) = color {
                pixel.copy_from_slice(&c);
            } else { 
                pixel.copy_from_slice(&[255, 255, 255, 255]);
            }   

        }

    }

}