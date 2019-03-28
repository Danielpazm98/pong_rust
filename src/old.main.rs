struct Game 
{
    gl: GlGraphics,

    rows: i32,
    cols: i32,

    //snake: Snake,
    //just_eaten: bool,

    lpad: pad,
    rpad: pad,

    ball: Ball,

    counter: Counter,

    //food: Food,

    //score: u32,
}


impl Game
{
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.food.render(&mut self.gl, arg);
        self.snake.render(&mut self.gl, arg);
        
    }


    fn update(&mut self, arg: &UpdateArgs){
        self.snake.update();
        
        if &self.snake.head == &self.food.pos {
           self.snake.grow();
           self.food.update(&mut self.gl, arg);
        }   
    }


    fn pressed(&mut self, btn: &Button){

        let last_direction = self.snake.dir.clone();


        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    
    }

    fn die(&mut self) -> bool{

    
        if (&self.snake.head.0 >= &self.cols) || (&self.snake.head.0 < &0) || (&self.snake.head.1 >= &self.rows) || (&self.snake.head.1 < &0) {
            println!("has muerto");
            return true;
        }

        for element in self.snake.body.iter() {
            
            if element == &self.snake.head { 
                return true;
            }

        }

    return false;

    }

    
}


#[derive(Clone, PartialEq)]
enum Direction
{
    Right, Left, Up, Down,
}



struct Snake
{

    body: LinkedList<(i32, i32)>,
    head: (i32,i32),
    
    dir: Direction

}


impl Snake
{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        
 
        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        
        let squares: Vec<graphics::types::Rectangle> = self.body
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
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });


                }

    fn update(&mut self){
       
        let mut new_head = (self.head).clone();

        //self.head = (*self.body.front().expect("Snake has no body 2!")).clone();
            
        match self.dir {
            Direction::Left =>  new_head.0 = self.head.0 - 1,
            Direction::Right => new_head.0 = self.head.0 + 1,
            Direction::Up =>    new_head.1 = self.head.1 - 1,
            Direction::Down =>  new_head.1 = self.head.1 + 1,
        }

        self.body.push_front(self.head);
        
        self.head = new_head;

        self.body.pop_back().unwrap();

    }


    fn grow(&mut self){

      let mut new_head = (self.head).clone();

        //self.head = (*self.body.front().expect("Snake has no body 2!")).clone();
            
        match self.dir {
            Direction::Left =>  new_head.0 = self.head.0 - 1,
            Direction::Right => new_head.0 = self.head.0 + 1,
            Direction::Up =>    new_head.1 = self.head.1 - 1,
            Direction::Down =>  new_head.1 = self.head.1 + 1,
        }

        self.body.push_front(self.head);
        
        self.head = new_head;

    }

}



struct Food
{
    pos: (i32, i32)    
}



impl Food
{

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;
        
        let IDK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
  
        let square = graphics::rectangle::square(
            (self.pos.0 * 20) as f64,
            (self.pos.1 * 20) as f64,
            20_f64);


        gl.draw(args.viewport(), |c_,gl| {
            let transform = c_.transform;

            graphics::rectangle(IDK, square, transform, gl);
        });
     }

    fn update(&mut self, gl: &mut GlGraphics, args: &UpdateArgs){

        let n1: i32 = rand::thread_rng().gen_range(1, 10);
        let n2: i32 = rand::thread_rng().gen_range(1, 10);
        self.pos.0 = n1;
        self.pos.1 = n2;
        
        println!("comida++");

    }

}


fn main() {

    let opengl = OpenGL::V3_2;

    let n1: i32 = rand::thread_rng().gen_range(1, 10);
    let n2: i32 = rand::thread_rng().gen_range(1, 10);
   

    println!("N1: {} N2: {}", n1, n2);


    let mut window: GlutinWindow = WindowSettings::new(
            "snake game",
            [600,400]
            ).opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();

    let mut game = Game {
        
        cols: 10,
        rows: 10,

        gl: GlGraphics::new(opengl),

        snake: Snake { 
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
            dir: Direction::Right,
            head: (0,2),
        },


        food: Food {
            pos: (n1, n2),
        }
    };



    let mut events = Events::new(EventSettings::new()).ups(5);
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

        if let Some(d) = e.update_args() {
            if game.die() {
                break;       
            }
        }

    }
}

