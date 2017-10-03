export class CartridgeNROM
{
	constructor(rom)
	{
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
			this.ppuWrite = this.ppuWriteCHRROM
		}
		
		if (rom.mirroring)
			this.ppuCIRAMMirror = this.ppuCIRAMMirrorHorz
		else
			this.ppuCIRAMMirror = this.ppuCIRAMMirrorVert
	}
	
	
	cpuRead16KB(addr)
	{
		return this.prgROM[addr % 0x4000]
	}
	
	
	cpuRead32KB(addr)
	{
		return this.prgROM[addr % 0x8000]
	}
	
	
	cpuWrite(addr, val)
	{
		// Do nothing
	}
	
	
	ppuReadCHRROM(addr)
	{
		return this.chrROM[addr % 0x2000]
	}
	
	
	ppuReadCHRRAM(addr)
	{
		return this.chrRAM[addr % 0x2000]
	}
	
	
	ppuWriteCHRROM(addr, val)
	{
		// Do nothing
	}
	
	
	ppuWriteCHRRAM(addr, val)
	{
		if (addr < 0x2000)
			this.chrRAM[addr] = val
	}
	
	
	ppuCIRAMEnable(addr)
	{
		return true
	}
	
	
	ppuCIRAMMirrorHorz(addr)
	{
		return (addr & (1 << 11)) != 0
	}
	
	
	ppuCIRAMMirrorVert(addr)
	{
		return (addr & (1 << 10)) != 0
	}
}