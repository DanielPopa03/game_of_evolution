use sdl2::{event::Event, pixels::Color, Sdl};
use game_of_evolution::structs::game::Game;

fn main() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL2", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut running = true;

    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut game = Game::new();
    game.init();
  
    

    {
        let mut i = 0;
        while running {
            
            i += 1;
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        println!("{}", i);
                        running = false;
                    }
                    _ => {}
                }
            }
    
            game.turn();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            game.render(&mut canvas);
           
            canvas.present();
            
            
            // Wait for a short duration to control the speed of the loop
            ::std::thread::sleep(::std::time::Duration::from_millis(1000));
        } 
    }

}
