use rand::Rng;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::traits::creature::Creature;

use super::{carnivore::Carnivore, matrix_hash_table::MatrixHashTable};

pub struct Game {
    creatures: MatrixHashTable<Box<dyn Creature>>,
    eliminate_at_end_of_round: Vec<(usize, usize)>
}

impl Game {
    pub fn new() -> Self {
        Game {
            creatures: MatrixHashTable::new(),
            eliminate_at_end_of_round: Vec::new()
        }
    }
    
    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            // let x = rng.gen_range(0..800);
            // let y = rng.gen_range(0..800);
            let x = i;
            let y = i;
            let color = Color::RGB(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
            let health = rng.gen_range(0..100);
            let fullness = rng.gen_range(0..100);
            let fullness_max = rng.gen_range(0..100);
            let reproduction_time = rng.gen_range(0..100);
            let attack = rng.gen_range(0..100);
            let mut id = rng.gen_range(2..100);
            if i == 0 {
                id = 1;
            }
            let carnivor = Carnivore::new(color, health, fullness, fullness_max, reproduction_time, attack, id);
            self.add_creature(x as usize, y as usize, Box::new(carnivor));
        }
    }
    

    pub fn turn(&mut self) {
        let mut keys: Vec<(usize, usize)> = self.creatures.get_keys();

        for key in keys {
            self.move_creature(key.0, key.1);
        }

        keys = self.creatures.get_keys();

        for key in keys {
            
            if self.creatures.get(key.0, key.1).is_some() {

                self.attack(key.0, key.1);

                if self.creatures.get(key.0, key.1).unwrap().health() == false {
                    self.remove_creature(key.0, key.1);
                }

            }  else {
                continue;
            }
            
            if let Some(new_creature) = self.creatures.get(key.0, key.1).unwrap().reproduce() {
                self.add_creature(key.0, key.1, new_creature);
            }
        }

        self.update();
    }

    fn add_creature(&mut self, row: usize, col: usize, creature: Box<dyn Creature>) {
        self.creatures.insert(row, col, creature);
    }

    fn remove_creature(&mut self, row: usize, col: usize) {
        self.eliminate_at_end_of_round.push((row, col));
    }

    fn update(&mut self) {
        // println!("Eliminate at end of round: {:?}", self.eliminate_at_end_of_round);
        // println!("Eliminate at end1 of round: {:?}", self.creatures.data.keys());
        for (row, col) in &self.eliminate_at_end_of_round {
            self.creatures.delete(*row, *col);
        }
        self.eliminate_at_end_of_round.clear();
        // println!("Eliminate at end2 of round: {:?}", self.creatures.data.keys());
    }

    fn move_creature(&mut self, row: usize, col: usize) {
        let mut rng = rand::thread_rng();
        // Randomly choose a direction: 0=right, 1=left, 2=up, 3=down
        let chance = rng.gen_range(0..4);
        match chance {
            0 => {
                if row as i32 + 1 < 800 {
                    if self.creatures.get(row + 1, col).is_none() {
                        let creature = self.creatures.delete(row, col).unwrap();
                        if creature.get_id() == 1 {
                            println!("He moved from {x} {y} to {x1} {y1}", x=row, y=col, x1=row+1, y1=col);
                        }
                        self.creatures.insert(row + 1, col, creature);
                    }
                }
            } // Right
            1 => {
                if row as i32 - 1 >= 0 {
                    if self.creatures.get(row - 1, col).is_none() {
                        let creature = self.creatures.delete(row, col).unwrap();
                        if creature.get_id() == 1 {
                            println!("He moved from {x} {y} to {x1} {y1}", x=row, y=col, x1=row-1, y1=col);
                        }
                        self.creatures.insert(row - 1, col, creature);
                    }
                }
            } // Left
            2 => {
                if col as i32 + 1 < 800 {
                    if self.creatures.get(row, col + 1).is_none() {
                        let creature = self.creatures.delete(row, col).unwrap();
                        if creature.get_id() == 1 {
                            println!("He moved from {x} {y} to {x1} {y1}", x=row, y=col, x1=row, y1=col+1);
                        }
                        self.creatures.insert(row, col + 1,  creature);
                    }
                }
            } // Up
            3 => {
                if col as i32 - 1 >= 0 {
                    if self.creatures.get(row, col - 1).is_none() {
                        let creature = self.creatures.delete(row, col).unwrap();
                        if creature.get_id() == 1 {
                            println!("He moved from {x} {y} to {x1} {y1}", x=row, y=col, x1=row, y1=col-1);
                        }
                        self.creatures.insert(row, col - 1, creature);
                    }
                }
            } // Down
            _ => (), // No movement if out of bounds
        }
    }

    fn attack(&mut self, row: usize, col: usize) {
        let attack_of_creature = | creatures:&mut MatrixHashTable<Box<dyn Creature>> | -> i32 {
            if let Some(creature) = creatures.get(row, col) {
                if creature.get_id() == 1 {
                    println!("He attacked");
                }
                return creature.attack();
            }
            return 0;
        };
       
        if let Some(creature) = (self.creatures.get(row, col)).cloned() {  
            let (x, y) = (row as i32, col as i32);
            let mut rng = rand::thread_rng();
            // Randomly choose a direction: 0=right, 1=left, 2=up, 3=down
            let chance = rng.gen_range(0..4);
            match chance {
                0 => {
                    if x + 1 < 800 {
                        if self.creatures.get(x as usize + 1, y as usize).is_some() {
                            if creature.type_of_prey() == self.creatures.get(x as usize + 1, y as usize).unwrap().type_of_creature() {
                                let attack = attack_of_creature(&mut self.creatures);
                                if let Some(other_creature) = self.creatures.get(x as usize + 1, y as usize) {
                                    
                                    other_creature.reduce_health(attack);
                                    
                                }
                            }  
                        }    
                    }
                } // Right
                1 => {
                    if x - 1 >= 0 {
                        if self.creatures.get(x as usize - 1, y as usize).is_some() {
                            if creature.type_of_prey() == self.creatures.get(x as usize - 1, y as usize).unwrap().type_of_creature() {
                                let attack = attack_of_creature(&mut self.creatures);
                                if let Some(other_creature) = self.creatures.get(x as usize - 1, y as usize) {
                                   
                                    other_creature.reduce_health(attack);
                                
                                }
                            }  
                        } 
                    }
                } // Left
                2 => {
                    if y + 1 < 800 {
                        if self.creatures.get(x as usize, y as usize + 1).is_some() {
                            if creature.type_of_prey() == self.creatures.get(x as usize, y as usize + 1).unwrap().type_of_creature() {
                                let attack = attack_of_creature(&mut self.creatures);
                                if let Some(other_creature) = self.creatures.get(x as usize, y as usize + 1) {
                                    
                                    other_creature.reduce_health(attack);
                                    
                                }
                            }  
                        } 
                    }
                } // Up
                3 => {
                    if y - 1 >= 0 {
                        if self.creatures.get(x as usize, y as usize - 1).is_some() {
                            if creature.type_of_prey() == self.creatures.get(x as usize, y as usize - 1).unwrap().type_of_creature() {
                                let attack = attack_of_creature(&mut self.creatures);
                                if let Some(other_creature) = self.creatures.get(x as usize, y as usize - 1) {

                                    other_creature.reduce_health(attack);
                                    
                                }
                            }  
                        } 
                    }
                } // Down
                _ => (), // No movement if out of bounds
            }
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let keys: Vec<(usize, usize)> = self.creatures.get_keys();


        for key in keys {
            if let Some(creature) = self.creatures.get(key.0, key.1) {
                if creature.get_id() == 1 {
                    println!("He was rendered at {x} {y}", x=key.0, y=key.1);
                }
                creature.paint(canvas, (key.0 as i32, key.1 as i32));
            }
        }
    }

}
