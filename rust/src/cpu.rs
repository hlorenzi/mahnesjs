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
		self.dispatch_opcode(hooks);
	}
	
	
	fn dispatch_opcode(&mut self, hooks: &CpuHooks)
	{
		static OPCODE_TABLE: [[fn(&mut Cpu, &CpuHooks); 8]; 256] =
		[
			/* 0x0 brk --- */ [Cpu::read_data, Cpu::push_brk3,  Cpu::push_brk4, Cpu::exec_brk5, Cpu::exec_brk6, Cpu::exec_stk7, Cpu::trap,    Cpu::trap],
			/* 0x1 ora ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x2 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x3 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,     Cpu::trap],
			/* 0x4 --- --- */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x5 ora zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x6 asl zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x7 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x8 php --- */ [Cpu::read_dat2, Cpu::push_p,   Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap],
			/* 0x9 ora imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xa asl --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xb --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xc --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xd ora abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xe asl abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xf --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x10 bpl rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x11 ora pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x12 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x13 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x14 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x15 ora zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x16 asl zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x17 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x18 clc --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x19 ora aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x1a nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x1b --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x1c --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0x1d ora abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x1e asl abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0x1f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x20 jsr abs */ [Cpu::read_data, Cpu::dummy,    Cpu::exec_jsr4, Cpu::exec_jsr5, Cpu::exec_jsr6, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x21 and ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x22 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x23 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x24 bit zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x25 and zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x26 rol zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x27 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x28 plp --- */ [Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_plp4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x29 and imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2a rol --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2b --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2c bit abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2d and abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2e rol abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x2f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x30 bmi rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x31 and pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x32 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x33 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x34 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x35 and zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x36 rol zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x37 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x38 sec --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x39 and aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x3a nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x3b --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x3c --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0x3d and abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x3e rol abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0x3f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x40 rti --- */ [Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_rti4, Cpu::exec_rti5, Cpu::exec_rti6, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x41 eor ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x42 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x43 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x44 --- --- */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x45 eor zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x46 lsr zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x47 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x48 pha --- */ [Cpu::read_dat2, Cpu::push_a,   Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x49 eor imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4a lsr --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4b --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4c jmp abs */ [Cpu::read_addr, Cpu::exec_jmp3, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4d eor abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4e lsr abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x4f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x50 bvc rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x51 eor pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x52 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x53 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x54 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x55 eor zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x56 lsr zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x57 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x58 cli --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x59 eor aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x5a nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x5b --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x5c --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0x5d eor abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x5e lsr abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0x5f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x60 rts --- */ [Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_rts4, Cpu::exec_rts5, Cpu::exec_rts6, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x61 adc ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x62 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x63 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x64 --- --- */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x65 adc zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x66 ror zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x67 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x68 pla --- */ [Cpu::read_dat2, Cpu::incr_s,   Cpu::exec_pla4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0x69 adc imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6a ror --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6b --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6c jmp ind */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_ind4, Cpu::exec_ind5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6d adc abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6e ror abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x6f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x70 bvs rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x71 adc pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x72 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x73 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0x74 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x75 adc zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x76 ror zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x77 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x78 sei --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x79 adc aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x7a nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x7b --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x7c --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0x7d adc abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0x7e ror abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0x7f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x80 nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x81 sta ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x82 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x83 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x84 sty zer */ [Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x85 sta zer */ [Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x86 stx zer */ [Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x87 --- --- */ [Cpu::read_addr, Cpu::exec_wrt2,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0x88 dey --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x89 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8a txa --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8b --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8c sty abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8d sta abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8e stx abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x8f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x90 bcc rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x91 sta pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x92 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x93 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x94 sty zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x95 sta zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x96 stx zry */ [Cpu::read_addr, Cpu::exec_zry3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x97 --- --- */ [Cpu::read_addr, Cpu::exec_zry3, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x98 tya --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x99 sta aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9a txs --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9b --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9c --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9d sta abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_wrt1, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9e --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0x9f --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa0 ldy imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa1 lda ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xa2 ldx imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa3 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa4 ldy zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa5 lda zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa6 ldx zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa7 --- --- */ [Cpu::read_addr, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa8 tay --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xa9 lda imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xaa tax --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xab --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xac ldy abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xad lda abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xae ldx abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xaf --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb0 bcs rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb1 lda pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0xb2 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb3 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb4 ldy zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb5 lda zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb6 ldx zry */ [Cpu::read_addr, Cpu::exec_zry3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb7 --- --- */ [Cpu::read_addr, Cpu::exec_zry3, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb8 clv --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xb9 lda aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xba tsx --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xbb --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xbc ldy abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xbd lda abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xbe ldx aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xbf --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc0 cpy imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc1 cmp ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xc2 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc3 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0xc4 cpy zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc5 cmp zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc6 dec zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc7 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc8 iny --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xc9 cmp imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xca dex --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xcb --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xcc cpy abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xcd cmp abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xce dec abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xcf --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd0 bne rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd1 cmp pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0xd2 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd3 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0xd4 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd5 cmp zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd6 dec zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd7 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd8 cld --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xd9 cmp aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xda nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xdb --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xdc --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0xdd cmp abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xde dec abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0xdf --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe0 cpx imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe1 sbc ptx */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,     Cpu::trap],
			/* 0xe2 --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe3 --- --- */ [Cpu::read_addr, Cpu::exec_ptx3, Cpu::exec_ptx4, Cpu::exec_ptx5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0xe4 cpx zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe5 sbc zer */ [Cpu::read_addr, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe6 inc zer */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::exec_zer5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe7 --- --- */ [Cpu::read_addr, Cpu::exec_mdf1, Cpu::exec_mdf2, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe8 inx --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xe9 sbc imm */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xea nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xeb --- --- */ [Cpu::exec_imm,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xec cpx abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xed sbc abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xee inc abs */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xef --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf0 beq rel */ [Cpu::read_data, Cpu::exec_rel3, Cpu::exec_rel4, Cpu::exec_rel5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf1 sbc pty */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5_r, Cpu::exec_rd1, Cpu::trap,      Cpu::trap,   Cpu::trap],
			/* 0xf2 --- --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf3 --- --- */ [Cpu::read_addr, Cpu::read_dat2, Cpu::exec_pty4, Cpu::exec_pty5, Cpu::exec_mdf1, Cpu::exec_ptx7, Cpu::trap,    Cpu::trap],
			/* 0xf4 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf5 sbc zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf6 inc zrx */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::exec_abs6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf7 --- --- */ [Cpu::read_addr, Cpu::exec_zrx3, Cpu::exec_mdf1, Cpu::exec_abs5, Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf8 sed --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xf9 sbc aby */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xfa nop --- */ [Cpu::exec_imp,  Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xfb --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_aby4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
			/* 0xfc --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::trap,      Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap], 
			/* 0xfd sbc abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4_r, Cpu::exec_rd1,  Cpu::trap,      Cpu::trap,      Cpu::trap,  Cpu::trap],
			/* 0xfe inc abx */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::exec_abx7, Cpu::trap,    Cpu::trap],
			/* 0xff --- --- */ [Cpu::read_addr, Cpu::exec_abs3, Cpu::exec_abx4, Cpu::exec_mdf1, Cpu::exec_abx6, Cpu::trap,      Cpu::trap,    Cpu::trap],
		];
	
		OPCODE_TABLE[self.opcode as usize][self.opcode_step as usize](self, hooks);
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
		self.opcode = (hooks.read)(self.reg_pc);
		self.increment_pc();
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
	
	
	fn incr_s(self: &mut Cpu, hooks: &CpuHooks)
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
	
	
	fn exec_rts6(self: &mut Cpu, hooks: &CpuHooks)
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
	
	
	fn exec_ptx7(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.exec_op(hooks);
	}
	
	
	fn exec_stk7(self: &mut Cpu, hooks: &CpuHooks)
	{
		self.reg_pc |= ((hooks.read)(0xffff) as u16) << 8;
		self.end_opcode();
	}
	
	
	fn exec_ptx8(self: &mut Cpu, hooks: &CpuHooks)
	{
		(hooks.write)(self.internal_addr, self.internal_data);
		self.end_opcode();
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
	
	
	fn exec_op(self: &mut Cpu, hooks: &CpuHooks)
	{
		
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