import { Cartridge } from "./cartridge.js"


export class CartridgeNROM extends Cartridge
{
	constructor(rom)
	{
		super(rom)
		
		this.prgROM = rom.prgROM
		this.chrROM = rom.chrROM
		
		if (rom.prgByteNum < 32000)
			this.cpuRead = this.cpuRead16KB
		else
			this.cpuRead = this.cpuRead32KB
		
		if (rom.chrByteNum == 0)
		{
			this.chrRAM = new Uint8Array(0x2000)
			this.ppuRead = this.ppuReadCHRRAM
			this.ppuWrite = this.ppuWriteCHRRAM
		}
		else
		{
			this.chrRAM = null
			this.ppuRead = this.ppuReadCHRROM
		}
		
		this.ppuCIRAMMirror = rom.mirroring ? this.ppuCIRAMMirrorHorz : this.ppuCIRAMMirrorVert
	}
	
	
	getBoardName()
	{
		return "NROM"
	}
	
	
	getINESMapperCode()
	{
		return 0
	}
	
	
	cpuRead16KB(addr)
	{
		return this.prgROM[addr % 0x4000]
	}
	
	
	cpuRead32KB(addr)
	{
		return this.prgROM[addr % 0x8000]
	}
	
	
	ppuReadCHRROM(addr)
	{
		return this.chrROM[addr % 0x2000]
	}
	
	
	ppuReadCHRRAM(addr)
	{
		return this.chrRAM[addr % 0x2000]
	}
	
	
	ppuWriteCHRRAM(addr, val)
	{
		if (addr < 0x2000)
			this.chrRAM[addr] = val
	}
}