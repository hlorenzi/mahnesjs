import { Cartridge } from "./cartridge.js"


export class CartridgeUxROM extends Cartridge
{
	constructor(rom)
	{
		super(rom)
		
		this.regBank = 0
		
		this.prgROM = rom.prgROM
		this.chrROM = rom.chrROM
		this.bankNum = rom.prg16KBBankNum
		
		this.chrRAM = new Uint8Array(0x2000)
		
		this.ppuCIRAMMirror = rom.mirroring ? this.ppuCIRAMMirrorHorz : this.ppuCIRAMMirrorVert
	}
	
	
	getBoardName()
	{
		return "UxROM"
	}
	
	
	getINESMapperCode()
	{
		return 2
	}
	
	
	cpuRead(addr)
	{
		if (addr >= 0xc000)
			return this.prgROM[(this.bankNum - 1) * 0x4000 + addr % 0x4000]
		else
			return this.prgROM[this.regBank * 0x4000 + addr % 0x4000]
	}
	
	
	cpuWrite(addr, val)
	{
		if (addr >= 0x8000)
			this.regBank = val & 0xf
	}
	
	
	ppuRead(addr)
	{
		return this.chrRAM[addr % 0x2000]
	}
	
	
	ppuWrite(addr, val)
	{
		if (addr < 0x2000)
			this.chrRAM[addr % 0x2000] = val
	}
}