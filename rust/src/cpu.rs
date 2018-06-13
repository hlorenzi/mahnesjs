use cpu_opcodes;


type CpuReadFn = Fn(u16) -> u8;
type CpuWriteFn = Fn(u16, u8);
type CpuExecuteInstrFn = Fn(u16, u8, u8, u8);


const FLAG_C : u8 = 0b00000001; // Carry
const FLAG_Z : u8 = 0b00000010; // Zero
const FLAG_I : u8 = 0b00000100; // Interrupt
const FLAG_D : u8 = 0b00001000; // Decimal
const FLAG_B : u8 = 0b00010000; // Break
const FLAG_U : u8 = 0b00100000; // Unused
const FLAG_V : u8 = 0b01000000; // Overflow
const FLAG_N : u8 = 0b10000000; // Negative


pub struct Cpu
{
	signal_nmi: bool,
	acknowledge_nmi: bool,
	
	signal_irq: bool,
	acknowledge_irq: bool,
	
	opcode: u8,
	opcode_step: u8,
	
	routine_reset: bool,
	routine_nmi: bool,
	
	reg_pc: u16,
	reg_a: u8,
	reg_x: u8,
	reg_y: u8,
	reg_s: u8,
	reg_p: u8,
	
	internal_addr: u16,
	internal_data: u8
}


pub struct CpuHooks<'a>
{
	pub read: &'a CpuReadFn,
	pub write: &'a CpuWriteFn,
	
	pub execute_instr: &'a CpuExecuteInstrFn
}


impl Cpu
{
	pub fn new() -> Cpu
	{
		Cpu
		{
			signal_nmi: false,
			acknowledge_nmi: false,
			
			signal_irq: false,
			acknowledge_irq: false,
			
			opcode: 0,
			opcode_step: 0,
			
			routine_reset: true,
			routine_nmi: false,
			
			reg_pc: 0,
			reg_a: 0,
			reg_x: 0,
			reg_y: 0,
			reg_s: 0,
			reg_p: 0,
			
			internal_addr: 0,
			internal_data: 0
		}
	}
	
	
	pub fn reset(&mut self)
	{
		self.opcode = 0;
		self.opcode_step = 0;
		
		self.routine_reset = true;
		self.routine_nmi = false;
		
		self.reg_pc = 0;
		self.reg_a = 0;
		self.reg_x = 0;
		self.reg_y = 0;
		self.reg_s = 0;
		self.reg_p = 0;
		
		self.internal_addr = 0;
		self.internal_data = 0;
	}
	
	
	pub fn clock(&mut self, hooks: &CpuHooks)
	{
		
	}
	
	
	fn dispatch_opcode(&mut self, hooks: &CpuHooks)
	{
		static OPCODE_TABLE: [[fn(&mut Cpu, &CpuHooks); 8]; 7] =
		[
			/* 0x00 BRK --- */ [Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x01 ORA ptx */ [Cpu::read_addr, Cpu::read_ptx1, Cpu::read_ptx2, Cpu::read_ptx3, Cpu::exec_ora, Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x02 --- --- */ [Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x03 --- --- */ [Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x04 --- --- */ [Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x05 ORA zer */ [Cpu::read_addr, Cpu::exec_ora,  Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x06 ASL zer */ [Cpu::read_addr, Cpu::read_data, Cpu::exec_asl,  Cpu::trap,      Cpu::trap,     Cpu::trap,      Cpu::trap,      Cpu::trap],
		];
	
		OPCODE_TABLE[self.opcode as usize][self.opcode_step as usize](self, hooks);
	}
	
	
	fn end_opcode(&mut self)
	{
		self.opcode_step = 0;
	}
	
	
	fn trap(self: &mut Cpu, _hooks: &CpuHooks)
	{
	
	}
	
	
	fn read_addr(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr = (hooks.read)(self.reg_pc) as u16;
		self.reg_pc = self.reg_pc.wrapping_add(1);
	}
	
	
	fn read_data(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.reg_pc);
	}
	
	
	fn read_ptx1(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.read)(self.internal_addr); // Dummy read
		self.internal_data = (self.internal_addr as u8).wrapping_add(self.reg_x);
	}
	
	
	fn read_ptx2(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr = (hooks.read)(self.internal_data as u16) as u16;
	}
	
	
	fn read_ptx3(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr |= ((hooks.read)(self.internal_data.wrapping_add(1) as u16) as u16) << 8;
	}
	
	
	fn read_pty(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
	}
	
	
	fn exec_ora(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
		self.reg_a |= self.internal_data;
		
		let reg_a = self.reg_a;
		self.adjust_flag_z(reg_a);
		self.adjust_flag_n(reg_a);
		
		self.end_opcode();
	}
	
	
	fn exec_asl(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.end_opcode();
	}
	
	
	fn adjust_flag_z(&mut self, val: u8)
	{
		self.reg_p &= !FLAG_Z;
		self.reg_p |= if val == 0 { FLAG_Z } else { 0 };
	}
	
	
	fn adjust_flag_n(&mut self, val: u8)
	{
		self.reg_p &= !FLAG_N;
		self.reg_p |= if (val & 0x80) != 0 { FLAG_N } else { 0 };
	}
}