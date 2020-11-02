#[allow(dead_code)]
#[allow(unused_variables)]
use crate::flappy::bird::Bird;
use crate::renderer::renderer::Renderer;
use rand::Rng;

use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Clone)]
pub struct Pipe {
    pub x: f32,
    pub top: f32,
    pub bottom: f32,
    pub w: i32,
    pub speed: f32,

    width: u32,
    height: u32,
}

impl Pipe {
    pub fn new(width: u32, height: u32) -> Pipe {
        let f_height = height as f32;
        let mut rng = rand::thread_rng();
        // How big is the empty space
        let spacing = 125.0;
        // Where is the center of the empty space
        let center_y = rng.gen_range(spacing, f_height - spacing);

        Pipe {
            // Top and bottom of pipe
            top: center_y - spacing / 2.0,
            bottom: f_height - (center_y + spacing / 2.0),
            // Starts at the edge
            x: width as f32,
            // width of the pipe
            w: 80,
            // How fast
            speed: 6.0,

            height,
            width,
        }
    }

    // Did this pipe hit a bird?
    pub fn hits(&self, bird: &Bird) -> bool {
        if (bird.y - bird.r) < self.top as f32
            || (bird.y + bird.r) > (self.height as f32 - self.bottom as f32)
        {
            if bird.x > self.x && bird.x < self.x + self.w as f32 {
                return true;
            }
        }
        false
    }

    pub fn show(&self, canvas: &mut Canvas<Window>) {
        Renderer::rect(canvas, self.x as i32, 0, self.w as u32, self.top as u32).unwrap();
        Renderer::rect(
            canvas,
            self.x as i32,
            self.height as i32 - self.bottom as i32,
            self.w as u32,
            self.bottom as u32,
        )
        .unwrap();
    }

    // Update the pipe
    pub fn update(&mut self) {
        self.x -= self.speed;
    }

    // Has it moved offscreen?
    pub fn offscreen(&self) -> bool {
        if (self.x as i32) < -self.w {
            return true;
        }
        return false;
    }
}
