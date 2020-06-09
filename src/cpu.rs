mod registers;

use registers::Registers;

#[derive(Debug)]
pub struct CPU{
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }
}

