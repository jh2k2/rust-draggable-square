extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    coord: [f64; 2]
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let rotation = self.rotation;
        let (x, y) = (self.coord[0], self.coord[1]);

        self.coord = [x,y];
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, [0.0, 0.0, 50.0, 50.0], transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn get_coord(&mut self) -> [f64; 2] {
        self.coord
    }

    fn drag(&mut self, mouse_coord: [f64; 2]) {
        self.coord = mouse_coord;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("app", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        coord: [100.0, 100.0]
    };

    let mut cursor = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    let mut held = false;
    let mut pos_diff = [0.0, 0.0];

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            let distance =((app.get_coord()[0] - cursor[0]).powi(2) + (app.get_coord()[1]-cursor[1]).powi(2)).sqrt().abs();
            if button == MouseButton::Left && distance < 50.0 {
                pos_diff = [app.get_coord()[0] - cursor[0], app.get_coord()[1]-cursor[1]];
                held = true;
            }
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                held = false;
            }
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        e.mouse_cursor(|pos| {
            cursor = pos;
        });

        if held {
            app.drag([cursor[0] + pos_diff[0], cursor[1] + pos_diff[1]]);
        }
    }
}