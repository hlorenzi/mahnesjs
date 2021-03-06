use Cpu;
use Ppu;
use Cartridge;


pub struct Core
{
	pub clocks: usize,
	
	pub cartridge: Box<Cartridge>,
	pub cpu: Cpu,
	pub ppu: Ppu,
	
	pub controller_shiftreg: u8,
	pub controller_strobe: u8,
	
	pub ram: [u8; 0x800],
	pub vram: [u8; 0x800],
	pub palram: [u8; 0x20],
	
	pub screen: [u8; 256 * 240 * 4],
	
	pub controller1: u8
}


impl Core
{
	pub fn new(cartridge: Box<Cartridge>) -> Box<Core>
	{
		let mut core = Box::new(Core
		{
			clocks: 0,
			
			cartridge: cartridge,
			cpu: Cpu::new(),
			ppu: Ppu::new(),
			
			controller_shiftreg: 0,
			controller_strobe: 0,
			
			ram: [0; 0x800],
			vram: [0; 0x800],
			palram: [0; 0x20],
			
			screen: [0; 256 * 240 * 4],
			
			controller1: 0
		});
		
		let core_ptr = (&mut *core) as *mut Core;
		
		core.cpu.hook_read = Box::new(move |addr| unsafe { Core::cpu_read(core_ptr, addr) });
		core.cpu.hook_write = Box::new(move |addr, val| unsafe { Core::cpu_write(core_ptr, addr, val) });
		
		core.ppu.hook_read = Box::new(move |addr| unsafe { Core::ppu_read(core_ptr, addr) });
		core.ppu.hook_write = Box::new(move |addr, val| unsafe { Core::ppu_write(core_ptr, addr, val) });
		core.ppu.hook_output_dot = Box::new(move |scanline, dot, color, mask| unsafe { Core::ppu_output_dot(core_ptr, scanline, dot, color, mask) });
		core.ppu.hook_drive_nmi = Box::new(move |active| unsafe { (*core_ptr).cpu.drive_nmi(active) });
		
		core
	}
	
	
	pub fn reset(&mut self)
	{
		self.ram = [0; 0x800];
		self.vram = [0; 0x800];
		self.palram = [0; 0x20];
		
		self.cpu.reset();
		self.ppu.reset();
		
		self.clocks = 0;
	}
	
	
	pub fn run(&mut self)
	{
		self.cpu.clock();
		self.ppu.clock();
		self.ppu.clock();
		self.ppu.clock();
		
		self.clocks = self.clocks.wrapping_add(3);
	}
	
	
	unsafe fn cpu_read(core: *mut Core, addr: u16) -> u8
	{
		let cartridge_read = (*core).cartridge.cpu_read(addr);
		
		if addr < 0x2000
		{
			(*core).ram[(addr & 0x7ff) as usize]
		}
		
		else if addr < 0x3000
		{
			match addr % 8
			{
				2 => (*core).ppu.read_reg_status(),
				4 => (*core).ppu.read_reg_oamdata(),
				7 => (*core).ppu.read_reg_data(),
				_ => 0
			}
		}
		
		else if addr == 0x4016
		{
			let bit = (*core).controller_shiftreg & 1;
			(*core).controller_shiftreg >>= 1;
			(*core).controller_shiftreg |= 0x80;
			bit
		}
		
		else
			{ cartridge_read }
	}
	
	
	unsafe fn cpu_write(core: *mut Core, addr: u16, val: u8)
	{
		(*core).cartridge.cpu_write(addr, val);
		
		if addr < 0x2000
		{
			(*core).ram[(addr & 0x7ff) as usize] = val;
		}
		
		else if addr < 0x3000
		{
			match addr % 8
			{
				0 => (*core).ppu.write_reg_ctrl(val),
				1 => (*core).ppu.write_reg_mask(val),
				3 => (*core).ppu.write_reg_oamaddr(val),
				4 => (*core).ppu.write_reg_oamdata(val),
				5 => (*core).ppu.write_reg_scroll(val),
				6 => (*core).ppu.write_reg_addr(val),
				7 => (*core).ppu.write_reg_data(val),
				_ => unreachable!()
			}
		}
		
		else if addr == 0x4014
		{
			if val != 0x40
			{
				for i in 0..256
					{ (*core).ppu.oam[i] = (*core).ram[((val as usize) << 8) + i]; }
			}
		}
		
		else if addr == 0x4016
		{
			if (val & 1) != 0
			{
				(*core).controller_strobe = 0;
				(*core).controller_shiftreg = (*core).controller1;
			}
			else
				{ (*core).controller_strobe = 1; }
		}
	}
	
	
	unsafe fn ppu_read(core: *mut Core, addr: u16) -> u8
	{
		let cartridge_read = (*core).cartridge.ppu_read(addr);
		
		if addr < 0x2000
			{ cartridge_read }
		
		else if addr < 0x3000
		{
			let mirror = if (*core).cartridge.ppu_ciram_mirror(addr | 0x8000) { 0x400 } else { 0 };
			(*core).vram[(((addr & 0x3ff) | mirror) & 0x7ff) as usize]
		}
		
		else if addr >= 0x3f00 && addr < 0x4000
			{ (*core).palram[((addr - 0x3f00) & 0x1f) as usize] }
		
		else
			{ 0 }
	}
	
	
	unsafe fn ppu_write(core: *mut Core, addr: u16, val: u8)
	{
		(*core).cartridge.ppu_write(addr, val);
		
		if addr >= 0x2000 && addr < 0x3000
		{
			let mirror = if (*core).cartridge.ppu_ciram_mirror(addr | 0x8000) { 0x400 } else { 0 };
			(*core).vram[(((addr & 0x3ff) | mirror) & 0x7ff) as usize] = val;
		}
		
		else if addr >= 0x3f00 && addr < 0x4000
		{
			if addr & 0xf == 0
			{
				for i in 0..8
					{ (*core).palram[i * 4] = val; }
			}
			else
				{ (*core).palram[((addr - 0x3f00) & 0x1f) as usize] = val; }
		}
	}
	
	
	unsafe fn ppu_output_dot(core: *mut Core, scanline: usize, dot: usize, color: u8, _mask: u8)
	{
		let screen_addr = (scanline * 256 + dot) * 4;
		let palette_addr = color as usize * 4;
		
		(*core).screen[screen_addr + 0] = PALETTE_DEFAULT[palette_addr + 0];
		(*core).screen[screen_addr + 1] = PALETTE_DEFAULT[palette_addr + 1];
		(*core).screen[screen_addr + 2] = PALETTE_DEFAULT[palette_addr + 2];
		(*core).screen[screen_addr + 3] = PALETTE_DEFAULT[palette_addr + 3];
	}
}


