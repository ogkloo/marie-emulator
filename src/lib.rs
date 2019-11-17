use std::io;
use std::io::prelude::*;

/// MARIE has a 12 bit address space, which is weird, but so goes. The reasoning for this is so
/// that full instructions can fit in a 16 bit space.
/// The ROM for this hypothetical machine is defined as a 1KB space for a "kernel" to fit in. Use
/// this memory carefully as it's very limited. It's defined as 16 bit because the instructions are
/// 16 bits long and this is done for convenience.
/// Debug is not derived on account of dumping the entire memory is a really bad idea.
pub struct Marie {
    // 12 bit address space, so 2^12 = 4096 bytes.
    volatile_memory: [i16; 4096],
    // The 16 bit accumulator and program counter.
    ac: i16,
    pc: u16,
}

impl Default for Marie {
    fn default() -> Self {
        Marie {
            volatile_memory: [0; 4096],
            ac: 0,
            pc: 0,
        }
    }
}

#[allow(dead_code)]
impl Marie {
    fn jns(&mut self, arg: u16) {
        self.volatile_memory[arg as usize] = self.pc as i16;
        self.pc = arg;
    }

    fn load(&mut self, arg: u16) {
        self.ac = self.volatile_memory[arg as usize] as i16;
    }

    fn store(&mut self, arg: u16) {
        self.volatile_memory[arg as usize] = self.ac;
    }

    fn add(&mut self, arg: u16) {
        self.ac += arg as i16;
    }

    fn sub(&mut self, arg: u16) {
        self.ac -= arg as i16;
    }

    fn input<R: Read>(&mut self, stream: &mut R) -> io::Result<()> {
        let mut buffer: [u8; 2] = [0; 2];
        if stream.read(&mut buffer)? == 2 {
            self.ac = (buffer[0] as i16) | (buffer[1] as i16);
        }
        Ok(())
    }

    fn output<W: Write>(&mut self, stream: &mut W) -> io::Result<()> {
        let ac = [(self.ac & 0xff) as u8, (self.ac.swap_bytes() & 0xff) as u8];
        stream.write_all(&ac)?;
        Ok(())
    }

    fn skipcond(&mut self, arg: u16) {
        match arg {
            0x0000 => {
                if self.ac < 0 {
                    self.pc += 1;
                }
            }
            0x0400 => {
                if self.ac == 0 {
                    self.pc += 1;
                }
            }
            0x0800 => {
                if self.ac > 0 {
                    self.pc += 1;
                }
            }
            _ => self.pc = self.pc,
        }
    }

    fn clear(&mut self) {
        self.ac = 0;
    }

    fn jump(&mut self, arg: u16) {
        self.pc = arg;
    }

    fn addi(&mut self, arg: u16) {
        self.ac += self.volatile_memory[arg as usize];
    }

    fn jumpi(&mut self, arg: u16) {
        self.pc = self.volatile_memory[arg as usize] as u16;
    }

    fn loadi(&mut self, arg: u16) {
        let indirect_pointer = self.volatile_memory[arg as usize] as usize;
        self.ac = self.volatile_memory[indirect_pointer];
    }

    fn storei(&mut self, arg: u16) {
        let indirect_pointer = self.volatile_memory[arg as usize] as usize;
        self.volatile_memory[indirect_pointer] = self.ac;
    }

    pub fn run<R: Read, W: Write>(mut self, program: Vec<u16>, rstream: &mut R, wstream: &mut W) {
        while self.pc < (program.len() as u16) {
            let instruction = program[self.pc as usize];
            let opcode = instruction & 0xf000;
            let argument = instruction & 0x0fff;
            match opcode {
                0x0000 => self.jns(argument),
                0x1000 => self.load(argument),
                0x2000 => self.store(argument),
                0x3000 => self.add(argument),
                0x4000 => self.sub(argument),
                0x5000 => match self.input(rstream) {
                    Ok(()) => (),
                    Err(error) => panic!("Problem opening stream in read mode: {:?}", error),
                },
                0x6000 => match self.output(wstream) {
                    Ok(()) => (),
                    Err(error) => panic!("Problem opening stream in write mode: {:?}", error),
                },
                0x7000 => break,
                0x8000 => self.skipcond(argument),
                0x9000 => self.clear(),
                0xa000 => self.jump(argument),
                0xb000 => self.addi(argument),
                0xc000 => self.jumpi(argument),
                0xd000 => self.loadi(argument),
                0xe000 => self.storei(argument),
                _ => panic!("Bad instruction!"),
            }
        }
    }
}
