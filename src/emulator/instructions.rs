use std::fmt;

type Address = u16;
type Register = u8;
type Byte = u8;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
	_0NNN(Address),
	_00E0,
	_00EE,
	_1NNN(Address),
	_2NNN(Address),
	_3XNN(Register, Byte),
	_4XNN(Register, Byte),
	_5XY0(Register, Register),
	_6XNN(Register, Byte),
	_7XNN(Register, Byte),
	_8XY0(Register, Register),
	_8XY1(Register, Register),
	_8XY2(Register, Register),
	_8XY3(Register, Register),
	_8XY4(Register, Register),
	_8XY5(Register, Register),
	_8XY6(Register),
	_8XY7(Register, Register),
	_8XYE(Register),
	_9XY0(Register, Register),
	ANNN(Address),
	BNNN(Address),
	CXNN(Register, Byte),
	DXYN(Register, Register, Byte),
	EX9E(Register),
	EXA1(Register),
	FX07(Register),
	FX0A(Register),
	FX15(Register),
	FX18(Register),
	FX1E(Register),
	FX29(Register),
	FX33(Register),
	FX55(Register),
	FX65(Register),
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::_0NNN(address) => write!(f, "SYS {:#05x}", address),
			Self::_00E0 => write!(f, "CLS"),
			Self::_00EE => write!(f, "RET"),
			Self::_1NNN(address) => write!(f, "JP {:#05x}", address),
			Self::_2NNN(address) => write!(f, "CALL {:#05x}", address),
			Self::_3XNN(vx, byte) => write!(f, "SE V{:X}, {:#04x}", vx, byte),
			Self::_4XNN(vx, byte) => write!(f, "SNE V{:X}, {:#04x}", vx, byte),
			Self::_5XY0(vx, vy) => write!(f, "SE V{:X}, V{:X}", vx, vy),
			Self::_6XNN(vx, byte) => write!(f, "LD V{:X}, {:#04x}", vx, byte),
			Self::_7XNN(vx, byte) => write!(f, "ADD V{:X}, {:#04x}", vx, byte),
			Self::_8XY0(vx, vy) => write!(f, "LD V{:X}, V{:X}", vx, vy),
			Self::_8XY1(vx, vy) => write!(f, "OR V{:X}, V{:X}", vx, vy),
			Self::_8XY2(vx, vy) => write!(f, "AND V{:X}, V{:X}", vx, vy),
			Self::_8XY3(vx, vy) => write!(f, "XOR V{:X}, V{:X}", vx, vy),
			Self::_8XY4(vx, vy) => write!(f, "ADD V{:X}, V{:X}", vx, vy),
			Self::_8XY5(vx, vy) => write!(f, "SUB V{:X}, V{:X}", vx, vy),
			Self::_8XY6(vx) => write!(f, "SHR V{:X}", vx),
			Self::_8XY7(vx, vy) => write!(f, "SUBN V{:X}, V{:X}", vx, vy),
			Self::_8XYE(vx) => write!(f, "SHL V{:X}", vx),
			Self::_9XY0(vx, vy) => write!(f, "SNE {}, {}", vx, vy),
			Self::ANNN(address) => write!(f, "LD I, {:#05x}", address),
			Self::BNNN(address) => write!(f, "JP V0), {:#05x}", address),
			Self::CXNN(vx, byte) => write!(f, "RND {}, {:#04x}", vx, byte),
			Self::DXYN(vx, vy, n) => {
				write!(f, "DRW V{:X}, V{:X}, {}", vx, vy, n)
			}
			Self::EX9E(vx) => write!(f, "SKP V{:X}", vx),
			Self::EXA1(vx) => write!(f, "SKNP V{:X}", vx),
			Self::FX07(vx) => write!(f, "LD V{:X}, DT", vx),
			Self::FX0A(vx) => write!(f, "LD V{:X}, K", vx),
			Self::FX15(vx) => write!(f, "LD DT, V{:X}", vx),
			Self::FX18(vx) => write!(f, "LD ST, V{:X}", vx),
			Self::FX1E(vx) => write!(f, "ADD I, V{:X}", vx),
			Self::FX29(vx) => write!(f, "LD F, V{:X}", vx),
			Self::FX33(vx) => write!(f, "LD B, V{:X}", vx),
			Self::FX55(vx) => write!(f, "LD [I, ]V{:X}", vx),
			Self::FX65(vx) => write!(f, "LD V{:X}, [I]", vx),
		}
	}
}
