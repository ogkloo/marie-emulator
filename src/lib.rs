/// Encoding of the MARIE instruction set.
/// Each of these is technically 4  **bits ** long, and this should be respected in an assembler
/// for the language, however for the purposes of readability they'll be encoded this way.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    JNS,
    LOAD,
    STORE,
    ADD,
    SUB,
    INPUT,
    OUTPUT,
    HALT,
    SKIPCOND,
    JUMP,
    CLEAR,
    ADDI,
    SUBT,
    JUMPI,
    DEC,
    HEX,
    NOP,
}

/// MARIE has a 12 bit address space, which is weird, but so goes. The reasoning for this is so
/// that full instructions can fit in a 16 bit space.
/// The ROM for this hypothetical machine is defined as a 1KB space for a "kernel" to fit in. Use
/// this memory carefully as it's very limited. It's defined as 16 bit because the instructions are
/// 16 bits long and this is done for convenience.
/// Debug is not derived on account of dumping the entire memory is a really bad idea.
pub struct Marie {
    // 12 bit address space, so 2^12 = 4096 bytes.
    pub volatile_memory: [u8; 4096],
    // Arbitrary, not listed in the MARIE specification.
    pub rom: [(Instruction, u16); 512],
    // The 16 bit accumulator and program counter.
    ac: u8,
    pc: u16,
}

impl Default for Marie {
    fn default() -> Self {
        Marie {
            volatile_memory: [0; 4096],
            rom: [(Instruction::NOP, 0); 512],
            ac: 0,
            pc: 0,
        }
    }
}

impl Marie {
    fn jns(&mut self, arg: u16) {
        if arg < 4096 {
            self.volatile_memory[arg as usize] = (self.pc & 0xff) as u8;
            self.volatile_memory[arg as usize] = (self.pc.swap_bytes() & 0xff) as u8;
        }
        self.pc = arg;
    }

    fn load(&mut self, arg: u16) {
        if arg < 4096 {
            self.ac = self.volatile_memory[arg as usize];
        }
    }

    fn store(&mut self, arg: u16) {
        if arg < 4096 && arg + 1 < 4096 {
            self.volatile_memory[arg as usize] = self.ac;
        }
    }

    pub fn tests(mut self) {
        // Test setup
        self.volatile_memory[0] = 2;
        self.volatile_memory[1] = 255;
        self.volatile_memory[2] = 255;

        // Test LOAD
        self.load(1);
        println!("Loaded {}", self.ac);

        // Test JNS
        self.jns(0);
        println!(
            "Address stored in: {}, address: {}",
            self.pc, self.volatile_memory[2]
        );

        // Test STORE
        self.store(3);
        println!(
            "Stored {} at address 3: {}",
            self.ac, self.volatile_memory[3]
        );
    }
}
