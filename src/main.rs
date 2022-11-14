mod chip8;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;
const CLOCK: u32 = 60; // Hz

fn main() {
    let mut chip8 = chip8::Chip8::new();

    chip8
        .load_rom("rom/IBMLogo.ch8")
        .expect("Error reading rom");

    // println!("{:#02X?}", chip8);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rCHIP-8", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::Num1) => chip8.key_press(0x1),
                        Some(Keycode::Num2) => chip8.key_press(0x2),
                        Some(Keycode::Num3) => chip8.key_press(0x3),
                        Some(Keycode::Num4) => chip8.key_press(0xC),

                        Some(Keycode::Q) => chip8.key_press(0x4),
                        Some(Keycode::W) => chip8.key_press(0x5),
                        Some(Keycode::E) => chip8.key_press(0x6),
                        Some(Keycode::R) => chip8.key_press(0xD),

                        Some(Keycode::A) => chip8.key_press(0x7),
                        Some(Keycode::S) => chip8.key_press(0x8),
                        Some(Keycode::D) => chip8.key_press(0x9),
                        Some(Keycode::F) => chip8.key_press(0xE),

                        Some(Keycode::Z) => chip8.key_press(0xA),
                        Some(Keycode::X) => chip8.key_press(0x0),
                        Some(Keycode::C) => chip8.key_press(0xB),
                        Some(Keycode::V) => chip8.key_press(0xF),
                        _ => {}
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::Num1) => chip8.key_release(0x1),
                        Some(Keycode::Num2) => chip8.key_release(0x2),
                        Some(Keycode::Num3) => chip8.key_release(0x3),
                        Some(Keycode::Num4) => chip8.key_release(0xC),

                        Some(Keycode::Q) => chip8.key_release(0x4),
                        Some(Keycode::W) => chip8.key_release(0x5),
                        Some(Keycode::E) => chip8.key_release(0x6),
                        Some(Keycode::R) => chip8.key_release(0xD),

                        Some(Keycode::A) => chip8.key_release(0x7),
                        Some(Keycode::S) => chip8.key_release(0x8),
                        Some(Keycode::D) => chip8.key_release(0x9),
                        Some(Keycode::F) => chip8.key_release(0xE),

                        Some(Keycode::Z) => chip8.key_release(0xA),
                        Some(Keycode::X) => chip8.key_release(0x0),
                        Some(Keycode::C) => chip8.key_release(0xB),
                        Some(Keycode::V) => chip8.key_release(0xF),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        chip8.run();



        canvas.present();
        ::std::thread::sleep(Duration::from_millis((1_000 / CLOCK) as u64));
    }
}
