// Dependencies go here
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;

pub mod lib;
use crate::lib::snake;
use crate::lib::types;

// this is main
fn main() {
    let canvas_width: u32 = 720;
    let canvas_height: u32 = 720;
    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let (columns, rows) = (canvas_width/80 , canvas_height/80);
    let cell_width = columns * rows;
    let mut grid = lib::grid_init(columns, rows);
    let mut direction = (1, 0);
    let mut s = snake::init(1 ,1, types::Cell{red: 2,green:2,blue: 2});
    thread::spawn(move || { // move puts the variables used to inside the thread
        // some work here
    });
    'game: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    .. // dont care about the rest of the information that this provides
                } => break 'game, // we name it here so it breaks the outer loop `game 
                // if we didn't explicity say where we want to break it would break the for loop
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = (-1, 0);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = (1, 0);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    direction = (0, -1);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = (0, 1);
                },
                _ => continue 'game,
            }
        }
        s = snake::update_position(&mut s, direction);
        grid = snake::fill_move(grid, &s);
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);

        thread::sleep(time::Duration::from_millis(800));
    }
}
