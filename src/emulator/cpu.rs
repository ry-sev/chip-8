use super::{
	Instruction, DISPLAY_HEIGHT, DISPLAY_WIDTH, FONTS, FONTS_SIZE, NUM_KEYS,
	NUM_REGS, RAM_SIZE, STACK_SIZE, START_ADDRESS,
};
use anyhow::{anyhow, Error};
use rand::Rng;

macro_rules! invalid_instruction {
	($x:expr) => {
		Err(anyhow!(format!("Could not decode instruction {:#6x}", $x)))
	};
}

#[derive(Debug)]
pub struct Cpu {
	i: u16,
	pc: u16,
	sp: u8,
	reg: Vec<u8>,
	pub memory: Vec<u8>,
	stack: Vec<u16>,
	pub display: Vec<bool>,
	pub delay_timer: u8,
	pub sound_timer: u8,
	pub keys: Vec<bool>,
}

impl Cpu {
	pub fn new() -> Self {
		let mut cpu = Self {
			i: 0,
			pc: START_ADDRESS,
			sp: 0,
			reg: vec![0; NUM_REGS],
			memory: vec![0; RAM_SIZE],
			stack: vec![0; STACK_SIZE],
			display: vec![false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
			delay_timer: 0,
			sound_timer: 0,
			keys: vec![false; NUM_KEYS],
		};
		cpu.memory[..FONTS_SIZE].copy_from_slice(&FONTS);
		cpu
	}

	pub fn reset(&mut self) {
		self.i = 0;
		self.pc = START_ADDRESS;
		self.sp = 0;
		self.reg = vec![0; NUM_REGS];
		self.memory[..FONTS_SIZE].copy_from_slice(&FONTS);
		self.stack = vec![0; STACK_SIZE];
		self.display = vec![false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
		self.delay_timer = 0;
		self.sound_timer = 0;
		self.keys = vec![false; NUM_KEYS];
	}

	pub fn step(&mut self) -> Result<(), Error> {
		let opcode = self.fetch();
		let instruction = self.decode(opcode)?;
		self.execute(instruction)
	}

	fn fetch(&mut self) -> u16 {
		let high_byte = (self.memory[self.pc as usize]) as u16;
		let low_byte = (self.memory[(self.pc + 1) as usize]) as u16;
		self.pc += 2;
		(high_byte << 8) | low_byte
	}

	fn decode(&mut self, opcode: u16) -> Result<Instruction, Error> {
		let bit_1 = ((opcode & 0xF000) >> 12) as u8;
		let bit_2 = ((opcode & 0x0F00) >> 8) as u8;
		let bit_3 = ((opcode & 0x00F0) >> 4) as u8;
		let bit_4 = (opcode & 0x000F) as u8;
		let low_byte = (opcode & 0xFF) as u8;

		let instruction = match bit_1 {
			0x0 => match low_byte {
				0xE0 => Instruction::_00E0,
				0xEE => Instruction::_00EE,
				_ => Instruction::_0NNN(opcode & 0xFFF),
			},
			0x1 => Instruction::_1NNN(opcode & 0xFFF),
			0x2 => Instruction::_2NNN(opcode & 0xFFF),
			0x3 => Instruction::_3XNN(bit_2, low_byte),
			0x4 => Instruction::_4XNN(bit_2, low_byte),
			0x5 => Instruction::_5XY0(bit_2, bit_3),
			0x6 => Instruction::_6XNN(bit_2, low_byte),
			0x7 => Instruction::_7XNN(bit_2, low_byte),
			0x8 => match bit_4 {
				0x0 => Instruction::_8XY0(bit_2, bit_3),
				0x1 => Instruction::_8XY1(bit_2, bit_3),
				0x2 => Instruction::_8XY2(bit_2, bit_3),
				0x3 => Instruction::_8XY3(bit_2, bit_3),
				0x4 => Instruction::_8XY4(bit_2, bit_3),
				0x5 => Instruction::_8XY5(bit_2, bit_3),
				0x6 => Instruction::_8XY6(bit_2),
				0x7 => Instruction::_8XY7(bit_2, bit_3),
				0xE => Instruction::_8XYE(bit_2),
				_ => return invalid_instruction!(opcode),
			},
			0x9 => Instruction::_9XY0(bit_2, bit_3),
			0xA => Instruction::ANNN(opcode & 0xFFF),
			0xB => Instruction::BNNN(opcode & 0xFFF),
			0xC => Instruction::CXNN(bit_2, low_byte),
			0xD => Instruction::DXYN(bit_2, bit_3, bit_4),
			0xE => match low_byte {
				0x9E => Instruction::EX9E(bit_2),
				0xA1 => Instruction::EXA1(bit_2),
				_ => return invalid_instruction!(opcode),
			},
			0xF => match low_byte {
				0x07 => Instruction::FX07(bit_2),
				0x0A => Instruction::FX0A(bit_2),
				0x15 => Instruction::FX15(bit_2),
				0x18 => Instruction::FX18(bit_2),
				0x1E => Instruction::FX1E(bit_2),
				0x29 => Instruction::FX29(bit_2),
				0x33 => Instruction::FX33(bit_2),
				0x55 => Instruction::FX55(bit_2),
				0x65 => Instruction::FX65(bit_2),
				_ => return invalid_instruction!(opcode),
			},
			_ => return invalid_instruction!(opcode),
		};
		Ok(instruction)
	}

	fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
		match instruction {
			Instruction::_0NNN(_address) => {}
			Instruction::_00E0 => {
				self.display = vec![false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
			}
			Instruction::_00EE => self.pc = self.pop(),
			Instruction::_1NNN(address) => self.pc = address,
			Instruction::_2NNN(address) => {
				self.push(self.pc);
				self.pc = address;
			}
			Instruction::_3XNN(vx, byte) => {
				if self.reg[vx as usize] == byte {
					self.pc += 2;
				}
			}
			Instruction::_4XNN(vx, byte) => {
				if self.reg[vx as usize] != byte {
					self.pc += 2;
				}
			}
			Instruction::_5XY0(vx, vy) => {
				if self.reg[vx as usize] == self.reg[vy as usize] {
					self.pc += 2;
				}
			}
			Instruction::_6XNN(vx, byte) => self.reg[vx as usize] = byte,
			Instruction::_7XNN(vx, byte) => {
				self.reg[vx as usize] = self.reg[vx as usize].wrapping_add(byte)
			}
			Instruction::_8XY0(vx, vy) => {
				self.reg[vx as usize] = self.reg[vy as usize]
			}
			Instruction::_8XY1(vx, vy) => {
				self.reg[vx as usize] |= self.reg[vy as usize]
			}
			Instruction::_8XY2(vx, vy) => {
				self.reg[vx as usize] &= self.reg[vy as usize]
			}
			Instruction::_8XY3(vx, vy) => {
				self.reg[vx as usize] ^= self.reg[vy as usize]
			}
			Instruction::_8XY4(vx, vy) => {
				let (result, overflow) =
					self.reg[vx as usize].overflowing_add(self.reg[vy as usize]);
				self.reg[vx as usize] = result;
				self.reg[0xF] = if overflow { 1 } else { 0 };
			}
			Instruction::_8XY5(vx, vy) => {
				let (result, borrow) =
					self.reg[vx as usize].overflowing_sub(self.reg[vy as usize]);
				self.reg[vx as usize] = result;
				self.reg[0xF] = if borrow { 0 } else { 1 };
			}
			Instruction::_8XY6(vx) => {
				let lsb = self.reg[vx as usize] & 1;
				self.reg[vx as usize] >>= 1;
				self.reg[0xF] = lsb;
			}
			Instruction::_8XY7(vx, vy) => {
				let (result, borrow) =
					self.reg[vy as usize].overflowing_sub(self.reg[vx as usize]);
				self.reg[vx as usize] = result;
				self.reg[0xF] = if borrow { 0 } else { 1 };
			}
			Instruction::_8XYE(vx) => {
				let lsb = (self.reg[vx as usize] >> 7) & 1;
				self.reg[vx as usize] <<= 1;
				self.reg[0xF] = lsb;
			}
			Instruction::_9XY0(vx, vy) => {
				if self.reg[vx as usize] != self.reg[vy as usize] {
					self.pc += 2;
				}
			}
			Instruction::ANNN(address) => self.i = address,
			Instruction::BNNN(address) => self.pc = address + (self.reg[0] as u16),
			Instruction::CXNN(vx, byte) => {
				let random_number: u8 = rand::thread_rng().gen();
				self.reg[vx as usize] = random_number & byte;
			}
			Instruction::DXYN(vx, vy, n) => {
				let x_coordinate = self.reg[vx as usize] as u16;
				let y_coordinate = self.reg[vy as usize] as u16;

				let mut flipped = false;
				for y_line in 0..n {
					let address = self.i + y_line as u16;
					let pixels = self.memory[address as usize];
					for x_line in 0..8 {
						if pixels & (0x80 >> x_line) != 0 {
							let x = (x_coordinate + x_line) as usize % DISPLAY_WIDTH;
							let y = (y_coordinate + y_line as u16) as usize
								% DISPLAY_HEIGHT;

							let pixel_index = x + DISPLAY_WIDTH * y;
							flipped |= self.display[pixel_index];
							self.display[pixel_index] ^= true;
						}
					}
				}
				self.reg[0xF] = if flipped { 1 } else { 0 };
			}
			Instruction::EX9E(vx) => {
				let key = self.keys[self.reg[vx as usize] as usize];
				if key {
					self.pc += 2;
				}
			}
			Instruction::EXA1(vx) => {
				let vx = self.reg[vx as usize];
				let key = self.keys[vx as usize];
				if !key {
					self.pc += 2;
				}
			}
			Instruction::FX07(vx) => self.reg[vx as usize] = self.delay_timer,
			Instruction::FX0A(vx) => {
				let mut key_pressed = false;
				for i in 0..self.keys.len() {
					if self.keys[i] {
						self.reg[vx as usize] = i as u8;
						key_pressed = true;
						break;
					}
				}
				if !key_pressed {
					self.pc -= 2;
				}
			}
			Instruction::FX15(vx) => self.delay_timer = self.reg[vx as usize],
			Instruction::FX18(vx) => self.sound_timer = self.reg[vx as usize],
			Instruction::FX1E(vx) => self.i += (self.reg[vx as usize]) as u16,
			Instruction::FX29(vx) => {
				let character = self.reg[vx as usize] as u16;
				self.i = character * 5;
			}
			Instruction::FX33(vx) => {
				let vx = self.reg[vx as usize] as f32;
				let hundreds = (vx / 100.0).floor() as u8;
				let tens = ((vx / 10.0) % 10.0).floor() as u8;
				let ones = (vx % 10.0) as u8;

				self.memory[self.i as usize] = hundreds;
				self.memory[(self.i + 1) as usize] = tens;
				self.memory[(self.i + 2) as usize] = ones;
			}
			Instruction::FX55(vx) => {
				for index in 0..=vx as usize {
					self.memory[self.i as usize + index] = self.reg[index];
				}
			}
			Instruction::FX65(vx) => {
				for index in 0..=vx as usize {
					self.reg[index] = self.memory[self.i as usize + index];
				}
			}
		}
		Ok(())
	}

	fn push(&mut self, data: u16) {
		self.stack[self.sp as usize] = data;
		self.sp += 1;
	}

	fn pop(&mut self) -> u16 {
		self.sp -= 1;
		self.stack[self.sp as usize]
	}
}

impl Default for Cpu {
	fn default() -> Self {
		Self::new()
	}
}
