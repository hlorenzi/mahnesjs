import { CPU } from "./cpu.js"
import { ROM } from "./rom.js"


export class Core
{
	constructor()
	{
		this.rom = null
		this.cartridge = null
		
		this.cpu = new CPU()
		this.cpu.connect(
			(addr) => this.cpuRead(addr),
			(addr, val) => this.cpuWrite(addr, val))
		
		this.ram = new Uint8Array(0x800)
	}


	loadINES(buffer)
	{
		this.rom = new ROM(buffer)
		this.cartridge = this.rom.makeCartridge()
	}
	
	
	reset()
	{
		this.cpu.reset()
	}
	
	
	cpuRead(addr)
	{
		if (addr < 0x8000)
			return this.ram[addr % 0x800]
		else
			return this.cartridge.cpuRead(addr)
	}
	
	
	cpuWrite(addr, val)
	{
		if (addr < 0x8000)
			this.ram[addr % 0x800] = val
		else
			this.cartridge.cpuWrite(addr, val)
	}
}