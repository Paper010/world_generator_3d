
use macroquad::prelude::*;

const MOVE_SPEED: f32 = 1.0;
const LOOK_SPEED: f32 = 0.1;


#[macro_use] extern crate worldgen;

use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{NoiseMapGenerator, NoiseMapGeneratorBase, NoiseMap, Seed, Step, Size};
use worldgen::world::{World, Tile};
use worldgen::world::tile::{Constraint, ConstraintType};



#[macroquad::main("3D")]
async fn main() {
    let noise = PerlinNoise::new();
    let mut string:String = String::from("");

   

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello?"))
        .set(Step::of(0.005, 0.005));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05));

    let nm = Box::new(nm1 + nm2 * 3);

    let world = World::new()
        .set(Size::of(80, 50))

        // Water
        .add(Tile::new('~')
            .when(constraint!(nm.clone(), < -0.1)))

        // Grass
        .add(Tile::new(',')
            .when(constraint!(nm.clone(), < 0.45)))

        // Mountains
        .add(Tile::new('^')
            .when(constraint!(nm.clone(), > 0.8)))

        // Hills
        .add(Tile::new('n'));

    for row in world.generate(0, 0).iter() {
        for val in row.iter() {
            for c in val.iter() {
                string += &c.to_string();
            }

            string += "\n";
        }

        string += "\n";
    }

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::Up) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Right) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }
        // Going 3d! 
        let mut z:f32 = -10.0;
        
        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        clear_background(SKYBLUE);
      
        
        for line in string.lines() {
            z += 1f32;
            let mut x:f32 = 1f32;
             for char in line.chars() {
                x += 1f32;
                let mut y:f32=0f32;
                /* 
                    ~ = y-1
                    , = y
                    n = y+1
                    ^ = y+2 
                */  
                if char == 'n' {
                    y = 2f32;
                }else if char ==',' {
                    y = 1f32;
                }else if char == '~'{
                    y = -1f32;
                }else if char == ','{
                    y = 0f32;
                };
    
                //draw_cube_wires(vec3(x, y, z), vec3(2., 2., 2.),WHITE);
                
                if y <= -1f32 {
                draw_cube(vec3(x, y, z), vec3(2., 2., 2.),None,BLUE);
                }
                if y == 1f32 || y == 0f32{ 
                draw_cube(vec3(x, y, z), vec3(2., 2., 2.),None,GREEN);
                }
                if y >=2f32 {
                 draw_cube(vec3(x, y, z), vec3(2., 2., 2.),None,BLACK);
                } 
            }
        };
        
    
        set_default_camera();
        next_frame().await;
     



    }
}