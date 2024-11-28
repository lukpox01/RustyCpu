#![allow(dead_code)]

use std::process::exit;

const MEMORY_SIZE: usize = 256;

struct Cpu {
    registers: Registers,
    flags: Flags,
    memory: Memory,
    current_instruction: Option<Instruction>,
    running: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            registers: Registers::default(),
            flags: Flags::default(),
            memory: Memory::default(),
            current_instruction: None,
            running: true,
        }
    }
}

impl Cpu {

    fn debug(&self) {
        println!("- - - DEBUG - - -");
        println!("Registers: {:?}", self.registers);
        println!("Flags: {:?}", self.flags);
        println!("Memory: {:?}", self.memory.data);
        println!("Current instruction: {:?}", self.current_instruction);
        println!("- - - - - - - - -");
    }

    fn fetch(&mut self) -> u8 {
        let byte = self.memory.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    fn fetch_instruction(&mut self) -> Result<Instruction, &'static str> {

        if self.registers.pc >= self.memory.len() as u16 {
            return Err("Program counter out of bounds");
        }

        let opcode_bin = self.fetch();

        if self.registers.pc >= self.memory.len() as u16 {
            println!("{}, {}", self.registers.pc, self.memory.len());
            return Err("Incomplete instruction: missing operand byte");
        }

        let operands_bin = self.fetch();

        let mode = (operands_bin >> 6) & 0b11;
        let reg1 = (operands_bin >> 3) & 0b111;
        let reg2 = operands_bin & 0b111;

        if reg1 >= self.registers.len() as u8 || reg2 >= self.registers.len() as u8 {
            return Err("Invalid register number");
        }


        let addressing_mode = match mode {
            0 => AddressingMode::Immediate,
            1 => AddressingMode::Register,
            2 => AddressingMode::Indirect,
            3 => AddressingMode::Memory,
            _ => panic!("Invalid addressing mode")
        }; 

        let data = match addressing_mode {
            AddressingMode::Register | AddressingMode::Indirect => None,
            AddressingMode::Immediate | AddressingMode::Memory => {
                if self.registers.pc >= self.memory.len() as u16 {
                    return Err("Incomplete instruction: missing data byte");
                }
                Some(self.fetch())
            }
        };

        let opcode = match Opcode::from_byte(opcode_bin) {
            Some(opcode) => opcode,
            None => return Err("Invalid opcode")
        };

        Ok(Instruction {
            opcode: opcode,
            mode: addressing_mode,
            reg1: reg1,
            reg2: reg2,
            data: data,
        })
    }

    fn execute(&mut self, instruction: Instruction) {
        println!("{:?}", instruction);
        self.current_instruction = Some(instruction.clone());
        match instruction.opcode {
            Opcode::LOAD => self.load_imediate(instruction),
            Opcode::STORE => self.store_imediate(instruction),
            Opcode::MOV => self.mov_imediate(instruction),
            Opcode::SWAP => self.swap_register(instruction),
            Opcode::ADD => self.add_imediate(instruction),
            Opcode::SUB => self.sub_imediate(instruction),
            Opcode::MUL => self.mul_imediate(instruction),
            Opcode::DIV => self.div_imediate(instruction),
            Opcode::INC => self.inc_register(instruction),
            Opcode::DEC => self.dec_register(instruction),
            Opcode::AND => self.and_imediate(instruction),
            Opcode::OR => self.or_imediate(instruction),
            Opcode::XOR => self.xor_imediate(instruction),
            Opcode::NOT => self.not_register(instruction),
            Opcode::SHL => self.shl_imediate(instruction),
            Opcode::SHR => self.shr_imediate(instruction),
            Opcode::JMP => self.jmp_imediate(instruction),
            Opcode::JZ => self.jz_imediate(instruction),
            Opcode::JNZ => self.jnz_imediate(instruction),
            Opcode::JC => self.jc_imediate(instruction),
            Opcode::CALL => self.call_imediate(instruction),
            Opcode::RET => self.ret(instruction), // TODO: implement it so it wont take another byte as register
            Opcode::PUSH => self.push_imediate(instruction),
            Opcode::POP => self.pop_register(instruction),
            Opcode::NOP => self.nop(instruction), // TODO: implement it so it wont take another byte as register
            Opcode::HALT => self.halt(),
            _ => panic!("Invalid opcode")
        }
    }

