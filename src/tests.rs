#[cfg(test)]
mod tests {
    use crate::Cpu;

    #[test]
    fn test_load_immediate() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0000_0000; // load r0, 1
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0001;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 1);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_store_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 10;
        cpu.memory.data[0] = 0b0000_0001; // store r0, 10
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_1010;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.memory.read(10), 10);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_mov_immediate() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0000_0010; // mov r1, 3
        cpu.memory.data[1] = 0b0000_1000;
        cpu.memory.data[2] = 0b0000_0011;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r1, 3);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_swap_register() {
        let mut cpu = Cpu::default();
        cpu.registers.r1 = 5;
        cpu.registers.r2 = 10;
        cpu.memory.data[0] = 0b0000_0011; // swap r1, r2
        cpu.memory.data[1] = 0b0100_1010;
        cpu.memory.data[2] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r1, 10);
        assert_eq!(cpu.registers.r2, 5);
        assert_eq!(cpu.registers.pc, 3 + 2); // Adjust for HALT
    }

    #[test]
    fn test_add_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 5;
        cpu.memory.data[0] = 0b0001_0000; // add r0, 3
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0011;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 8);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_sub_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 5;
        cpu.memory.data[0] = 0b0001_0001; // sub r0, 3
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0011;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 2);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_mul_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 5;
        cpu.memory.data[0] = 0b0001_0010; // mul r0, 3
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0011;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 15);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_div_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 6;
        cpu.memory.data[0] = 0b0001_0011; // div r0, 3
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0011;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 2);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_inc_register() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 5;
        cpu.memory.data[0] = 0b0001_0100; // inc r0
        cpu.memory.data[1] = 0b0100_0000;
        cpu.memory.data[2] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 6);
        assert_eq!(cpu.registers.pc, 3 + 2); // Adjust for HALT
    }

    #[test]
    fn test_dec_register() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 5;
        cpu.memory.data[0] = 0b0001_0101; // dec r0
        cpu.memory.data[1] = 0b0100_0000;
        cpu.memory.data[2] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 4);
        assert_eq!(cpu.registers.pc, 3 + 2); // Adjust for HALT
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b1010_1010;
        cpu.memory.data[0] = 0b0010_0000; // and r0, 0b1100_1100
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b1100_1100;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 0b1000_1000);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_or_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b1010_1010;
        cpu.memory.data[0] = 0b0010_0001; // or r0, 0b1100_1100
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b1100_1100;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 0b1110_1110);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_xor_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b1010_1010;
        cpu.memory.data[0] = 0b0010_0010; // xor r0, 0b1100_1100
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b1100_1100;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 0b0110_0110);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_not_register() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b1010_1010;
        cpu.memory.data[0] = 0b0010_0011; // not r0
        cpu.memory.data[1] = 0b0100_0000;
        cpu.memory.data[2] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, !0b1010_1010);
        assert_eq!(cpu.registers.pc, 3 + 2); // Adjust for HALT
    }

    #[test]
    fn test_shl_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b0001_0000;
        cpu.memory.data[0] = 0b0010_0100; // shl r0, 2
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0010;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 0b0100_0000);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_shr_immediate() {
        let mut cpu = Cpu::default();
        cpu.registers.r0 = 0b0100_0000;
        cpu.memory.data[0] = 0b0010_0101; // shr r0, 2
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0010;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 0b0001_0000);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_jmp_immediate() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0011_0000; // jmp 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
    }

    #[test]
    fn test_jz_immediate() {
        let mut cpu = Cpu::default();
        cpu.flags.zero = true;
        cpu.memory.data[0] = 0b0011_0001; // jz 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
    }

    #[test]
    fn test_jnz_immediate() {
        let mut cpu = Cpu::default();
        cpu.flags.zero = false;
        cpu.memory.data[0] = 0b0011_0010; // jnz 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
    }

    #[test]
    fn test_jc_immediate() {
        let mut cpu = Cpu::default();
        cpu.flags.carry = true;
        cpu.memory.data[0] = 0b0011_0011; // jc 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
    }

    #[test]
    fn test_call_immediate() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0011_0100; // call 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        cpu.debug();
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
        assert_eq!(cpu.memory.read(cpu.registers.sp), 3);
        assert_eq!(cpu.memory.read(cpu.registers.sp.wrapping_add(1)), 0);

        
    }

    #[test]
    fn test_ret() {
        // TODO not working
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0011_0100; // call 5
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_0101;
        cpu.memory.data[5] = 0b0011_0101; // ret
        cpu.memory.data[7] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 5 + 2+1); // Adjust for HALT
    }

    #[test]
    fn test_push_immediate() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0100_0000; // push 10
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_1010;
        cpu.memory.data[3] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.memory.read(cpu.registers.sp), 10);
        assert_eq!(cpu.registers.pc, 4 + 2); // Adjust for HALT
    }

    #[test]
    fn test_pop_register() {
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0100_0000; // push 10
        cpu.memory.data[1] = 0b0000_0000;
        cpu.memory.data[2] = 0b0000_1010;
        cpu.memory.data[3] = 0b0100_0001; // pop r0
        cpu.memory.data[4] = 0b0100_0000;
        cpu.memory.data[5] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.r0, 10);
        assert_eq!(cpu.registers.pc, 6 + 2); // Adjust for HALT
    }

    #[test]
    fn test_nop() {
        // TODO not working
        let mut cpu = Cpu::default();
        cpu.memory.data[0] = 0b0111_0000; // nop
        cpu.memory.data[1] = 0b0100_0000; 
        cpu.memory.data[2] = 0b0111_1111; // halt
        cpu.run();
        assert_eq!(cpu.registers.pc, 2+2+1); // Adjust for HALT
    }
}