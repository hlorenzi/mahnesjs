mod cpu;
pub mod cpu_opcodes;


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
		
		execute_instr: &|_, _, _, _| {}
	};
	
	cpu.clock(&cpu_hooks);
	cpu.clock(&cpu_hooks);
	cpu.clock(&cpu_hooks);
	cpu.clock(&cpu_hooks);
}