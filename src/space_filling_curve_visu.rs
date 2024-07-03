extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use crate::exercises::reverse_map::reverse_map_dim;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    max_p : f64
}

fn float_to_rgb(value: f32) -> [f32; 4] {
  let hue = 360.0 * value; // Map float value to hue range [0, 360]
  let saturation = 1.0; // Set saturation to 100% (full saturation)
  let value = 1.0; // Set value to 100% (full value/brightness)

  // Convert HSV to RGB
  let chroma = value * saturation;
  let hue_segment = hue / 60.0;
  let x = chroma * (1.0 - (hue_segment % 2.0 - 1.0).abs());

  let (r, g, b) = if hue >= 0.0 && hue < 60.0 {
      (chroma, x, 0.0)
  } else if hue >= 60.0 && hue < 120.0 {
      (x, chroma, 0.0)
  } else if hue >= 120.0 && hue < 180.0 {
      (0.0, chroma, x)
  } else if hue >= 180.0 && hue < 240.0 {
      (0.0, x, chroma)
  } else if hue >= 240.0 && hue < 300.0 {
      (x, 0.0, chroma)
  } else {
      (chroma, 0.0, x)
  };

  let m = value - chroma;
  [r + m, g + m, b + m, 1.0] // Add back in the brightness offset and set alpha to 1.0
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let draw_speed = 1;
        let dim = 2;
        let nb : u64 = (1 << (dim * 2)) - 1;
        let step = 1.0 / (nb as f64);
        let mut p = 0.0;
       
        
        self.gl.draw(args.viewport(), |c, gl| {
            
            clear(BACKGROUND, gl);

            self.max_p += step * ((draw_speed) as f64);
            if self.max_p >= 1.0 {
              self.max_p = 1.0;
            }

       
            
            while p <= self.max_p {
              p += step;
              let (x1, y1) = reverse_map_dim(p - step, dim);
              let (x2 , y2) = reverse_map_dim(p, dim);
              
              let resize_x = (args.width as f64) / ((1 << dim) as f64);
              let resize_y = (args.height as f64) / ((1 << dim) as f64);
    
              let x1 = (x1 as f64) * resize_x;
              let x2 = (x2 as f64) * resize_x;
              let y1 = (y1 as f64) * resize_y;
              let y2 = (y2 as f64) * resize_y;
              let line_unit = [x1, y1, x2, y2];
              let color = float_to_rgb(p as f32);
              line(color, 1.0, line_unit, c.transform.trans(0.0, 0.0), gl);
              line(color, 1.0, line_unit, c.transform.trans(0.0, 0.0), gl);
            }

         
   

            
        });
    }
    
}

pub fn run() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("hilbert-curve", [1024, 1024])
        /* .opengl(opengl) */
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        max_p : 0.0
    };
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
          app.render(&r);
        }
    }
}
