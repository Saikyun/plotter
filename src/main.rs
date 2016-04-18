extern crate rand;
extern crate sdl2;
extern crate sdl2_image;

use rand::Rng;
use std::path::Path;

use sdl2_image::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

fn max(v1: i32, v2: i32) -> i32 {
    if v1 >= v2 { v1 } else { v2 }
}

fn min(v1: i32, v2: i32) -> i32 {
    if v1 >= v2 { v1 } else { v2 }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2_image::init(INIT_PNG).unwrap();
    let mut timer = sdl_context.timer().unwrap();

    let height = 720;

    let window = video_subsystem.window("Plotter", 1280, height as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let pen_tex = renderer.load_texture(Path::new("assets/pen.png")).unwrap();
    let eraser_tex = renderer.load_texture(Path::new("assets/eraser.png")).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut offset = 0;
    let starting_position = height / 2;
    let mut points: Vec<i32> = vec![starting_position];

    let interval = 1_000 / 60;
    let mut before = timer.ticks();

    let step_size = 3;

    'running: loop {
        let dt = timer.ticks() - before;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = timer.ticks();

        offset += step_size;

        let last_y = *points.last().unwrap();
        let mut rng = rand::thread_rng();
        let new_point = rng.gen_range(max(last_y - 5, -height/2), min(last_y + 5, height/2));
        points.push(new_point);

        renderer.set_draw_color(Color::RGB(40, 40, 35));
        renderer.clear();

        for pos in 1..points.len() {
            renderer.set_draw_color(Color::RGB(230, 220, 230));
            renderer.draw_line(
                Point::new((pos as i32 - 1) * step_size - offset + 1280 / 2, *points.get(pos - 1).unwrap()),
                Point::new((pos as i32) * step_size - offset + 1280 / 2, *points.get(pos).unwrap())
            ).unwrap();
        }

        let len = points.len();
        if len * step_size as usize > 1200 / 2 {
            points.drain(0..len - ((1200 / 2) / step_size) as usize);
            offset = step_size * (points.len() as i32 - 1);
        }

        renderer.copy(&pen_tex, None, Some(sdl2::rect::Rect::new(1280 / 2, *points.last().unwrap() - pen_tex.query().height as i32, pen_tex.query().width, pen_tex.query().height)));
        renderer.copy(&eraser_tex, None, Some(sdl2::rect::Rect::new(40 - eraser_tex.query().width as i32, *points.first().unwrap() - eraser_tex.query().height as i32 + 14, eraser_tex.query().width, eraser_tex.query().height)));

        renderer.present();
    }
}
