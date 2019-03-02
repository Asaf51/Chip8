use rand::Rng;

const RAM_SIZE: usize = 0xFFF;
const STACK_SIZE: usize = 16;
const START_OF_PROGRAM: usize = 0x200;

pub struct Cpu {
    memory: [u8; RAM_SIZE],
    v: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    stack: [u8; STACK_SIZE]
}

struct Opcode {
    opcode: u16
}

impl Opcode {
    fn new(opcode: u16) -> Opcode {
        Opcode {
            opcode
        }
    }

    fn execute(&self, cpu: &mut Cpu) {
        match self.opcode & 0xf000 {
            0x0000 => self.run_0x0xxx_opcode(cpu),
            0x1000 => cpu.pc = self.get_address(), // JP addr
            0x2000 => { // CALL addr
                cpu.stack[cpu.sp + 1] = cpu.pc as u8;
                cpu.sp += 1;
                cpu.pc = self.get_address()
            },
            0x3000 => {
                if cpu.v[self.get_nibble(1)] == self.get_lowest_byte() {
                    cpu.pc += 2;
                }
            },
            0x4000 => {
                if cpu.v[self.get_nibble(1)] != self.get_lowest_byte() {
                    cpu.pc += 2;
                }
            },
            0x5000 => {
                let (x, y) = self.get_xy_register_numbers();
                if cpu.v[x] == cpu.v[y] {
                    cpu.pc += 2;
                }
            },
            0x6000 => {
                let value = self.get_lowest_byte() as u8;
                let reg_number = self.get_nibble(1);

                cpu.v[reg_number] = value;
            },
            0x7000 => {
                let value = self.get_lowest_byte() as u8;
                let reg_number = self.get_nibble(1);

                cpu.v[reg_number] += value;
            },
            0x8000 => self.run_0x8xxx_opcode(cpu),
            0x9000 => {
                let (x, y) = self.get_xy_register_numbers();
                if cpu.v[x] != cpu.v[y] {
                    cpu.pc += 2;
                }
            },
            0xA000 => cpu.i = self.get_address(),
            0xB000 => cpu.pc = cpu.v[0] as usize + self.get_address(),
            0xC000 => {
                let random_number = rand::thread_rng().gen_range(0, 0xFF);
                cpu.v[self.get_nibble(1)] = self.get_lowest_byte() & random_number;
            },
            0xD000 => unimplemented!(),
            0xE000 => self.run_0xexxx_opcode(cpu),
            0xF000 => self.run_0xfxxx_opcode(cpu),
            _ => unimplemented!()
        };

    }

    fn run_0xfxxx_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xFF {
            0x07 => unimplemented!(), // LD Vx, DT
            0x0A => unimplemented!(), // LD Vx, K
            0x15 => unimplemented!(), // LD DT, Vx
            0x18 => unimplemented!(), // LD ST, Vx
            0x1E => {
                cpu.i = cpu.v[self.get_nibble(1)] as usize;
            },
            0x29 => unimplemented!(), // LD F, Vx,
            0x33 => {
                cpu.memory[cpu.i] = cpu.v[self.get_nibble(1)] / 100;
                cpu.memory[cpu.i + 1] = (cpu.v[self.get_nibble(1)] / 10) % 10;
                cpu.memory[cpu.i + 2] = (cpu.v[self.get_nibble(1)] % 100) % 10;
            },
            0x55 => {
                for reg_index in 0..self.get_nibble(1) {
                    cpu.memory[cpu.i + reg_index] = cpu.v[reg_index];
                }
            },
            0x65 => {
                for reg_index in 0..self.get_nibble(1) {
                    cpu.v[reg_index] = cpu.memory[cpu.i + reg_index];
                }
            },
            _ => unimplemented!()
        }
    }

    fn run_0xexxx_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xFF {
            0x9E => unimplemented!(), // SKP Vx
            0xA1 => unimplemented!(), // SKNP Vx
            _ => unimplemented!()
        }
    }

    fn get_lowest_byte(&self) -> u8{
        (self.opcode >> 8) as u8
    }

    fn get_nibble(&self, nibble: u8) -> usize {
        ((self.opcode >> (nibble * 4)) & 0xF) as usize
    }

    fn get_address(&self) -> usize {
        (self.opcode >> 4) as usize
    }

    fn get_xy_register_numbers(&self) -> (usize, usize) {
        (self.get_nibble(1), self.get_nibble(2))
    }

    fn run_0x0xxx_opcode(&self, cpu: &mut Cpu) {
        match self.opcode & 0xf {
            0x0 => unimplemented!(),
            0xe => {
                cpu.pc = cpu.memory[cpu.sp] as usize;
                cpu.sp -= 1;
            },
            _ => unimplemented!()
        }
    }

    fn run_0x8xxx_opcode(&self, cpu: &mut Cpu) {
        let (x, y) = self.get_xy_register_numbers();
        match self.opcode & 0xf {
            0x0 => cpu.v[x] = cpu.v[y],
            0x1 => cpu.v[x] |= cpu.v[y],
            0x2 => cpu.v[x] &= cpu.v[y],
            0x3 => cpu.v[x] ^= cpu.v[y],
            0x4 => {
                let res: u16 = (cpu.v[x] + cpu.v[y]).into();
                cpu.v[0xF] = if res > 0xFF { 1 } else { 0 };
                cpu.v[x] = (res & 0x00FF) as u8
            },
            0x5 => {
                cpu.v[0xF] = if cpu.v[x] > cpu.v[y] { 1 } else { 0 };
                cpu.v[x] -= cpu.v[y]
            },
            0x6 => {
                cpu.v[0xF] = cpu.v[x] & 1;
                cpu.v[x] >>= 1;
            },
            0x7 => {
                cpu.v[0xF] = if cpu.v[x] < cpu.v[y] { 1 } else { 0 };
                cpu.v[x] = cpu.v[y] - cpu.v[x];
            },
            0xE => {
                cpu.v[0xF] = cpu.v[x] >> 7;
                cpu.v[x] <<= 1;
            },
            _ => unimplemented!()
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; RAM_SIZE],
            v: [0; 16],
            i: 0,
            pc: START_OF_PROGRAM,
            sp: 0,
            stack: [0; STACK_SIZE],
        }
    }

    fn fetch_opcode(&mut self) -> Opcode {
        let opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        self.pc += 1;
        Opcode::new(opcode)
    }

    fn run_opcode(&mut self) {
        let opcode: Opcode = self.fetch_opcode();
        opcode.execute(self);
    }

    pub fn tick(&mut self) {
        self.run_opcode();

    }
}