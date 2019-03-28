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

    ball: Ball,

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
        self.ball.render(&mut self.gl, arg);
    }


    fn update(&mut self, arg: &UpdateArgs){
       self.ball.update(&mut self.gl, arg);

        if (self.ball.a_pos.1 <= 0) || (self.ball.a_pos.1 >= (self.cols - 1)){
            self.ball.up_collision();
        }
/*
        if (self.ball.a_pos.0 <= ) || (self.ball.a_pos.0 >= cols){
            ball.lateral_collision();
        }

        if (self.ball.a_pos.0 <= 0) || (self.ball.a_pos.0 >= cols){
            ball.lateral_collision();
        }

*/

    }
  

    fn pressed(&mut self, btn: &Button){
        
        self.rpad.dir = match btn {
            &Button::Keyboard(Key::Up) =>
                Direction::Up,
            &Button::Keyboard(Key::Down) =>
                Direction::Down,
            _ => Direction::Null
        };
 
        self.lpad.dir = match btn {
            &Button::Keyboard(Key::W) =>
                Direction::Up,
            &Button::Keyboard(Key::S) =>
                Direction::Down,
            _ => Direction::Null
        };

        self.lpad.mov = true;
        self.rpad.mov = true;
        self.lpad.update();
        self.rpad.update();
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
    dir: Direction,
    
    mov: bool
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



    fn update(&mut self){
        
        if self.mov == true {

            match self.dir {
                Direction::Up => {
                    self.lenght[0].1 = self.lenght[0].1 - 1;
                    self.lenght[1].1 = self.lenght[1].1 - 1;
                }
                Direction::Down => {
                    self.lenght[0].1 = self.lenght[0].1 + 1;
                    self.lenght[1].1 = self.lenght[1].1 + 1;
                }
                Direction::Null => { }
            }
            self.mov = false;
        }
    }

}



struct Ball
{
    p_pos: (i32,i32),
    a_pos: (i32,i32),
    n_pos: (i32,i32),

    impact: bool,
    dir: (i32,i32),

}


impl Ball
{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        
        let IDK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
  
        let square = graphics::rectangle::square(
            (self.a_pos.0 * 10) as f64,
            (self.a_pos.1 * 10) as f64,
            20_f64);


        gl.draw(args.viewport(), |c_,gl| {
            let transform = c_.transform;

            graphics::rectangle(IDK, square, transform, gl);
        });
     }


    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs){
        
        self.a_pos.0 = self.a_pos.0 + self.dir.0;
        self.a_pos.1 = self.a_pos.1 + self.dir.1;

    }

    fn up_collision(&mut self){
        self.dir.1 = -self.dir.1;
    }
/*
    fn lateral_collision(&mut self){
        self.dir.1 = -self.dir.1;
    }
*/
}


fn main() {

    let opengl = OpenGL::V3_2;

//    let n1: i32 = rand::thread_rng().gen_range(-3, 3);
//    let n2: i32 = rand::thread_rng().gen_range(-3, 3);
   
    let n1: i32 = 1;
    let n2: i32 = 1;
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
            lenght: vec![(1,9), (1,10)],
            dir: Direction::Null,
            mov: false,
        },

        rpad: Pad {
            lenght: vec![(28,9), (28,10)],
            dir: Direction::Null,
            mov: false,
        },

        ball: Ball {
            p_pos: (15,10),
            a_pos: (15,10),
            n_pos: (15,10),
            
            dir: (n1, n2),
            impact: false,
        }
    
   };



    let mut events = Events::new(EventSettings::new()).ups(2);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        
        
        if let Some(u) = e.update_args() {
            game.update(&u);

        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

        /*
        if let Some(d) = e.update_args() {
            if game.die() {
                break;       
            }
        }
        */
    }
}
