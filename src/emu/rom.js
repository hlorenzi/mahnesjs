import { CartridgeNROM } from "./cartridge_nrom.js"


export class ROM
{
	constructor(buffer)
	{
		if (buffer[0] != "N".charCodeAt(0) ||
			buffer[1] != "E".charCodeAt(0) ||
			buffer[2] != "S".charCodeAt(0) ||
			buffer[3] != 0x1a)
			throw "invalid iNES magic number"
		
		this.prg16KBBankNum = buffer[4]
		this.chr8KBBankNum = buffer[5]
		this.prgByteNum = this.prg16KBBankNum * 0x4000
		this.chrByteNum = this.chr8KBBankNum * 0x2000
		
		this.prgROM = buffer.slice(16, buffer.length - 16)
		this.chrROM = buffer.slice(16 + this.prgByteNum, buffer.length - 16 - this.prgByteNum)
		
		this.mapperId = (buffer[7] & 0xf0) | ((buffer[6] & 0xf0) >> 4)
	
		this.mirroring = (buffer[6] & 0x01) == 0
		this.hasSRAM = (buffer[6] & 0x02) != 0
		this.hasTrainer = (buffer[6] & 0x04) != 0
		this.hasExtraRAM = (buffer[10] & 0x10) != 0
		this.hasBusConflicts = (buffer[10] & 0x20) != 0
		
		this.region = buffer[10] & 0x3
	}
	
	
	makeCartridge()
	{
		switch (this.mapperId)
		{
			case 0:
				return new CartridgeNROM(this)
				
			default:
				throw "unsupported mapper: 0x" + this.mapperId.toString(16)
		}
	}
}