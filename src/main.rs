mod cpu;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;
const SCALE_X: u32 = WIDTH / 64;
const SCALE_Y: u32 = HEIGHT / 32;
const CLOCK: u32 = 60; // Hz

fn main() {
    let mut cpu = cpu::Cpu::new();

    // cpu.load_rom("rom/IBMLogo.ch8").expect("Error reading rom");
    // cpu.load_rom("rom/test_opcode.ch8").expect("Error reading rom");
    cpu.load_rom("rom/c8games/INVADERS").expect("Error reading rom");

    // println!("{:#02X?}", cpu);

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
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Num1) => cpu.key_press(0x1),
                    Some(Keycode::Num2) => cpu.key_press(0x2),
                    Some(Keycode::Num3) => cpu.key_press(0x3),
                    Some(Keycode::Num4) => cpu.key_press(0xC),

                    Some(Keycode::Q) => cpu.key_press(0x4),
                    Some(Keycode::W) => cpu.key_press(0x5),
                    Some(Keycode::E) => cpu.key_press(0x6),
                    Some(Keycode::R) => cpu.key_press(0xD),

                    Some(Keycode::A) => cpu.key_press(0x7),
                    Some(Keycode::S) => cpu.key_press(0x8),
                    Some(Keycode::D) => cpu.key_press(0x9),
                    Some(Keycode::F) => cpu.key_press(0xE),

                    Some(Keycode::Z) => cpu.key_press(0xA),
                    Some(Keycode::X) => cpu.key_press(0x0),
                    Some(Keycode::C) => cpu.key_press(0xB),
                    Some(Keycode::V) => cpu.key_press(0xF),
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::Num1) => cpu.key_release(0x1),
                    Some(Keycode::Num2) => cpu.key_release(0x2),
                    Some(Keycode::Num3) => cpu.key_release(0x3),
                    Some(Keycode::Num4) => cpu.key_release(0xC),

                    Some(Keycode::Q) => cpu.key_release(0x4),
                    Some(Keycode::W) => cpu.key_release(0x5),
                    Some(Keycode::E) => cpu.key_release(0x6),
                    Some(Keycode::R) => cpu.key_release(0xD),

                    Some(Keycode::A) => cpu.key_release(0x7),
                    Some(Keycode::S) => cpu.key_release(0x8),
                    Some(Keycode::D) => cpu.key_release(0x9),
                    Some(Keycode::F) => cpu.key_release(0xE),

                    Some(Keycode::Z) => cpu.key_release(0xA),
                    Some(Keycode::X) => cpu.key_release(0x0),
                    Some(Keycode::C) => cpu.key_release(0xB),
                    Some(Keycode::V) => cpu.key_release(0xF),
                    _ => {}
                },
                _ => {}
            }
        }

        cpu.run();

        canvas.set_draw_color(Color::GREEN);

        // if cpu.draw() {
        for row in 0..32 {
            for col in 0..64 {
                if cpu.vram(col as usize, row as usize) != 0 {
                    let rect = Rect::new((col * SCALE_X) as i32, (row * SCALE_Y) as i32, SCALE_X, SCALE_Y);
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }
        // }

        // break 'running;

        canvas.present();
        ::std::thread::sleep(Duration::from_micros(1000000 / CLOCK as u64));
    }
}
