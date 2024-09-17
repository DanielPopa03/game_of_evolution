use rand::Rng;
use sdl2::{pixels::Color, render::Canvas, rect::Point, video::Window};

use crate::traits::creature::Creature;

#[derive(Clone)]
pub struct Carnivore {
    color: Color,
    health: i32,
    fullness: i32,
    fullness_max: i32,
    current_time: i32,
    reproduction_time: i32,
    attack: i32,
    id: i32,
}

impl Carnivore {
    pub fn new(color: Color, health: i32, fullness: i32, fullness_max: i32, reproduction_time: i32, attack: i32, id: i32) -> Carnivore {
        Carnivore {
            color,
            health,
            fullness,
            fullness_max,
            current_time: 0,
            reproduction_time,
            attack,
            id,
        }
    }
}

impl Creature for Carnivore {

    fn clone_box(&self) -> Box<dyn Creature> {
        Box::new(self.clone()) // Clone the concrete type
    }

    fn reproduce(&mut self) -> Option<Box<dyn Creature>>{
        if self.current_time < self.reproduction_time {
            self.current_time += 1;
            return None;
        }
        self.current_time = 0;
        let mut carnivor = self.clone();
        //carnivor.id *= 2;
        carnivor.current_time = 0;
        let mut rng = rand::thread_rng();
        // Randomly choose a direction: 0=right, 1=left, 2=up, 3=down
        let chance = rng.gen_range(0..6);
        match chance {
            0 => {
                let good_chance = rng.gen_range(0..3);
                match good_chance {
                    0 => {
                        if self.color.g as i32 + 10 > 255 {
                            carnivor.color =
                            Color::RGB(self.color.r, self.color.g - 10, self.color.b)
                        } else {
                            carnivor.color =
                            Color::RGB(self.color.r, self.color.g + 10, self.color.b)
                        }   
                        
                    }
                    1 => carnivor.attack += 10 ,
                    2 => carnivor.fullness_max += 10,
                    _ => (),
                }
            } // Good
            1 => {
                let eq_chance = rng.gen_range(0..3);
                match eq_chance {
                    0 => {
                        carnivor.attack -= 10;
                        carnivor.fullness_max += 10;
                    }
                    1 => {
                        carnivor.attack += 10;
                        carnivor.fullness_max -= 10;
                    }
                    2 => {
                        carnivor.reproduction_time -= 1;
                        carnivor.fullness_max -= 10;
                        carnivor.attack -= 10;
                    }
                    _ => (),
                }
            } // Equaly good and bad
            2 => {
                let bad_chance = rng.gen_range(0..3);
                match bad_chance {
                    0 => { 
                        if self.color.r as i32 + 10 > 255 {
                            carnivor.color = Color::RGB(self.color.r - 10, self.color.g, self.color.b)
                        } else {
                        carnivor.color = Color::RGB(self.color.r + 10, self.color.g, self.color.b)
                        }   
                    },
                    1 => carnivor.attack -= 10,
                    2 => carnivor.fullness_max -= 10,
                    _ => (),
                }
            } // Bad
            _ => (), // No movement if out of bounds
        }
        return Some(Box::new(carnivor));
    }

    fn get_id(&self) -> i32 {
        return self.id;
    }

    fn health(&mut self) -> bool {
        self.fullness -= 10;
        if self.fullness <= 0 {
            self.health -= 50;
        } else {
            if self.health < 100 {
                self.health += 10;
            }
        }
        return self.health > 0;
    }

    fn show(&self){
        println!(
            "( health:{},fullness:{},fullness_max:{},reproduction_time:{},attack:{})",
            self.health,
            self.fullness,
            self.fullness_max,
            self.reproduction_time,
            self.attack
        );
    }

    fn attack(&mut self) -> i32 {
        print!("Attack");
        if self.fullness + self.attack > self.fullness_max {
            self.fullness = self.fullness_max;
        }
        return self.attack;
    }

    fn type_of_creature(&self) -> String {
        return "Carnivor".to_string();
    }

    fn type_of_prey(&self) -> String {
        return "Carnivor".to_string();
    }

    fn paint(&self, canvas: &mut Canvas<Window>, coordinates: (i32, i32)) {
        canvas.set_draw_color(self.color);
        canvas.draw_point(Point::new(coordinates.0, coordinates.1)).unwrap();
    }

    fn add_health(&mut self, amount: i32) -> i32 {
        self.health += amount;
        return self.health;
    }

    fn reduce_health(&mut self, amount: i32) -> i32 {
        print!("Reduce health");
        self.health -= amount;
        return self.health;
    }

    fn can_move_and_attack(&self) -> bool {
        return true;
    }
}