use Cartridge;
use RomINES;


pub struct CartridgeNROM
{
	prg_rom: Vec<u8>,
	chr_rom: Vec<u8>,
	chr_ram: [u8; 0x2000],
	mirroring: bool
}


impl CartridgeNROM
{
	pub fn from_ines(ines: RomINES) -> CartridgeNROM
	{
		CartridgeNROM
		{
			prg_rom: ines.prg_rom,
			chr_rom: ines.chr_rom,
			chr_ram: [0; 0x2000],
			mirroring: ines.mirroring
		}
	}
}


impl Cartridge for CartridgeNROM
{
	fn get_board_name(&self) -> String
	{
		"NROM".to_string()
	}
	
	
	fn get_ines_mapper_code(&self) -> usize
	{
		0
	}
	
	
	fn cpu_read(&mut self, addr: u16) -> u8
	{
		if self.prg_rom.len() < 0x8000
			{ self.prg_rom[(addr & 0x3fff) as usize] }
		else
			{ self.prg_rom[(addr & 0x7fff) as usize] }
	}
	
	
	fn ppu_read(&mut self, addr: u16) -> u8
	{
		if self.chr_rom.len() == 0
			{ self.chr_ram[(addr & 0x1fff) as usize] }
		else
			{ self.chr_rom[(addr & 0x1fff) as usize] }
	}
	
	
	fn ppu_write(&mut self, addr: u16, val: u8)
	{
		if self.chr_rom.len() == 0 && addr < 0x2000
			{ self.chr_ram[addr as usize] = val; }
	}
	
	
	fn ppu_ciram_mirror(&self, addr: u16) -> bool
	{
		if self.mirroring
			{ self.ppu_ciram_mirror_horz(addr) }
		else
			{ self.ppu_ciram_mirror_vert(addr) }
	}
}