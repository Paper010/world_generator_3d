use macroquad::prelude::*;

const PLAYER_SIZE:Vec2 = const_vec2!([150f32,40f32]);
const PLAYER_SPEED:f32 = 15.0;

const BALL_SIZE:f32 = 20f32;
const BALL_SPEED:f32 = 10.0;

const BLOCK_SIZE:Vec2 = const_vec2!([100f32,20f32]);

enum State {
    Over,
    Start,
    Running,
    Null
}

struct Player {
    rect:Rect,
}

struct Ball {
    rect:Rect, //although a ball, use rect cus easy, draw ball though
    vel: Vec2
}

struct Block {
    rect:Rect,
    lives: u32,
}

impl Player {
    pub fn new()->Self{
        Self {
        rect: Rect::new(
                  screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                  screen_height() - 100.0,
                  PLAYER_SIZE.x,
                  PLAYER_SIZE.y
                  )
        }
    }

   pub fn update(&mut self){
        let dir = match (is_key_down(KeyCode::A )||is_key_down(KeyCode::Left),is_key_down(KeyCode::D)||is_key_down(KeyCode::Right)){
            (true,false)=>-1f32,
            (false,true)=> 1f32,
            _=> 0f32
        };
         
        self.rect.x += dir * PLAYER_SPEED;
        if self.rect.x < 0f32{
            self.rect.x = 0f32;
        }
        if self.rect.x  > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        } 
   }
   
   pub fn draw(&mut self){
        draw_rectangle(self.rect.x,self.rect.y,PLAYER_SIZE.x,PLAYER_SIZE.y,WHITE);
   }

}

impl Ball {
    pub fn new(pos:Vec2)->Self{
        Self
        {
           rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
           vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }   
    pub  fn draw(&mut self){
        draw_circle(self.rect.x,self.rect.y,BALL_SIZE,WHITE);
    }

    pub fn update(&mut self,) {
        self.rect.x += self.vel.x * BALL_SPEED;
        self.rect.y += self.vel.y * BALL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
        
    }



}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                 pos.x,
                 pos.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y,
                 
                ),
           lives: 1
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x,self.rect.y,self.rect.w, self.rect.h, GRAY);
    }
}

fn check_collision(a:&mut Rect,b:&mut Rect,vel:&mut Vec2)-> bool {
    let intersection = match a.intersect(*b){
        Some(intersection)=> intersection,
        None => return false,
    };

    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();

    match intersection.w > intersection.h {
        true => {
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false =>{
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
        
    }
    true
    
}

fn gen_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5.5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(
            board_start_pos + vec2(block_x, block_y)
        ));
    }
}


#[macroquad::main("Arkanoid")]
async fn main(){
    let mut player:Player = Player::new();
    let mut game_state = State::Start;
    let mut balls:Vec<Ball> = Vec::new(); 
    let mut blocks:Vec<Block> = Vec::new();

    loop {
       
        match game_state {
        State::Start =>{
           *&mut balls.push(Ball::new(
                player.rect.point() + vec2(player.rect.w * 0.5f32 - BALL_SIZE * 0.5f32, -50f32
                 )),
            );
           
            gen_blocks(&mut blocks);
            
           clear_background(BLACK);
            
            player.update();
            player.draw();
            
            for ball in &mut balls[0..]{
                ball.draw();
            };
           
            for block in &blocks[0..]{
                block.draw();
            };
            
            game_state = State::Running;
        },
        
        State::Over =>{
            return println!("Game Over");
            },

        State::Running=>{
                    clear_background(BLACK);
                    player.update();
                    player.draw();
                    for ball in &mut balls[0..]{
                        ball.draw();
                        ball.update();
                        check_collision(&mut ball.rect, &mut player.rect,&mut ball.vel); 
                        for block in &mut blocks[0..]{
                            if check_collision(&mut ball.rect, &mut block.rect, &mut ball.vel) {
                                block.lives -= 1;    
                            };
                        };       
                    };
                     &mut blocks.retain(|b| b.lives > 0);
                                
                    balls.retain(|ball| ball.rect.y < screen_height());
                    for block in &blocks[0..]{
                        block.draw();

                    };
        },
             _=> return,
        }    
        if balls.is_empty(){
            game_state= State::Over;   
        };
        next_frame().await;
    }
}

