#[allow(dead_code)]
#[allow(unused_variables)]
extern crate sdl2;

mod flappy;
mod matrix;
mod nn;
mod renderer;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use flappy::game::Game;

fn main() -> Result<(), String> {
    let width: u32 = 800;
    let height: u32 = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL2", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    //let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    let texture_creator = canvas.texture_creator();
    let mut tex = texture_creator
        .create_texture_target(None, width, height)
        .map_err(|_| String::from("Unable to create texture."))?;

    let mut game = Game::new(width, height);

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        //let ticks = timer.ticks() as i32;

        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas
            .with_texture_canvas(&mut tex, |the_canvas| {
                game.draw(the_canvas).unwrap();
            })
            .map_err(|_| String::from("Failed to draw on texture"))?;
        canvas.copy(&tex, None, Rect::new(0, 0, width, height))?;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.present();

        std::thread::sleep(Duration::from_millis(33));
    }

    Ok(())
}
