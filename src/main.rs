use simplified_8_bit_emulator::CPU;

fn print_cpu_stats(cpu: &CPU) {
	println!("\n");
	println!("reg: {:?} data: {:?}",cpu.registers.registers[0].name, cpu.registers.registers[0].data);
	println!("reg: {:?} data: {:?}",cpu.registers.registers[1].name, cpu.registers.registers[1].data);
	println!("reg: {:?} data: {:?}",cpu.registers.registers[2].name, cpu.registers.registers[2].data);
	println!("reg: {:?} data: {:?}",cpu.registers.registers[3].name, cpu.registers.registers[3].data);
	println!("reg: {:?} data: {:?}",cpu.registers.registers[4].name, cpu.registers.registers[4].data);
}

fn print_memory_data(cpu: &CPU) {
	println!("Mem data: \n{:?}", cpu.memory.memory);
}

fn psuedo_rng(cpu: &mut CPU, regs: [usize; 5]) {
	cpu.store_reg(regs[0]);
	cpu.shr_reg(regs[0]);
	cpu.mov_reg_reg(regs[0], regs[1]);
	cpu.load_data(regs[0]);
	cpu.inc_reg(regs[4]);
	cpu.xor_reg_reg(regs[0], regs[1]);
	cpu.store_reg(regs[0]);
	cpu.shl_reg(regs[0]);
	cpu.mov_reg_reg(regs[0], regs[1]);
	cpu.load_data(regs[0]);
	cpu.inc_reg(regs[4]);
	cpu.xor_reg_reg(regs[0], regs[1]);
	cpu.store_reg(regs[0]);
	cpu.shr_reg(regs[0]);
	cpu.shr_reg(regs[0]);
	cpu.mov_reg_reg(regs[0], regs[1]);
	cpu.load_data(regs[0]);
	cpu.inc_reg(regs[4]);
	cpu.xor_reg_reg(regs[0], regs[1]);
	cpu.store_reg(regs[0]);
}

fn main() {
	println!("Compiled seccessfully");
	println!("Running...\n");
	let mut cpu = CPU::new(); // Creates the CPU
	let a: usize = 0;              // Binding for register ID
	let b: usize = 1;              // Binding for register ID
	let c: usize = 2;              // Binding for register ID
	let d: usize = 3;              // Binding for register ID
	let bp: usize = 4;             // Binding for register ID
	let regs = [a, b, c, d, bp]; // this is used for passing register ID's to functions
	println!("Initializing CPU CORE");
	cpu.update_reg_data(a, 0); // this command will replace the registers current data with new data provided
	cpu.update_reg_data(b, 0);
	cpu.update_reg_data(c, 0);
	cpu.update_reg_data(d, 0);
	cpu.update_reg_data(bp, 0);
	println!("Loading memory");
	println!("Memory ready\nMemory size in bytes: 256\n");
	println!("CPU Running...\n");
	cpu.update_reg_data(a, 5);
	cpu.inc_reg(a);                         // this command will increment the registers data by 1
	cpu.store_reg(a);               // this command will store the registers data into memory
	cpu.inc_reg(bp);
	cpu.update_reg_data(b, 200);
	cpu.store_reg(b);
	cpu.load_data(c);                 // this command will load the data from memory into the register
	cpu.add_reg_reg(a, c);
	cpu.inc_reg(bp);
	cpu.store_reg(a);
	cpu.reset_all_data();                   // this command will reset all registers and memory data to 0
	println!("\n");
	cpu.update_reg_data(a, 29);
	for _i in 0..=10 {
		psuedo_rng(&mut cpu, regs);          // example of a function with a mutable reference to the CPU and an array of register ID's
	}
	print_cpu_stats(&cpu);                   // this function will print current status of the CPU's registers
	print_memory_data(&cpu);                 // this function will print current status of the CPU's memory

	cpu.reset_all_data();
	cpu.update_reg_data(a, 5);
	cpu.update_reg_data(b, 10);
	cpu.sub_reg_reg(a, b);
	cpu.update_reg_data(a, 255);
	cpu.update_reg_data(b, 1);
	cpu.add_reg_reg(a, b);
	print_cpu_stats(&cpu);

	println!("\nProgram end\nShutting down virtualized CPU...\nGoodbye");
}