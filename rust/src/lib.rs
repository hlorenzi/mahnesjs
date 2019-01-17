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
fn test_cpu_simple()
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
	assert!(cpu.reg_p == 0x24);
	for _ in 0..1024
		{ cpu.clock(); }
	assert!(cpu.reg_a == 0x1c);
	assert!(cpu.reg_x == 0x1c);
	assert!(cpu.reg_p == 0x24);
}


#[test]
fn test_cpu_nestest()
{
	use std::{ptr, mem};
	use std::fs::File;
	use std::io::Read;
	
	let mut arr = [0_u8; 0x10000];
	let arr_ptr = unsafe { mem::transmute::<_, *mut u8>(&mut arr[0]) };
	
	let mut file = File::open("../examples/nestest.nes").unwrap();
	let mut buffer = Vec::<u8>::new();
	file.read_to_end(&mut buffer).unwrap();
	let ines = RomINES::new(&buffer);
	
	for i in 0..0x4000
		{ arr[0xc000 + i] = ines.prg_rom[i]; }
	
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
	
	cpu.reset();	
	cpu.set_pc(0xc000);
	cpu.clocks = 7;
	
	for _ in 0..14000
		{ cpu.clock(); }
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
	//let core_ptr = unsafe { std::mem::transmute::<_, *mut Core>(&mut core) };
	
	core.cpu.hook_execute_instr = Some(Box::new(move |cpu, addr, opcode, imm1, imm2|
	{
		//if cpu.clocks < 59670
		//	{ return; }
		
		println!("Clock {:5} | 0x{:04x} | A:{:02x} X:{:02x} Y:{:02x} S:{:02x} P:{:02x} | {}",
			cpu.clocks,
			addr,
			cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.reg_s, cpu.reg_p,
			cpu_dis::disassemble_instruction(addr, opcode, imm1, imm2));
			
		if addr == 0x8193
		{
			std::thread::sleep(std::time::Duration::from_millis(2500));
		}
			
		/*println!();
		
		for j in 0x0..0x10
		{
			print!("{:04x} : ", 0x100 + j * 0x10);
			
			for i in 0x0..0x10
			{
				let is_cur = cpu.reg_s as usize == (j * 0x10 + i);
					
				print!("{}", if is_cur { '[' } else { ' ' });
				print!("{:02x}", unsafe { (*core_ptr).ram[0x100 + j * 0x10 + i] });
				print!("{}", if is_cur { ']' } else { ' ' });
			}
				
			println!();
		}
		
		println!();
		
		std::thread::sleep(std::time::Duration::from_millis(500));*/
	}));
	
	for _ in 0..150000
		{ core.run(); }
}