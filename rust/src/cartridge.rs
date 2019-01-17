pub trait Cartridge
{
	fn get_board_name(&self) -> String
	{
		"???".to_string()
	}
	
	
	fn get_ines_mapper_code(&self) -> usize
	{
		0
	}
	
	
	fn cpu_read(&mut self, _addr: u16) -> u8
	{
		0
	}
	
	
	fn cpu_write(&mut self, _addr: u16, _val: u8)
	{
	
	}
	
	
	fn ppu_read(&mut self, _addr: u16) -> u8
	{
		0
	}
	
	
	fn ppu_write(&mut self, _addr: u16, _val: u8)
	{
	
	}
	
	
	fn ppu_ciram_enable(&self, _addr: u16) -> bool
	{
		false
	}
	
	
	fn ppu_ciram_mirror(&self, _addr: u16) -> bool
	{
		false
	}


	fn ppu_ciram_mirror_horz(&self, addr: u16) -> bool
	{
		(addr & (1 << 11)) != 0
	}


	fn ppu_ciram_mirror_vert(&self, addr: u16) -> bool
	{
		(addr & (1 << 10)) != 0
	}
}