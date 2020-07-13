use keypad::Keypad;
use display::{Display, FONT_SET};
use rand::ComplementaryMultiplyWithCarryGen;

pub struct Cpu {
    //index register
    pub i: u16,
    //program counter
    pub pc: u16,
    //memory
    pub memory: [u8; 4096],
    //registers
    pub v: [u8; 15],
    //peripherals
    pub keypad: Keypad,
    pub display: Display,
    //stack
    pub stack: [u16; 16],
    //stack pointer
    pub sp: u8,
    //delay timer
    pub dt: u8,
    //rand no gen
    pub rand: ComplementaryMultiplyWithCarryGen


}

fn read_word(memory: [u8; 4096], index:u16) -> u16 {
    (memory[index as usize] as u16) << 8
        | (memory[(index + 1) as usize] as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; 16],
            sp: 0,
            dt: 0,
            rand: ComplementaryMultipleWithCarryGen::new(1)
        }
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200;
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.stack = [0; 16];
        self.sp = 0;
        self.dt = 0;
        self.rand = ComplementaryMultipleWithCarryGen::new(1);
    }
}