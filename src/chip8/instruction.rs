trait Runnable {
    fn run();
}

enum InstructionType {
    Subroutine,
    Jump,
    Skip,
    Set,
    Add,
    LogicSet,
    BinaryOr,
    BinaryAnd,
    BinaryXor,
    LogicalXor,
    AluAdd,
    Subtract,
    Shift,
    SetIndex,
    JumpOffset,
    Random,
    Display,
    SkipIfKey,
    Timers,
    AddToIndex,
    GetKey,
    FontChar,
    BinaryCodedDecimalConversion,
    StoreAndLoadMemory,
}

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