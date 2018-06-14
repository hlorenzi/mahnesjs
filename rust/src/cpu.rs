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
	routine_irq: bool,
	
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
			routine_irq: false,
			
			reg_pc: 0,
			reg_a: 0,
			reg_x: 0,
			reg_y: 0,
			reg_s: 0xfd,
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
		self.routine_irq = false;
		
		self.reg_pc = 0;
		self.reg_a = 0;
		self.reg_x = 0;
		self.reg_y = 0;
		self.reg_s = 0xfd;
		self.reg_p = 0;
		
		self.internal_addr = 0;
		self.internal_data = 0;
	}
	
	
	pub fn drive_nmi(&mut self, active: bool)
	{
		if !self.signal_nmi && active
			{ self.acknowledge_nmi = true; }
			
		self.signal_nmi = true;
	}
	
	
	pub fn drive_irq(&mut self, active: bool)
	{
		if !self.signal_irq && active
			{ self.acknowledge_irq = true; }
			
		self.signal_irq = true;
	}
	
	
	pub fn clock(&mut self, hooks: &CpuHooks)
	{
		self.opcode_step += 1;
		
		if self.routine_reset
			{ self.run_reset_routine(hooks); }
			
		else if self.routine_nmi
			{ self.run_nmi_routine(hooks); }
		
		else if self.routine_irq
			{ self.run_irq_routine(hooks); }
			
		else
			{ self.dispatch_opcode(hooks); }
	}
	
	
	fn run_reset_routine(&mut self, hooks: &CpuHooks)
	{
		match self.opcode_step
		{
			1 | 2 | 3 | 4 => { }
			
			5 => self.reg_pc = (hooks.read)(0xfffc) as u16,
				
			6 =>
			{
				self.reg_pc |= ((hooks.read)(0xfffd) as u16) << 8;
				self.routine_reset = false;
				self.end_opcode();
			}
			
			_ => unreachable!()
		}
	}
	
	
	fn run_nmi_routine(&mut self, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		let reg_p = self.reg_p;
		
		match self.opcode_step
		{
			1 => { }
				
			2 => self.push_stack(hooks, (reg_pc >> 8) as u8),
				
			3 => self.push_stack(hooks, (reg_pc & 0xff) as u8),
			
			4 => self.push_stack(hooks, reg_p),
			
			5 => { }
			
			6 => self.reg_pc = (hooks.read)(0xfffa) as u16,
				
			7 =>
			{
				self.reg_pc |= ((hooks.read)(0xfffb) as u16) << 8;
				self.routine_nmi = false;
				self.end_opcode();
			}
			
			_ => unreachable!()
		}
	}
	
	
	fn run_irq_routine(&mut self, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		let reg_p = self.reg_p;
		
		match self.opcode_step
		{
			1 => self.reg_p &= !FLAG_I,
				
			2 => self.push_stack(hooks, (reg_pc >> 8) as u8),
				
			3 => self.push_stack(hooks, (reg_pc & 0xff) as u8),
			
			4 => self.push_stack(hooks, reg_p),
			
			5 => { }
			
			6 => self.reg_pc = (hooks.read)(0xfffe) as u16,
				
			7 =>
			{
				self.reg_pc |= ((hooks.read)(0xffff) as u16) << 8;
				self.routine_irq = false;
				self.end_opcode();
			}
			
			_ => unreachable!()
		}
	}
	
	
	fn dispatch_opcode(&mut self, hooks: &CpuHooks)
	{
		static OPCODE_TABLE: [[fn(&mut Cpu, &CpuHooks); 8]; 256] =
		[
			/* 0x00 BRK --- */ [Cpu::fetch_op, Cpu::read_data, Cpu::push_brk3, Cpu::push_brk4, Cpu::exec_brk5, Cpu::exec_brk6, Cpu::exec_stk7, Cpu::trap,      ], 
			/* 0x01 ORA ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x02 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x03 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x04 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x05 ORA zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x06 ASL zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x07 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x08 PHP --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::push_p,   Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x09 ORA imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x0a ASL --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x0b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x0c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x0d ORA abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x0e ASL abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x0f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x10 BPL rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x11 ORA pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x12 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x13 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x14 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x15 ORA zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x16 ASL zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x17 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x18 CLC --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x19 ORA aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x1a NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x1b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x1c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x1d ORA abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x1e ASL abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0x1f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x20 JSR abs */ [Cpu::fetch_op, Cpu::read_data, Cpu::dummy,    Cpu::exec_jsr4, Cpu::exec_jsr5, Cpu::exec_jsr6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x21 AND ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x22 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x23 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x24 BIT zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x25 AND zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x26 ROL zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x27 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x28 PLP --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_plp4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x29 AND imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x2a ROL --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x2b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x2c BIT abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x2d AND abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x2e ROL abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x2f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x30 BMI rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x31 AND pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x32 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x33 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x34 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x35 AND zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x36 ROL zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x37 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x38 SEC --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x39 AND aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x3a NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x3b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x3c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x3d AND abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x3e ROL abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0x3f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x40 RTI --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_rti4, Cpu::exec_rti5, Cpu::exec_rti6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x41 EOR ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x42 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x43 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x44 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x45 EOR zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x46 LSR zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x47 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x48 PHA --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::push_a,   Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x49 EOR imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x4a LSR --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x4b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x4c JMP abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_jmp3, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x4d EOR abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x4e LSR abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x4f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x50 BVC rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x51 EOR pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x52 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x53 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x54 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x55 EOR zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x56 LSR zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x57 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x58 CLI --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x59 EOR aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x5a NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x5b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x5c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x5d EOR abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x5e LSR abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0x5f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x60 RTS --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_rts4, Cpu::exec_rts5, Cpu::exec_rts6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x61 ADC ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x62 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x63 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x64 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x65 ADC zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x66 ROR zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x67 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x68 PLA --- */ [Cpu::fetch_op, Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_pla4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x69 ADC imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x6a ROR --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x6b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x6c JMP ind */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_ind4, Cpu::exec_ind5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x6d ADC abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x6e ROR abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x6f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x70 BVS rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x71 ADC pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x72 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x73 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x74 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x75 ADC zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x76 ROR zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0x77 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x78 SEI --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x79 ADC aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x7a NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x7b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x7c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x7d ADC abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x7e ROR abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0x7f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x80 NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x81 STA ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x82 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x83 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x84 STY zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x85 STA zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x86 STX zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x87 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x88 DEY --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x89 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8a TXA --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8c STY abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8d STA abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8e STX abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x8f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x90 BCC rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x91 STA pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      ], 
			/* 0x92 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x93 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x94 STY zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x95 STA zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x96 STX zry */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zry3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x97 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x98 TYA --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x99 STA aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9a TXS --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9b ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9c ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9d STA abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9e ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0x9f ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa0 LDY imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa1 LDA ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xa2 LDX imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa4 LDY zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa5 LDA zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa6 LDX zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa8 TAY --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xa9 LDA imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xaa TAX --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xab ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xac LDY abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xad LDA abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xae LDX abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xaf ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb0 BCS rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb1 LDA pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xb2 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb4 LDY zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb5 LDA zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb6 LDX zry */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zry3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb8 CLV --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xb9 LDA aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xba TSX --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xbb ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xbc LDY abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xbd LDA abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xbe LDX aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xbf ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc0 CPY imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc1 CMP ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xc2 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc4 CPY zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc5 CMP zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc6 DEC zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc8 INY --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xc9 CMP imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xca DEX --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xcb ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xcc CPY abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xcd CMP abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xce DEC abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0xcf ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd0 BNE rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd1 CMP pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xd2 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd4 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd5 CMP zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd6 DEC zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0xd7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd8 CLD --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xd9 CMP aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xda NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xdb ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xdc ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xdd CMP abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xde DEC abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0xdf ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe0 CPX imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe1 SBC ptx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xe2 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe4 CPX zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe5 SBC zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe6 INC zer */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe8 INX --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xe9 SBC imm */ [Cpu::fetch_op, Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xea NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xeb ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xec CPX abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xed SBC abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xee INC abs */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0xef ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf0 BEQ rel */ [Cpu::fetch_op, Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf1 SBC pty */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_pty3, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      ], 
			/* 0xf2 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf3 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf4 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf5 SBC zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf6 INC zrx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,      ], 
			/* 0xf7 ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf8 SED --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xf9 SBC aby */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xfa NOP --- */ [Cpu::fetch_op, Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xfb ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xfc ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xfd SBC abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
			/* 0xfe INC abx */ [Cpu::fetch_op, Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,      ], 
			/* 0xff ??? --- */ [Cpu::fetch_op, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      ], 
		];
	
		OPCODE_TABLE[self.opcode as usize][(self.opcode_step - 1) as usize](self, hooks);
	}
	
	
	fn end_opcode(&mut self)
	{
		self.opcode_step = 0;
	}
	
	
	fn end_opcode_and_prefetch(&mut self, hooks: &CpuHooks)
	{
		self.opcode_step = 1;
		Cpu::fetch_op(self, hooks);
	}
	
	
	fn increment_pc(&mut self)
	{
		self.reg_pc = self.reg_pc.wrapping_add(1);
	}
	
	
	fn trap(self: &mut Cpu, _hooks: &CpuHooks)
	{
		panic!("unhandled opcode(0x{:x}) step({})", self.opcode, self.opcode_step);
	}
	
	
	fn dummy(_self: &mut Cpu, _hooks: &CpuHooks)
	{
	
	}
	
	
	fn fetch_op(self: &mut Cpu, hooks: &CpuHooks)
	{
		if self.acknowledge_nmi
		{
			self.acknowledge_nmi = false;
			self.routine_nmi = true;
		}
		
		else if self.acknowledge_irq
		{
			self.acknowledge_irq = false;
			if (self.reg_p & FLAG_I) == 0
				{ self.routine_irq = true; }
		}
		
		else
		{	
			self.opcode = (hooks.read)(self.reg_pc);
			self.increment_pc();
		}
	}
	
	
	fn exec_imm(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.reg_pc);
		self.increment_pc();
		self.exec_op(hooks);
		self.end_opcode();
	}
	
	
	fn exec_imp(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.reg_pc);
		self.exec_op(hooks);
		self.end_opcode();
	}
	
	
	fn read_addr(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr = (hooks.read)(self.reg_pc) as u16;
		self.increment_pc();
	}
	
	
	fn read_data(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.reg_pc);
	}
	
	
	fn read_dat2(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.reg_pc);
		self.increment_pc();
	}
	
	
	fn exec_rd1(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
		self.exec_op(hooks);
		self.end_opcode();
	}
	
	
	fn exec_mdf1(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
	}
	
	
	fn exec_mdf2(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data); // Dummy write
		self.exec_op(hooks);
	}
	
	
	fn exec_wrt1(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.exec_op(hooks);
		self.end_opcode();
	}
	
	
	fn exec_wrt2(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
		self.exec_op(hooks);
		self.end_opcode();
	}
	
	
	fn exec_zrx3(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.read)(self.internal_addr); // Dummy read
		self.internal_addr = Cpu::calculate_effective_addr(self.internal_addr, self.reg_x, false);
	}
	
	
	fn exec_zry3(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.read)(self.internal_addr); // Dummy read
		self.internal_addr = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, false);
	}
	
	
	fn exec_abs3(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr |= ((hooks.read)(self.reg_pc) as u16) << 8;
		self.increment_pc();
	}
	
	
	fn exec_jmp3(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr |= ((hooks.read)(self.reg_pc) as u16) << 8;
		self.reg_pc = self.internal_addr;
		self.end_opcode();
	}
	
	
	fn exec_ptx3(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.read)(self.internal_addr); // Dummy read
		self.internal_data = (self.internal_addr as u8).wrapping_add(self.reg_x);
	}
	
	
	fn exec_pty3(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
	}
	
	
	fn exec_rel3(self: &mut Cpu, hooks: &CpuHooks)
	{
		let branch_taken = match self.opcode
		{
			cpu_opcodes::BPL => (self.reg_p & FLAG_N) == 0,
			cpu_opcodes::BMI => (self.reg_p & FLAG_N) != 0,
			cpu_opcodes::BVC => (self.reg_p & FLAG_V) == 0,
			cpu_opcodes::BVS => (self.reg_p & FLAG_V) != 0,
			cpu_opcodes::BCC => (self.reg_p & FLAG_C) == 0,
			cpu_opcodes::BCS => (self.reg_p & FLAG_C) != 0,
			cpu_opcodes::BNE => (self.reg_p & FLAG_Z) == 0,
			cpu_opcodes::BEQ => (self.reg_p & FLAG_Z) != 0,
			_ => unreachable!()
		};
		
		if branch_taken
			{ (hooks.read)(self.internal_addr); } // Dummy read
		else
			{ self.end_opcode_and_prefetch(hooks); }
	}
	
	
	fn push_brk3(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		self.push_stack(hooks, (reg_pc >> 8) as u8);
	}
	
	
	fn incr_s(self: &mut Cpu, _hooks: &CpuHooks)
	{
		self.reg_s.wrapping_add(1);
	}
	
	
	fn push_a(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_a = self.reg_a;
		self.push_stack(hooks, reg_a);
		self.end_opcode();
	}
	
	
	fn push_p(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_p = self.reg_p;
		self.push_stack(hooks, reg_p | FLAG_B | FLAG_U);
		self.end_opcode();
	}
	
	
	fn exec_abx4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_x, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_x, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
	}
	
	
	fn exec_abx4_r(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_x, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_x, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
		
		if addr_without_carry == addr_with_carry
		{
			self.exec_op(hooks);
			self.end_opcode();
		}
	}
	
	
	fn exec_aby4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
	}
	
	
	fn exec_aby4_r(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
		
		if addr_without_carry == addr_with_carry
		{
			self.exec_op(hooks);
			self.end_opcode();
		}
	}
	
	
	fn exec_ptx4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr = (hooks.read)(self.internal_data as u16) as u16;
	}
	
	
	fn exec_ind4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_data = (hooks.read)(self.internal_addr);
	}
	
	
	fn exec_pty4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr = ((hooks.read)(self.internal_addr.wrapping_add(1) & 0xff) as u16) << 8;
		self.internal_addr |= self.internal_data as u16;
	}
	
	
	fn exec_rel4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_branch_addr(self.reg_pc, self.internal_data, false);
		let addr_with_carry    = Cpu::calculate_branch_addr(self.reg_pc, self.internal_data, true);
	
		if addr_without_carry == addr_with_carry
		{
			self.reg_pc = addr_with_carry;
			self.end_opcode_and_prefetch(hooks);
		}
		else
			{ (hooks.read)(addr_without_carry); } // Dummy read
	}
	
	
	fn push_brk4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		self.push_stack(hooks, (reg_pc & 0xff) as u8);
	}
	
	
	fn exec_rti4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_p = self.read_stack(hooks);
		self.reg_s = self.reg_s.wrapping_add(1);
	}
	
	
	fn exec_rts4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = self.read_stack(hooks) as u16;
		self.reg_s = self.reg_s.wrapping_add(1);
	}
	
	
	fn exec_pla4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_a = self.read_stack(hooks);
		self.adjust_flag_z(reg_a);
		self.adjust_flag_n(reg_a);
		self.reg_a = reg_a;
		self.end_opcode();
	}
	
	
	fn exec_plp4(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_p = self.read_stack(hooks);
		self.end_opcode();
	}
	
	
	fn exec_jsr4(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		self.push_stack(hooks, (reg_pc >> 8) as u8);
	}
	
	
	fn exec_zer5(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.end_opcode();
	}
	
	
	fn exec_abs5(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.exec_op(hooks);
	}
	
	
	fn exec_ptx5(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.internal_addr |= (hooks.read)(self.internal_data.wrapping_add(1) as u16) as u16;
	}
	
	
	fn exec_pty5(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
	}
	
	
	fn exec_pty5_r(self: &mut Cpu, hooks: &CpuHooks)
	{
		let addr_without_carry = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, false);
		let addr_with_carry    = Cpu::calculate_effective_addr(self.internal_addr, self.reg_y, true);
	
		self.internal_addr = addr_with_carry;
		self.internal_data = (hooks.read)(addr_without_carry); // Wrong read if address needs carry
		
		if addr_without_carry == addr_with_carry
		{
			self.exec_op(hooks);
			self.end_opcode();
		}
	}
	
	
	fn exec_rel5(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = Cpu::calculate_branch_addr(self.reg_pc, self.internal_data, true);
		self.end_opcode_and_prefetch(hooks);
	}
	
	
	fn exec_ind5(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = self.internal_data as u16;
		self.reg_pc |= ((hooks.read)((self.internal_addr & 0xff00) | (self.internal_addr.wrapping_add(1) & 0xff)) as u16) << 8;
		self.end_opcode();
	}
	
	
	fn exec_brk5(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_p = self.reg_p;
		self.push_stack(hooks, reg_p);
	}
	
	
	fn exec_rti5(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = self.read_stack(hooks) as u16;
		self.reg_s = self.reg_s.wrapping_add(1);
	}
	
	
	fn exec_rts5(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc |= (self.read_stack(hooks) as u16) << 8;
	}
	
	
	fn exec_jsr5(self: &mut Cpu, hooks: &CpuHooks)
	{
		let reg_pc = self.reg_pc;
		self.push_stack(hooks, (reg_pc & 0xff) as u8);
	}
	
	
	fn exec_abs6(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.end_opcode();
	}
	
	
	fn exec_abx6(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.exec_op(hooks);
	}
	
	
	fn exec_brk6(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = (hooks.read)(0xfffe) as u16;
	}
	
	
	fn exec_rti6(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc |= (self.read_stack(hooks) as u16) << 8;
		self.end_opcode();
	}
	
	
	fn exec_rts6(self: &mut Cpu, _hooks: &CpuHooks)
	{
		self.increment_pc();
		self.end_opcode();
	}
	
	
	fn exec_jsr6(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc = (self.internal_data as u16) | (((hooks.read)(self.reg_pc) as u16) << 8);
		self.end_opcode();
	}
	
	
	fn exec_abx7(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.end_opcode();
	}
	
	
	/*fn exec_ptx7(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.exec_op(hooks);
	}*/
	
	
	fn exec_stk7(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc |= ((hooks.read)(0xffff) as u16) << 8;
		self.end_opcode();
	}
	
	
	/*fn exec_ptx8(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.end_opcode();
	}*/
	
	
	fn exec_op(self: &mut Cpu, hooks: &CpuHooks)
	{
		match self.opcode
		{
			cpu_opcodes::NOP |
			cpu_opcodes::NOP_2 |
			cpu_opcodes::NOP_3 |
			cpu_opcodes::NOP_4 |
			cpu_opcodes::NOP_5 |
			cpu_opcodes::NOP_6 |
			cpu_opcodes::NOP_7 |
			cpu_opcodes::NOP_8 |
			cpu_opcodes::PHA |
			cpu_opcodes::PLA |
			cpu_opcodes::PHP |
			cpu_opcodes::PLP =>
				{ }
				
			cpu_opcodes::CLC =>
				self.reg_p &= !FLAG_C,
				
			cpu_opcodes::SEC =>
				self.reg_p |= FLAG_C,
				
			cpu_opcodes::CLI =>
				self.reg_p &= !FLAG_I,
				
			cpu_opcodes::SEI =>
				self.reg_p |= FLAG_I,
				
			cpu_opcodes::CLD =>
				self.reg_p &= !FLAG_D,
				
			cpu_opcodes::SED =>
				self.reg_p |= FLAG_D,
				
			cpu_opcodes::CLV =>
				self.reg_p &= !FLAG_V,
				
			cpu_opcodes::TXA =>
			{
				let val = self.reg_x;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::TAX =>
			{
				let val = self.reg_a;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_x = val;
			}
				
			cpu_opcodes::TYA =>
			{
				let val = self.reg_y;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::TAY =>
			{
				let val = self.reg_a;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_y = val;
			}
				
			cpu_opcodes::TXS =>
				{ self.reg_s = self.reg_x; }
				
			cpu_opcodes::TSX =>
			{
				let val = self.reg_s;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_x = val;
			}
				
			cpu_opcodes::INX =>
			{
				let val = self.reg_x.wrapping_add(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_x = val;
			}
				
			cpu_opcodes::DEX =>
			{
				let val = self.reg_x.wrapping_sub(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_x = val;
			}
				
			cpu_opcodes::INY =>
			{
				let val = self.reg_y.wrapping_add(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_y = val;
			}
				
			cpu_opcodes::DEY =>
			{
				let val = self.reg_y.wrapping_sub(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_y = val;
			}
		
			cpu_opcodes::LDA_IMM |
			cpu_opcodes::LDA_ZER |
			cpu_opcodes::LDA_ZRX |
			cpu_opcodes::LDA_ABS |
			cpu_opcodes::LDA_ABX |
			cpu_opcodes::LDA_ABY |
			cpu_opcodes::LDA_PTX |
			cpu_opcodes::LDA_PTY =>
			{
				self.reg_a = self.internal_data;
				
				let reg_a = self.reg_a;
				self.adjust_flag_z(reg_a);
				self.adjust_flag_n(reg_a);
			}
			
			cpu_opcodes::LDX_IMM |
			cpu_opcodes::LDX_ZER |
			cpu_opcodes::LDX_ZRY |
			cpu_opcodes::LDX_ABS |
			cpu_opcodes::LDX_ABY =>
			{
				self.reg_x = self.internal_data;
				
				let reg_x = self.reg_x;
				self.adjust_flag_z(reg_x);
				self.adjust_flag_n(reg_x);
			}
			
			cpu_opcodes::LDY_IMM |
			cpu_opcodes::LDY_ZER |
			cpu_opcodes::LDY_ZRX |
			cpu_opcodes::LDY_ABS |
			cpu_opcodes::LDY_ABX =>
			{
				self.reg_y = self.internal_data;
				
				let reg_y = self.reg_y;
				self.adjust_flag_z(reg_y);
				self.adjust_flag_n(reg_y);
			}
		
			cpu_opcodes::STA_ZER |
			cpu_opcodes::STA_ZRX |
			cpu_opcodes::STA_ABS |
			cpu_opcodes::STA_ABX |
			cpu_opcodes::STA_ABY |
			cpu_opcodes::STA_PTX |
			cpu_opcodes::STA_PTY =>
				(hooks.write)(self.internal_addr, self.reg_a),
			
			cpu_opcodes::STX_ZER |
			cpu_opcodes::STX_ZRY |
			cpu_opcodes::STX_ABS =>
				(hooks.write)(self.internal_addr, self.reg_x),
		
			cpu_opcodes::STY_ZER |
			cpu_opcodes::STY_ZRX |
			cpu_opcodes::STY_ABS =>
				(hooks.write)(self.internal_addr, self.reg_y),
			
			cpu_opcodes::ADC_IMM |
			cpu_opcodes::ADC_ZER |
			cpu_opcodes::ADC_ZRX |
			cpu_opcodes::ADC_ABS |
			cpu_opcodes::ADC_ABX |
			cpu_opcodes::ADC_ABY |
			cpu_opcodes::ADC_PTX |
			cpu_opcodes::ADC_PTY =>
			{
				let val = (self.reg_a as u16)
					.wrapping_add(self.internal_data as u16)
					.wrapping_add(if (self.reg_p & FLAG_C) != 0 { 1 } else { 0 });
					
				self.adjust_flag_z((val & 0xff) as u8);
				self.adjust_flag_n((val & 0xff) as u8);
				
				let overflow =
					(((self.reg_a ^ self.internal_data) & 0x80) == 0) &&
					(((self.reg_a ^ ((val & 0xff) as u8)) & 0x80) != 0);
					
				let carry = val > 0xff;
				
				self.reg_p &= !(FLAG_V | FLAG_C);
				self.reg_p |= if overflow { FLAG_V } else { 0 };
				self.reg_p |= if carry { FLAG_C } else { 0 };
					
				self.reg_a = (val & 0xff) as u8;
			}
			
			cpu_opcodes::SBC_IMM |
			cpu_opcodes::SBC_ZER |
			cpu_opcodes::SBC_ZRX |
			cpu_opcodes::SBC_ABS |
			cpu_opcodes::SBC_ABX |
			cpu_opcodes::SBC_ABY |
			cpu_opcodes::SBC_PTX |
			cpu_opcodes::SBC_PTY =>
			{
				let val = (self.reg_a as u16)
					.wrapping_sub(self.internal_data as u16)
					.wrapping_sub(if (self.reg_p & FLAG_C) != 0 { 0 } else { 1 });
					
				self.adjust_flag_z((val & 0xff) as u8);
				self.adjust_flag_n((val & 0xff) as u8);
				
				let overflow =
					(((self.reg_a ^ self.internal_data) & 0x80) != 0) &&
					(((self.reg_a ^ ((val & 0xff) as u8)) & 0x80) != 0);
					
				let carry = val > 0xff;
				
				self.reg_p &= !(FLAG_V | FLAG_C);
				self.reg_p |= if overflow { FLAG_V } else { 0 };
				self.reg_p |= if carry { FLAG_C } else { 0 };
					
				self.reg_a = (val & 0xff) as u8;
			}
			
			cpu_opcodes::CMP_IMM |
			cpu_opcodes::CMP_ZER |
			cpu_opcodes::CMP_ZRX |
			cpu_opcodes::CMP_ABS |
			cpu_opcodes::CMP_ABX |
			cpu_opcodes::CMP_ABY |
			cpu_opcodes::CMP_PTX |
			cpu_opcodes::CMP_PTY =>
			{
				let val = (self.reg_a as u16)
					.wrapping_sub(self.internal_data as u16);
					
				self.adjust_flag_z((val & 0xff) as u8);
				self.adjust_flag_n((val & 0xff) as u8);
				
				let carry = val > 0xff;
				
				self.reg_p &= !FLAG_C;
				self.reg_p |= if carry { FLAG_C } else { 0 };
			}
			
			cpu_opcodes::CPX_IMM |
			cpu_opcodes::CPX_ZER |
			cpu_opcodes::CPX_ABS =>
			{
				let val = (self.reg_x as u16)
					.wrapping_sub(self.internal_data as u16);
					
				self.adjust_flag_z((val & 0xff) as u8);
				self.adjust_flag_n((val & 0xff) as u8);
				
				let carry = val > 0xff;
				
				self.reg_p &= !FLAG_C;
				self.reg_p |= if carry { FLAG_C } else { 0 };
			}
			
			cpu_opcodes::CPY_IMM |
			cpu_opcodes::CPY_ZER |
			cpu_opcodes::CPY_ABS =>
			{
				let val = (self.reg_y as u16)
					.wrapping_sub(self.internal_data as u16);
					
				self.adjust_flag_z((val & 0xff) as u8);
				self.adjust_flag_n((val & 0xff) as u8);
				
				let carry = val > 0xff;
				
				self.reg_p &= !FLAG_C;
				self.reg_p |= if carry { FLAG_C } else { 0 };
			}
			
			cpu_opcodes::AND_IMM |
			cpu_opcodes::AND_ZER |
			cpu_opcodes::AND_ZRX |
			cpu_opcodes::AND_ABS |
			cpu_opcodes::AND_ABX |
			cpu_opcodes::AND_ABY |
			cpu_opcodes::AND_PTX |
			cpu_opcodes::AND_PTY =>
			{
				let val = self.reg_a & self.internal_data;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				
				self.reg_a = val;
			}
			
			cpu_opcodes::ORA_IMM |
			cpu_opcodes::ORA_ZER |
			cpu_opcodes::ORA_ZRX |
			cpu_opcodes::ORA_ABS |
			cpu_opcodes::ORA_ABX |
			cpu_opcodes::ORA_ABY |
			cpu_opcodes::ORA_PTX |
			cpu_opcodes::ORA_PTY =>
			{
				let val = self.reg_a | self.internal_data;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				
				self.reg_a = val;
			}
			
			cpu_opcodes::EOR_IMM |
			cpu_opcodes::EOR_ZER |
			cpu_opcodes::EOR_ZRX |
			cpu_opcodes::EOR_ABS |
			cpu_opcodes::EOR_ABX |
			cpu_opcodes::EOR_ABY |
			cpu_opcodes::EOR_PTX |
			cpu_opcodes::EOR_PTY =>
			{
				let val = self.reg_a ^ self.internal_data;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				
				self.reg_a = val;
			}
			
			cpu_opcodes::BIT_ZER |
			cpu_opcodes::BIT_ABS =>
			{
				let val_z = self.reg_a & self.internal_data;
				let val_n = self.internal_data;
				self.adjust_flag_z(val_z);
				self.adjust_flag_n(val_n);
				
				self.reg_p &= !FLAG_V;
				self.reg_p |= if (self.internal_data & 0x40) != 0 { FLAG_V } else { 0 };
			}
				
			cpu_opcodes::ASL_IMP =>
			{
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (self.reg_a & 0x80) != 0 { FLAG_C } else { 0 };
				let val = self.reg_a << 1;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::LSR_IMP =>
			{
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (self.reg_a & 0x01) != 0 { FLAG_C } else { 0 };
				let val = self.reg_a >> 1;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::ROL_IMP =>
			{
				let val = ((self.reg_a as u16) << 1) | if (self.reg_p & FLAG_C) != 0 { 1 } else { 0 };
				self.reg_p &= !FLAG_C;
				self.reg_p |= if val > 0xff { FLAG_C } else { 0 };
				
				let val = val as u8;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::ROR_IMP =>
			{
				let val = (self.reg_a as u16) | if (self.reg_p & FLAG_C) != 0 { 0x100 } else { 0 };
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (val & 0x01) != 0 { FLAG_C } else { 0 };
				
				let val = (val >> 1) as u8;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.reg_a = val;
			}
				
			cpu_opcodes::ASL_ZER |
			cpu_opcodes::ASL_ZRX |
			cpu_opcodes::ASL_ABS |
			cpu_opcodes::ASL_ABX =>
			{
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (self.internal_data & 0x80) != 0 { FLAG_C } else { 0 };
				let val = self.internal_data << 1;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
				
			cpu_opcodes::LSR_ZER |
			cpu_opcodes::LSR_ZRX |
			cpu_opcodes::LSR_ABS |
			cpu_opcodes::LSR_ABX =>
			{
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (self.internal_data & 0x01) != 0 { FLAG_C } else { 0 };
				let val = self.internal_data >> 1;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
				
			cpu_opcodes::ROL_ZER |
			cpu_opcodes::ROL_ZRX |
			cpu_opcodes::ROL_ABS |
			cpu_opcodes::ROL_ABX =>
			{
				let val = ((self.internal_data as u16) << 1) | if (self.reg_p & FLAG_C) != 0 { 1 } else { 0 };
				self.reg_p &= !FLAG_C;
				self.reg_p |= if val > 0xff { FLAG_C } else { 0 };
				
				let val = val as u8;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
				
			cpu_opcodes::ROR_ZER |
			cpu_opcodes::ROR_ZRX |
			cpu_opcodes::ROR_ABS |
			cpu_opcodes::ROR_ABX =>
			{
				let val = (self.internal_data as u16) | if (self.reg_p & FLAG_C) != 0 { 0x100 } else { 0 };
				self.reg_p &= !FLAG_C;
				self.reg_p |= if (val & 0x01) != 0 { FLAG_C } else { 0 };
				
				let val = (val >> 1) as u8;
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
				
			cpu_opcodes::INC_ZER |
			cpu_opcodes::INC_ZRX |
			cpu_opcodes::INC_ABS |
			cpu_opcodes::INC_ABX =>
			{
				let val = self.internal_data.wrapping_add(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
				
			cpu_opcodes::DEC_ZER |
			cpu_opcodes::DEC_ZRX |
			cpu_opcodes::DEC_ABS |
			cpu_opcodes::DEC_ABX =>
			{
				let val = self.internal_data.wrapping_sub(1);
				self.adjust_flag_z(val);
				self.adjust_flag_n(val);
				self.internal_data = val;
			}
			
			_ => unreachable!()
		}
	}
	
	
	fn calculate_effective_addr(base: u16, offset: u8, carry: bool) -> u16
	{
		if carry
			{ base.wrapping_add(offset as u16) }
		else
			{ (base & 0xff00) | (base.wrapping_add(offset as u16) & 0xff) }
	}
	
	
	fn calculate_branch_addr(addr: u16, offset: u8, carry: bool) -> u16
	{
		let signed_offset = if (offset & 0x80) == 0
			{ offset as u16 }
		else
			{ 0xff00 | (offset as u16) };
	
		if carry
			{ addr.wrapping_add(signed_offset) }
		else
			{ (addr & 0xff00) | (addr.wrapping_add(signed_offset) & 0xff) }
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
	
	
	fn push_stack(&mut self, hooks: &CpuHooks, value: u8)
	{
		(hooks.write)(0x100 + (self.reg_s as u16), value);
		self.reg_s = self.reg_s.wrapping_sub(1);
	}
	
	
	fn read_stack(&mut self, hooks: &CpuHooks) -> u8
	{
		(hooks.read)(0x100 + (self.reg_s as u16))
	}
}