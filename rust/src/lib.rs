mod core;
mod cartridge;
mod cpu;
pub mod cpu_opcodes;
pub mod cpu_dis;



pub use core::Core;
pub use cartridge::Cartridge;
pub use cpu::Cpu;
pub use cpu::CpuHooks;


#[test]
fn test()
{
	use std::{ptr, mem};
	
	let mut arr = [0_u8; 0x10000];
	let arr_ptr = unsafe { mem::transmute::<_, *mut u8>(&mut arr[0]) };
	
	let mut cpu = Cpu::new();
	
	let cpu_hooks = CpuHooks
	{
		read: &move |addr| unsafe { ptr::read(arr_ptr.offset(addr as isize)) },
		write: &move |addr, val| unsafe { ptr::write(arr_ptr.offset(addr as isize), val) },
		
		execute_instr: &|cpu, addr, opcode, imm1, imm2|
		{
			println!("Clock {:5} | 0x{:04x} | A:{:02x} X:{:02x} Y:{:02x} S:{:02x} P:{:02x} | {}",
				cpu.clocks,
				addr,
				cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.reg_s, cpu.reg_p,
				cpu_dis::disassemble_instruction(addr, opcode, imm1, imm2));
		}
	};
	
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
		{ cpu.clock(&cpu_hooks); }
	assert!(cpu.reg_a == 0x1c);
	assert!(cpu.reg_x == 0x1c);
	assert!(cpu.reg_p == 0x00);
}