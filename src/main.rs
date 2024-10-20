use raylib::prelude::*;

struct Grid {
    size : i32,
    color : Color
}

impl Grid {
    fn draw(self : &Grid, width : i32, height : i32, r : &mut RaylibDrawHandle) {
       
        for v in 0..width {    
            r.draw_line(v * self.size, 0, v * self.size, height, self.color);
        }
    
        for h in 0..height {             
            r.draw_line(0, h * self.size, width, h * self.size, self.color);
        }
    }
}

fn random(min : f32, max : f32) -> f32 {    
    unsafe {
        let r =  raylib::ffi::GetRandomValue(min as i32, max as i32);
        r as f32        
    }
}


struct Metaball {
    position : Vector2,
    size : f32,    
    vel : Vector2,
    limit : Vector2
}

impl Metaball {
    fn new(limit: Vector2) -> Metaball {      

        let size  = random(5., 30.);
        let position = Vector2 { x : random(size, limit.x - size),  y : random(size, limit.y - size) };
 
        Metaball {
            position, size, 
            vel : Vector2 { x : random(1., 100.) , y: random(1., 100.) }, 
            limit
        }
    }  

    fn new_pos(position : Vector2, limit: Vector2)  -> Metaball {      

        let size  = random(15., 50.);
 
        Metaball {
            position, size, 
            vel : Vector2 { x : random(1., 100.) , y: random(1., 100.) }, 
            limit
        }
    }  

    fn dist(self : &Metaball, x: f32, y : f32 ) -> f32{         

        let p = Vector2 { x , y } ;
        let d = self.position - p;
        let s = d.length_sqr();
        if s == 0.0 { return 0.0; }
        let d = 1.0 * self.size /  s.sqrt();
        return d;
    }      

    fn update(self : &mut Metaball, elapsed : f32) {
        self.position += self.vel * elapsed;
        
        if self.position.x >= self.limit.x && self.vel.x > 0. {
            self.vel.x = -1.0 * self.vel.x.abs();
        } else if self.position.x < 0.  && self.vel.x < 0.{
            self.vel.x = self.vel.x.abs();
        }

        if self.position.y >= self.limit.y && self.vel.y > 0. {
            self.vel.y = -1.0 * self.vel.y.abs();
        } else if self.position.y < 0. && self.vel.y < 0. {
            self.vel.y = self.vel.y.abs();
        }
    }   
}

fn main() {

    let screen = (800, 600);    
    let buf_len = screen.0 * screen.1;
    let bounds = Vector2 { x : screen.0 as f32, y : screen.1 as f32 };

    let (mut rl, thread) = raylib::init()       
        .size(screen.0, screen.1)
        .title("Hello, Metaballs")
        .build();


    let mut blobs = vec![ 
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds),
        Metaball::new(bounds)      
    ];


    let img = Image::gen_image_color(screen.0, screen.1, Color::WHITE);    
    let mut texture = rl.load_texture_from_image(&thread, &img).unwrap();

    let grid = Grid { color : Color::WHITE, size : 50 };


    while !rl.window_should_close() {        

        let elapsed = rl.get_frame_time();

        for b in blobs.iter_mut() {
            b.update(elapsed);
        }

        let buf = unsafe { std::slice::from_raw_parts_mut(img.data as *mut u8, (buf_len * 4) as usize) };
           
        for x in 0..screen.0 {
            for y in 0..screen.1 {


                let mut d = 0.0;

                for b in blobs.iter() {
                    d += b.dist(x as f32, y as f32) * 50.0;                       
                }

                let u: u8 = if d.clamp(0.0, 255.0) < 100.0 { 0 } else { 200 };
                let x_f = (4 * x) as usize;
                let y_f = (4 * y * screen.0) as usize;

                buf[x_f + 0 + y_f ] = u;
                buf[x_f + 1 + y_f ] = u;
                buf[x_f + 2 + y_f ] = u;
                buf[x_f + 3 + y_f ] = 255;
            }                
        }

        Texture2D::update_texture(&mut texture, buf);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0,  Color::WHITE);

        grid.draw(screen.0, screen.1, &mut d);    
        d.draw_fps(screen.0 - 100, screen.1 - 50);

        if d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {           
            blobs.push(Metaball::new_pos(d.get_mouse_position(), bounds));
        }
        
    }
}
