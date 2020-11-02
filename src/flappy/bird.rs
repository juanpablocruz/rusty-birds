use crate::nn::nn::NeuralNetwork;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::flappy::pipe::Pipe;
use crate::renderer::renderer::Renderer;

fn random_gaussian() -> f32 {
    StdRng::from_entropy().sample(Standard)
}
fn range_map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}

#[derive(Debug, Clone)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub gravity: f32,
    pub lift: i32,
    pub velocity: f32,
    pub brain: NeuralNetwork,
    pub score: u64,
    pub fitness: f32,

    width: u32,
    height: u32,
}

fn mutate(x: f32) -> f32 {
    let mut res_x = x;
    if rand::thread_rng().gen_range(0.0, 1.0) < 0.1 {
        let offset = random_gaussian() * 0.5;
        let new_x = x + offset;
        res_x = new_x;
    }
    res_x
}

impl Bird {
    pub fn new(width: u32, height: u32, brain: NeuralNetwork) -> Bird {
        Bird {
            // position and size of bird
            x: 64.0,
            y: height as f32 / 2.0,
            r: 12.0,
            // Gravity, lift and velocity
            gravity: 0.8,
            lift: -12,
            velocity: 0.0,

            brain: brain,
            // Score is how many frames it's been alive
            score: 0,
            // Fitness is normalized version of score
            fitness: 0.0,
            height,
            width,
        }
    }

    pub fn copy(&self) -> Bird {
        let mut new_nn = self.brain.copy();
        new_nn.mutate(mutate);
        Bird::new(self.width, self.height, new_nn)
    }

    pub fn show(&self, canvas: &mut Canvas<Window>) {
        Renderer::draw_circle(
            canvas,
            Point::new(self.x as i32, self.y as i32),
            self.r as i32,
        )
        .unwrap();
    }
    pub fn up(&mut self) {
        self.velocity += self.lift as f32;
    }
    pub fn bottom_top(&self) -> bool {
        self.y > self.height as f32 || self.y < 0.0
    }

    pub fn update(&mut self) {
        self.velocity += self.gravity;
        self.y += self.velocity;
        self.score += 1;
    }

    /**
     * This is the key function that decides
     * if it should jump or not jump
     */
    pub fn think(&mut self, pipes: &Vec<Pipe>) {
        // First find the closest pipe
        let mut closest = None;
        let mut record = f32::INFINITY;
        for i in 0..pipes.len() {
            let diff = pipes[i].x - self.x;
            if diff > 0.0 && diff < record {
                record = diff;
                closest = Some(i);
            }
        }

        if closest.is_some() {
            let pipe_i = closest.unwrap();
            // Now create the inputs to the neural network
            let mut inputs: [f32; 5] = [0.0; 5];
            let pipe = &pipes[pipe_i];
            // x position of closest pipe
            inputs[0] = range_map(pipe.x, self.x, self.width as f32, 0.0, 1.0);
            inputs[1] = range_map(pipe.top, 0.0, self.height as f32, 0.0, 1.0);
            inputs[2] = range_map(pipe.bottom, 0.0, self.height as f32, 0.0, 1.0);
            inputs[3] = range_map(self.y, 0.0, self.height as f32, 0.0, 1.0);
            inputs[4] = range_map(self.velocity, -5.0, 5.0, 0.0, 1.0);

            // Get the outputs from the network
            let action = self.brain.predict(&inputs).unwrap();
            // Decide to jump or not
            if action[1] > action[0] {
                self.up();
            }
        };
    }
}
