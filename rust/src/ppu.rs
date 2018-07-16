type PpuReadFn = Fn(u16) -> u8;
type PpuWriteFn = Fn(u16, u8);
type PpuOutputDotFn = Fn(usize, usize, u8, u8);
type PpuDriveNmiFn = Fn(bool);


static FLAG_VBLANK: u8 = 0b10000000;


pub struct Ppu
{
	scanline: usize,
	dot: usize,
	frame: usize,
	
	reg_ctrl: u8,
	reg_mask: u8,
	reg_status: u8,
	
	scroll_v: u16,
	scroll_t: u16,
	scroll_x: u8,
	
	address_nibble: bool,
	internal_latch: u8,
	
	oam: [u8; 0x100],
	oam_address: u8,
	
	internal_pattern_lo: u8,
	internal_pattern_hi: u8,
	internal_palette: u8
}


pub struct PpuHooks<'a>
{
	pub read: &'a PpuReadFn,
	pub write: &'a PpuWriteFn,
	
	pub output_dot: &'a PpuOutputDotFn,
	pub drive_nmi: &'a PpuDriveNmiFn
}



impl Ppu
{
	pub fn new() -> Ppu
	{
		Ppu
		{
			scanline: 240,
			dot: 0,
			frame: 0,
			
			reg_ctrl: 0,
			reg_mask: 0,
			reg_status: 0,
			
			scroll_v: 0,
			scroll_t: 0,
			scroll_x: 0,
			
			address_nibble: false,
			internal_latch: 0,
			
			oam: [0; 0x100],
			oam_address: 0,
			
			internal_pattern_lo: 0,
			internal_pattern_hi: 0,
			internal_palette: 0
		}
	}
	
	
	pub fn reset(&mut self)
	{
		self.scanline = 240;
		self.dot = 0;
		self.frame = 0;
		
		self.reg_ctrl = 0;
		self.reg_mask = 0;
		self.reg_status = 0;
		
		self.scroll_v = 0;
		self.scroll_t = 0;
		self.scroll_x = 0;
		
		self.address_nibble = false;
		self.internal_latch = 0;
		
		self.oam = [0; 0x100];
		self.oam_address = 0;
		
		self.internal_pattern_lo = 0;
		self.internal_pattern_hi = 0;
		self.internal_palette = 0;
	}
	
	
	pub fn write_reg_ctrl(&mut self, val: u8)
	{
		self.reg_ctrl = val;
		self.scroll_t &= !(0x3 << 10);
		self.scroll_t |= (val as u16 & 0x3) << 10;
	}
	
	
	pub fn write_reg_mask(&mut self, val: u8)
	{
		self.reg_mask = val;
	}
	
	
	pub fn write_reg_scroll(&mut self, val: u8)
	{
		if self.address_nibble
		{
			self.scroll_x = val & 0x7;
			
			self.scroll_t &= !0x1f;
			self.scroll_t |= (val as u16 >> 3) & 0x1f;
		}
		else
		{
			self.scroll_t &= !(0x7 << 12);
			self.scroll_t |= (val as u16 & 0x7) << 12;

			self.scroll_t &= !(0xf8 >> 3 << 5);
			self.scroll_t |= (val as u16 & 0xf8) >> 3 << 5;
		}
		
		self.address_nibble = !self.address_nibble;
	}
	
	
	pub fn write_reg_addr(&mut self, val: u8)
	{
		if !self.address_nibble
		{
			self.scroll_t &= !(0x1 << 14);
			self.scroll_t &= !(0x3f << 8);
			self.scroll_t |= (val as u16 & 0x3f) << 8;
		}
		else
		{
			self.scroll_t &= !0xff;
			self.scroll_t |= val as u16 & 0xff;
			self.scroll_v = self.scroll_t;
		}
		
		self.address_nibble = !self.address_nibble;
	}
	
	
	pub fn write_reg_data(&mut self, hooks: &PpuHooks, val: u8)
	{
		(hooks.write)(self.scroll_v, val);
		self.scroll_v += if (self.reg_ctrl & 0x04) == 0 { 1 } else { 32 };
		self.scroll_v &= 0xffff;
	}
	
	
	pub fn write_reg_oamaddr(&mut self, val: u8)
	{
		self.oam_address = val;
	}
	
	
	pub fn write_reg_oamdata(&mut self, val: u8)
	{
		self.oam[self.oam_address as usize] = val;
		self.oam_address = self.oam_address.wrapping_add(1);
	}
	
	
	pub fn read_reg_status(&mut self) -> u8
	{
		let val = self.reg_status;
		
		self.address_nibble = false;
		self.reg_status &= 0x7f;
		
		val
	}
	
	
	pub fn read_reg_data(&mut self, hooks: &PpuHooks) -> u8
	{
		let mut val = self.internal_latch;
		
		if self.scroll_v >= 0x3f00 && self.scroll_v < 0x4000
		{
			self.internal_latch = (hooks.read)(self.scroll_v - 0x1000);
			
			val = (hooks.read)(self.scroll_v);
			if (self.reg_mask & 1) != 0
				{ val &= 0x30; }
		}
		else
		{
			self.internal_latch = (hooks.read)(self.scroll_v);
		}
		
		self.scroll_v += if (self.reg_ctrl & 0x04) != 0 { 32 } else { 1 };
		val
	}
	
	
	pub fn read_reg_oamdata(&mut self) -> u8
	{
		self.oam[self.oam_address as usize] & (if (self.oam_address & 0x3) == 0x2 { 0xe3 } else { 0xff })
	}
	
	
	pub fn run(&mut self, hooks: &PpuHooks)
	{
		if self.scanline >= 0 && self.scanline < 240
			{ self.run_visible_scanline(hooks); }
			
		else if self.scanline == 241
		{
			if self.dot == 0
				{ self.reg_status |= FLAG_VBLANK; }
		}
		
		else if self.scanline == 261
		{
			if self.dot == 1
			{
				self.reg_status &= !FLAG_VBLANK;
				self.reg_status &= 0x1f;
			}
			
			else if self.dot >= 280 && self.dot < 305
			{
				if (self.reg_mask & 0x18) != 0
				{
					self.scroll_v &= !0x7be0;
					self.scroll_v |= (self.scroll_t & 0x7be0);
				}
			}
		}
		
		(hooks.drive_nmi)((self.reg_ctrl & 0x80) != 0 && (self.reg_status & 0x80) != 0);
		
		self.dot += 1;
		if self.dot == 341 || (self.dot == 340 && self.scanline == 261 && self.frame % 2 != 0)
		{
			self.dot = 0;
			self.scanline += 1;
			if self.scanline == 262
			{
				self.scanline = 0;
				self.frame += 1;
			}
		}
	}
	
	
	pub fn run_visible_scanline(&mut self, hooks: &PpuHooks)
	{
		if self.dot < 256
		{
			if (self.reg_mask & 0x18) == 0
			{
				let bkg_pixel_color = if self.scroll_v >= 0x3f00 && self.scroll_v < 0x4000
					{ 0x3f & (hooks.read)(self.scroll_v) }
				else
					{ 0x3f & (hooks.read)(0x3f00) };
					
				(hooks.output_dot)(self.scanline, self.dot, bkg_pixel_color, self.reg_mask);
			}
			
			else
			{
				let dot_into_tile = self.scroll_x.wrapping_add(self.dot) % 8;
				
				if self.dot == 0 || dot_into_tile == 0
				{
					let pal = ((hooks.read)(0x23c0 | (self.scroll_v & 0xc00) | ((self.scroll_v >> 4) & 0x38) | ((self.scroll_v >> 2) & 0x7))
						>> ((self.scroll_v & 0x2) | ((self.scroll_v >> 4) & 0x4)))
						& 0x3;
					
					let pattern_index = (hooks.read)(0x2000 | (self.scroll_v & 0xfff));
					let pattern_addr = ((self.reg_ctrl & 0x10) << 8) | (pattern_index << 4) | (self.scroll_v >> 12);
					
					self.internal_palette = pal;
					self.internal_pattern_lo = (hooks.read)(pattern_addr);
					self.internal_pattern_hi = (hooks.read)(pattern_addr + 8);
				}
				
				//let bkg_pixel = 
			}
		}
	}
}