export class Cartridge
{
	constructor(rom)
	{
		
	}
	
	
	getBoardName()
	{
		return null
	}
	
	
	getINESMapperCode()
	{
		return null
	}
	
	
	cpuRead(addr)
	{
		return 0
	}
	
	
	cpuWrite(addr, val)
	{
		// Do nothing
	}
	
	
	ppuRead(addr)
	{
		return 0
	}
	
	
	ppuWrite(addr, val)
	{
		// Do nothing
	}
	
	
	driveIRQ()
	{
		return false
	}
	
	
	ppuCIRAMEnable(addr)
	{
		return true
	}
	
	
	ppuCIRAMMirror(addr)
	{
		return false
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