extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;


struct Game 
{
    gl: GlGraphics,

    rows: i32,
    cols: i32,


    lpad: Pad,
    rpad: Pad,

    //ball: Ball,

    //counter: Counter,

}


impl Game
{
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        const GRAY: [f32; 4] = [0.2, 0.2, 0.2, 0.2];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GRAY, gl);
        });

        self.lpad.render(&mut self.gl, arg);
        self.rpad.render(&mut self.gl, arg);

    }

}


#[derive(Clone, PartialEq)]
enum Direction
{
    Up, Down, Null,
}



struct Pad
{
    lenght: Vec<(i32,i32)>,
    dir: Direction
}



impl Pad
{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.lenght
            .iter()
            .map(|(x,y)| {

                graphics::rectangle::square(
                    (x*20) as f64,
                    (y*20) as f64,
                    20_f64)
            })
            .collect();


        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter()
                .for_each(|square| graphics::rectangle(WHITE, square, transform, gl));
        });

    }

}

fn main() {

    let opengl = OpenGL::V3_2;

    //let n1: i32 = rand::thread_rng().gen_range(1, 10);
    //let n2: i32 = rand::thread_rng().gen_range(1, 10);
   

    //println!("N1: {} N2: {}", n1, n2);


    let mut window: GlutinWindow = WindowSettings::new(
            "snake game",
            [600,400]
            ).opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();

    let mut game = Game {
        
        gl: GlGraphics::new(opengl),
        
        cols: 40,
        rows: 60,

        lpad: Pad {
            lenght: vec![(10,200), (10,201)],
            dir: Direction::Null,
        },

        rpad: Pad {
            lenght: vec![(590,200), (590,201)],
            dir: Direction::Null,
        }
    
   };



    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        
        /*
        if let Some(u) = e.update_args() {
            game.update(&u);

        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

        if let Some(d) = e.update_args() {
            if game.die() {
                break;       
            }
        }
        */
    }
}