    fn run(&mut self) {
        while self.running {
            match self.fetch_instruction() {
                Ok(instruction) => {
                    self.execute(instruction);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    fn call_imediate(&mut self, instruction: Instruction) {
        let address = match instruction.data {
            Some(data) => data,
            None => return
        };

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write(self.registers.sp, (self.registers.pc >> 8) as u8);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write(self.registers.sp, self.registers.pc as u8);
        self.registers.pc = address as u16;
    }

    fn nop(&mut self, instruction: Instruction) {
        // Do nothing
    }

    fn pop_register(&mut self, instruction: Instruction) {
        let reg1 = match instruction.reg1 {
            0 => &mut self.registers.r0,
            1 => &mut self.registers.r1,
            2 => &mut self.registers.r2,
            3 => &mut self.registers.r3,
            4 => &mut self.registers.r4,
            5 => &mut self.registers.r6,
            7 => &mut self.registers.r7,
            _ => panic!("Invalid register number")
        };

        *reg1 = self.memory.read(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    fn push_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory.write(self.registers.sp, data);
    }

    fn ret(&mut self, _instruction: Instruction) {
        let low = self.memory.read(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
        let high = self.memory.read(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.registers.pc = (high << 8) | low;
    }

    fn jc_imediate(&mut self, instruction: Instruction) {
        let address = match instruction.data {
            Some(data) => data,
            None => return
        };

        if self.flags.carry {
            self.registers.pc = address as u16;
        }
    }

    fn jnz_imediate(&mut self, instruction: Instruction) {
        let address = match instruction.data {
            Some(data) => data,
            None => return
        };

        if !self.flags.zero {
            self.registers.pc = address as u16;
        }
    }

    fn jz_imediate(&mut self, instruction: Instruction) {
        let address = match instruction.data {
            Some(data) => data,
            None => return
        };

        if self.flags.zero {
            self.registers.pc = address as u16;
        }
    }

    fn jmp_imediate(&mut self, instruction: Instruction) {
        let address = match instruction.data {
            Some(data) => data,
            None => return
        };

        self.registers.pc = address as u16;
    }

    fn shl_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1 << data;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn shr_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1 >> data;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn not_register(&mut self, instruction: Instruction) {
        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = !reg1;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn xor_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1 ^ data as u16;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn and_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1 & data as u16;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn or_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1 | data as u16;

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
    }

    fn store_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let address = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        self.memory.write(address, data);
    }

    fn load_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        match instruction.reg1 {
            0 => self.registers.r0 = data as u16,
            1 => self.registers.r1 = data as u16,
            2 => self.registers.r2 = data as u16,
            3 => self.registers.r3 = data as u16,
            4 => self.registers.r4 = data as u16,
            5 => self.registers.r6 = data as u16,
            7 => self.registers.r7 = data as u16,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = data == 0;
    }

    fn mov_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        match instruction.reg1 {
            0 => self.registers.r0 = data as u16,
            1 => self.registers.r1 = data as u16,
            2 => self.registers.r2 = data as u16,
            3 => self.registers.r3 = data as u16,
            4 => self.registers.r4 = data as u16,
            5 => self.registers.r6 = data as u16,
            7 => self.registers.r7 = data as u16,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = data == 0;

    }

    fn add_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_add(data as u16);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result < reg1;
        self.flags.overflow = result < reg1;
    }

    fn swap_register(&mut self, instruction: Instruction) {
        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let reg2 = match instruction.reg2 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        match instruction.reg1 {
            0 => self.registers.r0 = reg2,
            1 => self.registers.r1 = reg2,
            2 => self.registers.r2 = reg2,
            3 => self.registers.r3 = reg2,
            4 => self.registers.r4 = reg2,
            5 => self.registers.r6 = reg2,
            7 => self.registers.r7 = reg2,
            _ => panic!("Invalid register number")
        };

        match instruction.reg2 {
            0 => self.registers.r0 = reg1,
            1 => self.registers.r1 = reg1,
            2 => self.registers.r2 = reg1,
            3 => self.registers.r3 = reg1,
            4 => self.registers.r4 = reg1,
            5 => self.registers.r6 = reg1,
            7 => self.registers.r7 = reg1,
            _ => panic!("Invalid register number")
        };
    }

    fn sub_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_sub(data as u16);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result > reg1;
        self.flags.overflow = result > reg1;
    }   

    fn mul_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_mul(data as u16);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result > reg1;
        self.flags.overflow = result > reg1;
    }

    fn div_imediate(&mut self, instruction: Instruction) {
        let data = match instruction.data {
            Some(data) => data,
            None => return
        };

        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_div(data as u16);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result > reg1;
        self.flags.overflow = result > reg1;
    }

    fn inc_register(&mut self, instruction: Instruction) {
        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_add(1);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result < reg1;
        self.flags.overflow = result < reg1;
    }

    fn dec_register(&mut self, instruction: Instruction) {
        let reg1 = match instruction.reg1 {
            0 => self.registers.r0,
            1 => self.registers.r1,
            2 => self.registers.r2,
            3 => self.registers.r3,
            4 => self.registers.r4,
            5 => self.registers.r6,
            7 => self.registers.r7,
            _ => panic!("Invalid register number")
        };

        let result = reg1.wrapping_sub(1);

        match instruction.reg1 {
            0 => self.registers.r0 = result,
            1 => self.registers.r1 = result,
            2 => self.registers.r2 = result,
            3 => self.registers.r3 = result,
            4 => self.registers.r4 = result,
            5 => self.registers.r6 = result,
            7 => self.registers.r7 = result,
            _ => panic!("Invalid register number")
        };

        self.flags.zero = result == 0;
        self.flags.negative = result > 0x7FFF;
        self.flags.carry = result < reg1;
        self.flags.overflow = result < reg1;
    }

    fn halt(&mut self) {
        self.running = false;
    }
}

struct Memory {
    data: [u8; MEMORY_SIZE]
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: [0; MEMORY_SIZE]
        }
    }
}

impl Memory {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    mode: AddressingMode,
    reg1: u8,
    reg2: u8,
    data: Option<u8>,  // For immediate values or addresses
}

#[derive(Debug, Clone)]
enum AddressingMode {
    Register,
    Immediate,
    Indirect,
    Memory
}

impl AddressingMode {
    fn from_byte(byte: u8) -> Option<AddressingMode> {
        match byte {
            1 => Some(AddressingMode::Register),
            0 => Some(AddressingMode::Immediate),
            2 => Some(AddressingMode::Indirect),
            3 => Some(AddressingMode::Memory),
            _ => None
        }
    }
    
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,

