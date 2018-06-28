use Cpu;
use CpuHooks;
use Cartridge;


pub struct Core
{
	pub clocks: usize,
	
	pub cartridge: Box<Cartridge>,
	pub cpu: Cpu,
	
	pub ram: [u8; 0x800],
	pub vram: [u8; 0x800],
	pub palram: [u8; 0x20],
}


impl Core
{
	pub fn new(cartridge: Box<Cartridge>) -> Core
	{
		Core
		{
			clocks: 0,
			
			cartridge: cartridge,
			cpu: Cpu::new(),
			
			ram: [0; 0x800],
			vram: [0; 0x800],
			palram: [0; 0x20],
		}
	}
	
	
	pub fn reset(&mut self)
	{
		self.ram = [0; 0x800];
		self.vram = [0; 0x800];
		self.palram = [0; 0x20];
		
		self.cpu.reset();
		
		self.clocks = 0;
	}
	
	
	pub fn clock(&mut self)
	{
		use std::mem;
		
		let core_ptr = unsafe { mem::transmute::<&mut Core, *mut Core>(self) };
		
		let cpu_hooks = CpuHooks
		{
			read: &move |addr| unsafe { Core::cpu_read(core_ptr, addr) },
			write: &move |addr, val| unsafe { Core::cpu_write(core_ptr, addr, val) },
			
			execute_instr: &|_cpu, _addr, _opcode, _imm1, _imm2|
			{
				/*println!("Clock {:5} | 0x{:04x} | A:{:02x} X:{:02x} Y:{:02x} S:{:02x} P:{:02x} | {}",
					cpu.clocks,
					addr,
					cpu.reg_a, cpu.reg_x, cpu.reg_y, cpu.reg_s, cpu.reg_p,
					cpu_dis::disassemble_instruction(addr, opcode, imm1, imm2));*/
			}
		};
		
		self.cpu.clock(&cpu_hooks);
		
		self.clocks = self.clocks.wrapping_add(3);
	}
	
	
	unsafe fn cpu_read(core: *mut Core, addr: u16) -> u8
	{
		use std::{ptr, mem};
		
		let cartridge_read = (*core).cartridge.cpu_read(addr);
		
		if addr < 0x2000
		{
			let mem_ptr = mem::transmute::<_, *mut u8>(&mut (*core).ram[0]);
			ptr::read(mem_ptr.offset((addr % 0x800) as isize))
		}
		
		else if addr < 0x3000
		{
			match addr % 8
			{
				2 => 0,//return this.ppu.readRegSTATUS()
				4 => 0,//return this.ppu.readRegOAMDATA()
				7 => 0,//return this.ppu.readRegDATA()
				_ => 0,//return 0
			}
		}
		
		else if addr == 0x4016
		{
			//const bit = this.controllerInput & 1
			//this.controllerInput >>= 1
			//this.controllerInput |= 0x80
			0//bit
		}
		
		else
			{ cartridge_read }
	}
	
	
	unsafe fn cpu_write(core: *mut Core, addr: u16, val: u8)
	{
		use std::{ptr, mem};
		
		(*core).cartridge.cpu_write(addr, val);
		
		if addr < 0x2000
		{
			let mem_ptr = mem::transmute::<_, *mut u8>(&mut (*core).ram[0]);
			ptr::write(mem_ptr.offset((addr % 0x800) as isize), val);
		}
	}
}