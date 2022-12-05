# simplified-8-bit-cpu-emulator

This started up as just an experiment with structs and creating my own types.

It has since then grown into a very very basic 8-bit CPU emulator with very basic instruction.

All CPU commands are currently called from the main.rs file

Functions handling CPU operations are supported by passing the args ex.( cpu: &mut CPU, regs: [usize; 5] )

another example: fn func_name(cpu: &mut CPU, regs: [usize; 5]) { cpu.add_reg_reg(regs[0], regs[1]) }


# Instructions
cpu.update_reg_data(reg, u8) -> replaces data in reg with new data

cpu.inc_reg(reg) -> increases data in reg by 1

cpu.store_reg(reg) -> store reg data in memory (reg(BP) holds address of memory)

cpu.load_data(reg) -> loads memory into a register

cpu.add_reg_reg(reg, reg) -> adds two registers together (result stored in the A register)

cpu.sub_reg_reg(reg, reg) -> subtracts two registers (result stored in the A register)

cpu.or_reg_reg(reg, reg) -> or's two registers (result stored in the A register)

cpu.xor_reg_reg(reg, reg) -> xor's two registers (result stored in the A register)

cpu.not_reg(reg) -> basically inverts the bits in a register

cpu.shl_reg(reg) -> shift bits in reg to the left by 1

cpu.shr_reg(reg) -> shift bits in reg to the right by 1

cpu.reset_all_data() -> sets data of registers and memory to 0

# Additional info

This CPU has 4 8-bit registers

256 bytes of memory or ram

# Planned features

16-bit memory addressing

Kernel interupts

A functional GUI

Multiple displays containing memory map and registers

File parsing for execution from a text file or other programming language file

# Getting started


1. First create a variable: "let cpu: CPU = CPU::new();"

2a. You can make a list of reg ID's: "let a: usize = 0;"

2b. The value is the index of the register

2c. A list of ID's can be made by: "let regs: [usize; 5] = [a, b, c, d, bp];"

3a. You can now begin calling CPU instructions: cpu.example_function(reg, reg)

3b. All data related to the CPU struct can be printed in a println!() macro using the {:?} format

3c. example: println!("{:?}", cpu.memory.memory[15]);

3d. This would print the memory value at address 15
