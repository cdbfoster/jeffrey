//
// This file is part of Jeffrey.
//
// Jeffrey is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Jeffrey is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Jeffrey. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2017 Chris Foster
//

extern crate rand;
extern crate sdl2;

use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use event_queue::EventQueue;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Jeffrey", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .unwrap();

    let _ = canvas.set_logical_size(800, 600);

    let event_pump = sdl_context.event_pump().unwrap();
    let mut event_queue = EventQueue::new(event_pump);

    'main: loop {
        event_queue.update();
        for event in event_queue.iter() {
            match event {
                &Event::Quit { .. } |
                &Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(20, 100, 20));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(80, 20, 20));
        let _ = canvas.draw_rect(Rect::new(50, 50, 100, 50));

        canvas.present();

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}

mod event_queue;