static PALETTE_DEFAULT: [u8; 64 * 4] =
[
	0x75, 0x75, 0x75, 0xff,
	0x27, 0x1b, 0x8f, 0xff,
	0x00, 0x00, 0xab, 0xff,
	0x47, 0x00, 0x9f, 0xff,
	0x8f, 0x00, 0x77, 0xff,
	0xab, 0x00, 0x13, 0xff,
	0xa7, 0x00, 0x00, 0xff,
	0x7f, 0x0b, 0x00, 0xff,
	0x43, 0x2f, 0x00, 0xff,
	0x00, 0x47, 0x00, 0xff,
	0x00, 0x51, 0x00, 0xff,
	0x00, 0x3f, 0x17, 0xff,
	0x1b, 0x3f, 0x5f, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,

	0xbc, 0xbc, 0xbc, 0xff,
	0x00, 0x73, 0xef, 0xff,
	0x23, 0x3b, 0xef, 0xff,
	0x83, 0x00, 0xf3, 0xff,
	0xbf, 0x00, 0xbf, 0xff,
	0xe7, 0x00, 0x5b, 0xff,
	0xdb, 0x2b, 0x00, 0xff,
	0xcb, 0x4f, 0x0f, 0xff,
	0x8b, 0x73, 0x00, 0xff,
	0x00, 0x97, 0x00, 0xff,
	0x00, 0xab, 0x00, 0xff,
	0x00, 0x93, 0x3b, 0xff,
	0x00, 0x83, 0x8b, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,

	0xff, 0xff, 0xff, 0xff,
	0x3f, 0xbf, 0xff, 0xff,
	0x5f, 0x97, 0xff, 0xff,
	0xa7, 0x8b, 0xfd, 0xff,
	0xf7, 0x7b, 0xff, 0xff,
	0xff, 0x77, 0xb7, 0xff,
	0xff, 0x77, 0x63, 0xff,
	0xff, 0x9b, 0x3b, 0xff,
	0xf3, 0xbf, 0x3f, 0xff,
	0x83, 0xd3, 0x13, 0xff,
	0x4f, 0xdf, 0x4b, 0xff,
	0x58, 0xf8, 0x98, 0xff,
	0x00, 0xeb, 0xdb, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,

	0xff, 0xff, 0xff, 0xff,
	0xab, 0xe7, 0xff, 0xff,
	0xc7, 0xd7, 0xff, 0xff,
	0xd7, 0xcb, 0xff, 0xff,
	0xff, 0xc7, 0xff, 0xff,
	0xff, 0xc7, 0xdb, 0xff,
	0xff, 0xbf, 0xb3, 0xff,
	0xff, 0xdb, 0xab, 0xff,
	0xff, 0xe7, 0xa3, 0xff,
	0xe3, 0xff, 0xa3, 0xff,
	0xab, 0xf3, 0xbf, 0xff,
	0xb3, 0xff, 0xcf, 0xff,
	0x9f, 0xff, 0xf3, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
	0x00, 0x00, 0x00, 0xff,
];