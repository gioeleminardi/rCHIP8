mod chip8;

fn main() {
    let mut chip8 = chip8::Chip8::new();

    chip8.load_rom("rom/IBMLogo.ch8").expect("Error reading rom");

    println!("{:#02X?}", chip8);

    chip8.run();
    chip8.run();
    chip8.run();
    chip8.run();
    chip8.run();
    chip8.run();
    chip8.run();
    chip8.run();
}