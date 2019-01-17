use Cartridge;
use CartridgeNROM;


pub struct RomINES
{
	pub prg_16kb_bank_num: usize,
	pub chr_8kb_bank_num: usize,
	pub prg_byte_num: usize,
	pub chr_byte_num: usize,
	
	pub prg_rom: Vec<u8>,
	pub chr_rom: Vec<u8>,
	
	pub mapper_id: u8,
	
	pub mirroring: bool,
	pub has_sram: bool,
	pub has_trainer: bool,
	pub has_extra_ram: bool,
	pub has_bus_conflicts: bool,
	
	pub region: usize
}


impl RomINES
{
	pub fn new(buffer: &[u8]) -> RomINES
	{
		if buffer[0] != 'N' as u8 || buffer[1] != 'E' as u8 || buffer[2] != 'S' as u8 || buffer[3] != 0x1a
			{ panic!("invalid iNES magic number"); }
			
		let prg_16kb_bank_num = buffer[4] as usize;
		let chr_8kb_bank_num = buffer[5] as usize;
		let prg_byte_num = prg_16kb_bank_num * 0x4000;
		let chr_byte_num = chr_8kb_bank_num * 0x2000;
		
		let mut prg_rom = Vec::with_capacity(prg_byte_num);
		for i in 0..prg_byte_num
			{ prg_rom.push(buffer[16 + i]); }
		
		let mut chr_rom = Vec::with_capacity(chr_byte_num);
		for i in 0..chr_byte_num
			{ chr_rom.push(buffer[16 + prg_byte_num + i]); }
			
		let mapper_id = (buffer[7] & 0xf0) | ((buffer[6] & 0xf0) >> 4);
	
		let mirroring = (buffer[6] & 0x01) == 0;
		let has_sram = (buffer[6] & 0x02) != 0;
		let has_trainer = (buffer[6] & 0x04) != 0;
		let has_extra_ram = (buffer[10] & 0x10) != 0;
		let has_bus_conflicts = (buffer[10] & 0x20) != 0;
		
		let region = buffer[10] as usize & 0x3;
		
		RomINES
		{
			prg_16kb_bank_num,
			chr_8kb_bank_num,
			prg_byte_num,
			chr_byte_num,
			
			prg_rom,
			chr_rom,
			
			mapper_id,
			
			mirroring,
			has_sram,
			has_trainer,
			has_extra_ram,
			has_bus_conflicts,
			
			region
		}
	}
	
	
	pub fn make_cartridge(self) -> Option<impl Cartridge>
	{
		match self.mapper_id
		{
			0 => Some(CartridgeNROM::from_ines(self)),
			_ => None
		}
	}
}