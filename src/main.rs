#![deny(missing_docs)]

//! A sudoku game;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub use crate::gameboard::Gameboard;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
mod gameboard;

fn main() {
    let opengl = OpenGL::V3_2;
    let setting = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = setting.build().expect("Cloud not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
            });
        }
    }
}