    pc: u16,
    sp: u16,
    bp: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
            sp: MEMORY_SIZE as u16, // Initialize stack pointer to the end of memory
            bp: 0,
        }
    }
}

impl Registers {
    fn len(&self) -> usize {
        7
    }
    
}

#[derive(Debug, Clone, Copy)]
struct Flags {
    zero: bool,
    negative: bool,
    carry: bool,
    overflow: bool,
    interrupt: bool
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            zero: false,
            negative: false,
            carry: false,
            overflow: false,
            interrupt: false
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    // Data Movement (0000)
    LOAD = 0x00,    // 0000 0000
    STORE = 0x01,   // 0000 0001
    MOV = 0x02,     // 0000 0010
    SWAP = 0x03,    // 0000 0011

    // Arithmetic (0001)
    ADD = 0x10,     // 0001 0000
    SUB = 0x11,     // 0001 0001
    MUL = 0x12,     // 0001 0010
    DIV = 0x13,     // 0001 0011
    INC = 0x14,     // 0001 0100
    DEC = 0x15,     // 0001 0101

    // Logic (0010)
    AND = 0x20,     // 0010 0000
    OR = 0x21,      // 0010 0001
    XOR = 0x22,     // 0010 0010
    NOT = 0x23,     // 0010 0011
    SHL = 0x24,     // 0010 0100
    SHR = 0x25,     // 0010 0101

    // Control Flow (0011)
    JMP = 0x30,     // 0011 0000
    JZ = 0x31,      // 0011 0001
    JNZ = 0x32,     // 0011 0010
    JC = 0x33,      // 0011 0011
    CALL = 0x34,    // 0011 0100
    RET = 0x35,     // 0011 0101

    // Stack (0100)
    PUSH = 0x40,    // 0100 0000
    POP = 0x41,     // 0100 0001

    // System (0111)
    NOP = 0x70,     // 0111 0000
    HALT = 0x7F,    // 0111 1111
}

impl Opcode {
    fn from_byte(byte: u8) -> Option<Opcode> {
        match byte {
            0x00 => Some(Opcode::LOAD),
            0x01 => Some(Opcode::STORE),
            0x02 => Some(Opcode::MOV),
            0x03 => Some(Opcode::SWAP),
            0x10 => Some(Opcode::ADD),
            0x11 => Some(Opcode::SUB),
            0x12 => Some(Opcode::MUL),
            0x13 => Some(Opcode::DIV),
            0x14 => Some(Opcode::INC),
            0x15 => Some(Opcode::DEC),
            0x20 => Some(Opcode::AND),
            0x21 => Some(Opcode::OR),
            0x22 => Some(Opcode::XOR),
            0x23 => Some(Opcode::NOT),
            0x24 => Some(Opcode::SHL),
            0x25 => Some(Opcode::SHR),
            0x30 => Some(Opcode::JMP),
            0x31 => Some(Opcode::JZ),
            0x32 => Some(Opcode::JNZ),
            0x33 => Some(Opcode::JC),
            0x34 => Some(Opcode::CALL),
            0x35 => Some(Opcode::RET),
            0x40 => Some(Opcode::PUSH),
            0x41 => Some(Opcode::POP),
            0x70 => Some(Opcode::NOP),
            0x7F => Some(Opcode::HALT),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests;