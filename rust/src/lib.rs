mod core;
mod cpu;
mod ppu;
mod rom_ines;
mod cartridge;
mod cartridge_nrom;
mod wasm;


pub mod cpu_opcodes;
pub mod cpu_dis;


pub use core::Core;
pub use cartridge::Cartridge;
pub use cartridge_nrom::CartridgeNROM;
pub use cpu::Cpu;
pub use ppu::Ppu;
pub use rom_ines::RomINES;


#[test]
fn test()
{
	use std::{ptr, mem};
	
	let mut arr = [0_u8; 0x10000];
	let arr_ptr = unsafe { mem::transmute::<_, *mut u8>(&mut arr[0]) };
	
	let mut cpu = Cpu::new();
	
	cpu.hook_read = Box::new(move |addr| unsafe { ptr::read(arr_ptr.offset(addr as isize)) });
	cpu.hook_write = Box::new(move |addr, val| unsafe { ptr::write(arr_ptr.offset(addr as isize), val) });
	
	cpu.hook_execute_instr = Some(Box::new(move |cpu, addr, opcode, imm1, imm2|
	{
		println!("Clock {:5} | 0x{:04x} | A:{:02x} X:{:02x} Y:{:02x} S:{:02x} P:{:02x} | {}",
			cpu.clocks,
			addr,
			cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.reg_s, cpu.reg_p,
			cpu_dis::disassemble_instruction(addr, opcode, imm1, imm2));
	}));
	
	arr[0] = cpu_opcodes::LDA_IMM;
	arr[1] = 0xab;
	arr[2] = cpu_opcodes::TAX;
	arr[3] = cpu_opcodes::INX;
	arr[4] = cpu_opcodes::TXA;
	arr[5] = cpu_opcodes::JMP_ABS;
	arr[6] = 0x02;
	arr[7] = 0x00;
	
	cpu.reset();
	
	assert!(cpu.reg_a == 0x00);
	assert!(cpu.reg_x == 0x00);
	assert!(cpu.reg_p == 0x00);
	for _ in 0..1024
		{ cpu.clock(); }
	assert!(cpu.reg_a == 0x1c);
	assert!(cpu.reg_x == 0x1c);
	assert!(cpu.reg_p == 0x00);
}


#[test]
fn test_core_bkgcolor()
{
	use std::fs::File;
	use std::io::Read;
	
	let mut file = File::open("../examples/bkgcolor.nes").unwrap();
	let mut buffer = Vec::<u8>::new();
	file.read_to_end(&mut buffer).unwrap();
	
	
	let ines = RomINES::new(&buffer);
	let cartridge = ines.make_cartridge().unwrap();

	let mut core = Core::new(Box::new(cartridge));
	
	core.cpu.hook_execute_instr = Some(Box::new(move |cpu, addr, opcode, imm1, imm2|
	{
		println!("Clock {:5} | 0x{:04x} | A:{:02x} X:{:02x} Y:{:02x} S:{:02x} P:{:02x} | {}",
			cpu.clocks,
			addr,
			cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.reg_s, cpu.reg_p,
			cpu_dis::disassemble_instruction(addr, opcode, imm1, imm2));
	}));
	
	for _ in 0..40000
		{ core.run(); }
}