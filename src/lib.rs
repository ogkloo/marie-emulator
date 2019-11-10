/// MARIE has a 12 bit address space, which is weird, but so goes. The reasoning for this is so
/// that full instructions can fit in a 16 bit space.
/// The ROM for this hypothetical machine is defined as a 1KB space for a "kernel" to fit in. Use
/// this memory carefully as it's very limited. It's defined as 16 bit because the instructions are
/// 16 bits long and this is done for convenience.
/// Debug is not derived on account of dumping the entire memory is a really bad idea.
pub struct Marie {
    // 12 bit address space, so 2^12 = 4096 bytes.
    volatile_memory: [u16; 4096],
    // The 16 bit accumulator and program counter.
    ac: u16,
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

impl Marie {
    fn jns(&mut self, arg: u16) {
        if arg < 4096 {
            self.volatile_memory[arg as usize] = self.pc;
        }
        self.pc = arg;
    }

    fn load(&mut self, arg: u16) {
        if arg < 4096 {
            self.ac = self.volatile_memory[arg as usize];
        }
    }

    fn store(&mut self, arg: u16) {
        if arg < 4096 {
            self.volatile_memory[arg as usize] = self.ac;
        }
    }

    fn add(&mut self, arg: u16) {
        if arg < 4096 {
            self.ac += self.volatile_memory[arg as usize];
        }
    }

    fn sub(&mut self, arg: u16) {
        if arg < 4096 {
            self.ac -= self.volatile_memory[arg as usize];
        }
    }

    pub fn tests(mut self) {
        // Test setup
        self.volatile_memory[0] = 2;
        self.volatile_memory[1] = 125;
        self.volatile_memory[2] = 125;
        self.volatile_memory[4] = 5;

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

        // Test ADD
        self.add(3);
        println!(
            "Added {} to produce {} in the AC",
            self.volatile_memory[3], self.ac
        );

        // Test SUB
        self.sub(4);
        println!(
            "Subbed {} to produce {} in the AC",
            self.volatile_memory[3], self.ac
        );
    }
}
