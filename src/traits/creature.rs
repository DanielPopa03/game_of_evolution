use sdl2::{render::Canvas, video::Window};

pub trait Creature{

    fn health(&mut self) -> bool;
    
    fn paint(&self, canvas: &mut Canvas<Window>, coordinates: (i32, i32));

    fn reproduce(&mut self) -> Option<Box<dyn Creature>>;

    fn add_health(&mut self, amount: i32) -> i32; 

    fn reduce_health(&mut self, amount: i32) -> i32;

    fn type_of_creature(&self) -> String;

    fn type_of_prey(&self) -> String;

    fn can_move_and_attack(&self) -> bool;
    
    fn attack(&mut self) -> i32; 

    fn get_id(&self) -> i32; 

    fn show(&self);
    fn clone_box(&self) -> Box<dyn Creature>;
}

impl Clone for Box<dyn Creature> {
    fn clone(&self) -> Box<dyn Creature> {
        self.clone_box()
    }
}