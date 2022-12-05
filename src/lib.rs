
#[derive(Debug)]
pub struct Mem {                  // this struct is used to store memory data in the form of ram
	pub memory: [u8; 256]
}
impl Mem {
	pub fn new() -> Mem {
		Mem {
			memory: [0; 256]
		}
	}
}

#[derive(Debug)]
pub struct Register {               // this struct is used to create a register for use in struct RegFile
	pub name: String,
	pub data: u8,
}
impl Register {
	pub fn new(name: String) -> Register {
		Register {
			name: name,
			data: 0,
		}
	}
}

#[derive(Debug)]
pub struct RegFile {                    // this struct creates an array of registers that the CPU can use
	pub registers: [Register; 5],
}
impl RegFile {
	pub fn new(a: Register, b: Register, c: Register, d: Register, bp: Register) -> RegFile {
		let regis = [a, b, c, d, bp];
		RegFile {
			registers: regis
		}
	}
}

#[derive(Debug)]
pub struct CPU {                        // this is the main struct of CPU. it has access to the other mentioned structs
	pub registers: RegFile,             // the CPU's new() function will automatically create necessary memory and register structs
	pub memory: Mem,
}
impl CPU {
	pub fn new() -> CPU {
		let a = Register::new("A".to_string());
		let b = Register::new("B".to_string());
		let c = Register::new("C".to_string());
		let d = Register::new("D".to_string());
		let bp = Register::new("BP".to_string());
		CPU {
			registers: RegFile::new(a, b, c, d, bp),
			memory: Mem::new(),
		}
	}
	// replace data in specified register with new data
	pub fn update_reg_data(&mut self, reg: usize, data: u8) {
		self.registers.registers[reg].data = data;
		println!("Pushing immediate value of {:?} to {:?}", data, self.registers.registers[reg].name);
	}
	// add two registers together and stores the result in the first register
	pub fn add_reg_reg(&mut self, reg1: usize, reg2: usize) {
		let val1 = self.registers.registers[reg1].data;
		let val2 = self.registers.registers[reg2].data;
		if val1.checked_add(val2) == None {
			println!("\nERROR: Addition Overflow error\n");
			println!("Overflow occurances not yet handled\n");
		} else {
			let result = val1 + val2;
			self.registers.registers[0].data = result;
			println!("{:?} added to {:?}", self.registers.registers[reg1].name, self.registers.registers[reg2].name);
			println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
		}
	}
	// subtracts two registers storing the result in the first register
	pub fn sub_reg_reg(&mut self, reg1: usize, reg2: usize) {
		let val1 = self.registers.registers[reg1].data;
		let val2 = self.registers.registers[reg2].data;
		if val1.checked_sub(val2) == None {
			println!("\nERROR: registers do not support negative values\n");
			println!("Overflow occurances not yet handled\n");
		} else {
			let result = val1 - val2;
			self.registers.registers[0].data = result;
			println!("{:?} subtracted from {:?}", self.registers.registers[reg1].name, self.registers.registers[reg2].name);
			println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
		}
	}
	// or's two registers storing the result in the first register
	pub fn or_reg_reg(&mut self, reg1: usize, reg2: usize) {
		let val1 = self.registers.registers[reg1].data;
		let val2 = self.registers.registers[reg2].data;
		let result = val1 | val2;
		self.registers.registers[0].data = result;
		println!("{:?} ORed with {:?}", self.registers.registers[reg1].name, self.registers.registers[reg2].name);
		println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
	}
	// and's two registers storing the result in the first register
	pub fn and_reg_reg(&mut self, reg1: usize, reg2: usize) {
		let val1 = self.registers.registers[reg1].data;
		let val2 = self.registers.registers[reg2].data;
		let result = val1 & val2;
		self.registers.registers[0].data = result;
		println!("{:?} ANDed with {:?}", self.registers.registers[reg1].name, self.registers.registers[reg2].name);
		println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
	}
	// xor's two registers storing the result in the first register
	pub fn xor_reg_reg(&mut self, reg1: usize, reg2: usize) {
		let val1 = self.registers.registers[reg1].data;
		let val2 = self.registers.registers[reg2].data;
		let result = val1 ^ val2;
		self.registers.registers[0].data = result;
		println!("{:?} XORed with {:?}", self.registers.registers[reg1].name, self.registers.registers[reg2].name);
		println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
	}
	// not's a register storing the result in the first register
	pub fn not_reg(&mut self, reg: usize) {
		let val = self.registers.registers[reg].data;
		let result = !val;
		self.registers.registers[0].data = result;
		println!("{:?} NOTed", self.registers.registers[reg].name);
		println!("{:?} now holds {:?}", self.registers.registers[0].name, result);
	}
	// shift the data bits of a register to the left
	pub fn shl_reg(&mut self, reg: usize) {
		let val = self.registers.registers[reg].data;
		let result = val << 1;
		self.registers.registers[reg].data = result;
		println!("{:?} shifted left", self.registers.registers[reg].name);
		println!("{:?} now holds {:?}", self.registers.registers[reg].name, result);
	}
	// shift the data bits of a register to the right
	pub fn shr_reg(&mut self, reg: usize) {
		let val = self.registers.registers[reg].data;
		let result = val >> 1;
		self.registers.registers[reg].data = result;
		println!("{:?} shifted right", self.registers.registers[reg].name);
		println!("{:?} now holds {:?}", self.registers.registers[reg].name, result);
	}
	// moves data from one register to another
	pub fn mov_reg_reg(&mut self, source: usize, dest: usize) {
		let data = self.registers.registers[source].data;
		self.registers.registers[dest].data = data;
		println!("Moved {:?} to {:?}", self.registers.registers[source].name, self.registers.registers[dest].name);
		println!("{:?} now holds {:?}", self.registers.registers[dest].name, data);
	}
	// stores the data of a register to memory located at "BP" registers data value
	pub fn store_reg(&mut self, source: usize) {
		let data = self.registers.registers[source].data;
		let dest = self.registers.registers[4].data as usize;
		self.memory.memory[dest] = data;
		println!("{:?} stored to memory at {:?}", self.registers.registers[source].name, dest);
	}
	// loads data from memory with address located in "BP" register into specified register
	pub fn load_data(&mut self, dest: usize) {
		let addr: usize = self.registers.registers[4].data.into();
		self.registers.registers[dest].data = self.memory.memory[addr];
		println!("{:?} is loading data from memory at {:?}", self.registers.registers[dest].name, addr);
		println!("{:?} now holds {:?}", self.registers.registers[dest].name, self.registers.registers[dest].data);
	}
	// increases the value of a register by 1
	pub fn inc_reg(&mut self, reg: usize) {
		if self.registers.registers[reg].data < 255 {
			self.registers.registers[reg].data += 1;
			let data = self.registers.registers[reg].data;
			println!("{:?} incremented to {:?}", self.registers.registers[reg].name, data);
		} else {
			println!("{:?} overflowed! setting to 0", self.registers.registers[reg].name);
			self.registers.registers[reg].data = 0;
		}
	}
	// resets all data in CPU to 0
	pub fn reset_all_data(&mut self) {
		println!("Resetting all data");
		for i in 0..5 {
			self.registers.registers[i].data = 0;
		}
		for i in 0..self.memory.memory.len() {
			self.memory.memory[i] = 0;
		}
		println!("All data reset");
	}
}