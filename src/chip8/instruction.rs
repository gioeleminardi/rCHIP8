trait Runnable {
    fn run();
}

enum InstructionType {}

struct Instruction {
    id: InstructionType,
}

// impl Instruction {
//     fn new() -> Instruction {
//         ()
//     }
// }

impl Runnable for Instruction {
    fn run() {
        todo!()
    }
}