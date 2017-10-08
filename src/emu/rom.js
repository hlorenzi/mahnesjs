import { CartridgeNROM } from "./cartridge_nrom.js"
import { CartridgeMMC1 } from "./cartridge_mmc1.js"
import { CartridgeUxROM } from "./cartridge_uxrom.js"
import { CartridgeMMC3 } from "./cartridge_mmc3.js"


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
		
		this.prgROM = buffer.slice(16, 16 + this.prgByteNum)
		this.chrROM = buffer.slice(16 + this.prgByteNum, 16 + this.prgByteNum + this.chrByteNum)
		
		this.mapperId = (buffer[7] & 0xf0) | ((buffer[6] & 0xf0) >> 4)
	
		this.mirroring = (buffer[6] & 0x01) == 0
		this.hasSRAM = (buffer[6] & 0x02) != 0
		this.hasTrainer = (buffer[6] & 0x04) != 0
		this.hasExtraRAM = (buffer[10] & 0x10) != 0
		this.hasBusConflicts = (buffer[10] & 0x20) != 0
		
		this.region = buffer[10] & 0x3
		
		console.log(this)
	}
	
	
	makeCartridge()
	{
		switch (this.mapperId)
		{
			case 0: return new CartridgeNROM(this)
			case 1: return new CartridgeMMC1(this)
			case 2: return new CartridgeUxROM(this)
			case 4: return new CartridgeMMC3(this)
			default: throw "unsupported mapper: 0x" + this.mapperId.toString(16)
		}
	}
}