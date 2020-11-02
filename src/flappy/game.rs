#[allow(dead_code)]
#[allow(unused_variables)]
use crate::flappy::bird::Bird;
use crate::flappy::pipe::Pipe;
use crate::nn::nn::NeuralNetwork;
use rand::Rng;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Game {
    counter: u32,
    best_bird: Option<usize>,
    high_score: u32,
    run_best: bool,
    active_birds: Vec<usize>,
    all_birds: Vec<Bird>,
    pipes: Vec<Pipe>,
    width: u32,
    height: u32,
    cycle_speed: i32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let total_population = 500;
        let mut active_birds: Vec<usize> = Vec::new();
        let mut all_birds = Vec::new();

        for i in 0..total_population {
            let bird = Bird::new(width, height, NeuralNetwork::new(5, 8, 2));
            all_birds.push(bird);
            active_birds.push(i);
        }

        Game {
            pipes: Vec::new(),
            counter: 0,
            best_bird: None,
            high_score: 0,
            run_best: false,
            width,
            height,
            all_birds,
            active_birds,
            cycle_speed: 1,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        //println!("RENDER");
        for _ in 0..self.cycle_speed {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            // Show all the pipes
            let acc: Vec<Pipe> = Vec::new();
            let new_pipes = self.pipes.iter().fold(acc, |acc, pipe| {
                let mut new_pipe = pipe.clone();
                new_pipe.update();
                if new_pipe.offscreen() {
                    return acc;
                }
                let mut new_acc = acc.clone();
                new_acc.push(new_pipe);
                new_acc
            });
            self.pipes = new_pipes.clone();

            if self.run_best {
                if self.best_bird.is_some() {
                    let best_index = self.best_bird.unwrap();
                    self.all_birds[best_index].think(&new_pipes);
                    self.all_birds[best_index].update();
                    for j in 0..new_pipes.len() {
                        // Start over, bird hit pipe
                        if new_pipes[j].hits(&self.all_birds[best_index]) {
                            self.reset_game();
                        }
                    }
                    self.best_bird = Some(best_index);
                    if self.all_birds[best_index].bottom_top() {
                        self.reset_game();
                    }
                }
            } else {
                let mut acc_birds: Vec<usize> = Vec::new();
                let mut add_bird;
                for i in 0..self.active_birds.len() {
                    add_bird = true;
                    let index = self.active_birds[i];
                    let bird = &mut self.all_birds[index];
                    bird.think(&new_pipes);
                    bird.update();

                    for j in 0..new_pipes.len() {
                        if new_pipes[j].hits(bird) {
                            add_bird = false;
                            break;
                        }
                    }
                    if !bird.bottom_top() && add_bird {
                        acc_birds.push(index);
                    }
                }
                self.active_birds = acc_birds;
            }
            if self.counter % 75 == 0 {
                self.pipes.push(Pipe::new(self.width, self.height));
            }
            self.counter = self.counter + 1;
        }
        let mut tmp_high_score: u32 = 0;
        if !self.run_best {
            // which is the best bird?
            let mut tmp_best_bird = None;
            for i in 0..self.active_birds.len() {
                let index = self.active_birds[i];
                let s = self.all_birds[index].score as u32;
                if s > tmp_high_score {
                    tmp_high_score = s;
                    tmp_best_bird = Some(index);
                }
            }

            // Is it the all time high scorer?
            if tmp_high_score > self.high_score {
                self.high_score = tmp_high_score;
                self.best_bird = tmp_best_bird;
            }
        } else {
            // Just one bird, the best one so far
            if self.best_bird.is_some() {
                let best_index = self.best_bird.ok_or("No best bird")?;
                let bb = &mut self.all_birds[best_index];
                let tmp_high_score: u32 = bb.score as u32;
                if tmp_high_score > self.high_score {
                    self.high_score = tmp_high_score;
                }
            }
        }
        println!("High score: {}", tmp_high_score);
        println!("All time high score: {}", self.high_score);
        // Draw everything
        self.pipes.iter().for_each(|pipe| {
            pipe.show(canvas);
        });

        if self.run_best && self.best_bird.is_some() {
            let best_index = self.best_bird.ok_or("No best bird")?;
            self.all_birds[best_index].show(canvas);
        } else {
            for i in 0..self.active_birds.len() {
                self.all_birds[self.active_birds[i]].show(canvas)
            }
            // If we're out of birds go to the next generation
            if self.active_birds.len() == 0 {
                println!("Next generation");
                self.next_generation();
            }
        }
        Ok(())
    }

    // Start the game over
    pub fn reset_game(&mut self) {
        self.counter = 0;
        if self.best_bird.is_some() {
            let best_index = self.best_bird.unwrap();
            self.all_birds[best_index].score = 0;
            self.best_bird = Some(best_index);
        }
        self.pipes = Vec::new();
    }

    // Create the next generation
    pub fn next_generation(&mut self) {
        self.normalize_fitness();
        self.active_birds = Game::generate(&self.all_birds);
        self.reset_game();
        for i in 0..self.active_birds.len() {
            let index = self.active_birds[i];
            self.all_birds[index] = self.all_birds[index].copy();
        }
    }

    pub fn generate(all_birds: &Vec<Bird>) -> Vec<usize> {
        let mut new_birds: Vec<usize> = Vec::new();
        for _ in 0..all_birds.len() {
            let index = Game::pool_selection(all_birds);
            new_birds.push(index);
        }
        new_birds
    }
    pub fn pool_selection(all_birds: &[Bird]) -> usize {
        let mut rng = rand::thread_rng();
        let mut index: usize = 0;
        let mut r = rng.gen_range(0.0, 1.0);

        // Keep subtracting probabilities until you get less than zero
        // Higer probabilities will be more likely to be fixed since
        // they will subtract a larger number towards zero
        while r > 0.0 {
            r -= all_birds[index].fitness;
            index += 1;
        }
        // Go back one
        index -= 1;
        return index;
    }

    // Normalize the fitness of all birds
    pub fn normalize_fitness(&mut self) {
        for i in 0..self.all_birds.len() {
            let mut b = &mut self.all_birds[i];
            let val = b.score;
            b.score = val;
        }
        // Add up all the scores
        let sum = self
            .all_birds
            .iter()
            .fold(0.0, |acc, e| acc + e.score as f32);
        if sum > 0.0 {
            for i in 0..self.all_birds.len() {
                let mut b = &mut self.all_birds[i];
                let new_fitness = (b.score as f32) / sum;
                b.fitness = new_fitness;
            }
        }
    }
}
