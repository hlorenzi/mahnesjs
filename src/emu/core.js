import { CPU } from "./cpu.js"
import { PPU } from "./ppu.js"
import { ROM } from "./rom.js"


export class Core
{
	constructor()
	{
		this.rom = null
		this.cartridge = null
		
		this.output = (scanline, dot, color, mask) => { }
		this.getInput = () => [false, false, false, false, false, false, false, false]
		
		this.clock = 0
		
		this.cpu = new CPU()
		this.cpu.connect(
			(addr) => this.cpuRead(addr),
			(addr, val) => this.cpuWrite(addr, val))
			
		this.ppu = new PPU()
		this.ppu.connect(
			(scanline, dot, color, mask) => this.output(scanline, dot, color, mask),
			(addr) => this.ppuRead(addr),
			(addr, val) => this.ppuWrite(addr, val),
			(active) => this.cpu.driveNMI(active))
		
		this.ram = new Uint8Array(0x800)
		this.vram = new Uint8Array(0x800)
		this.palram = new Uint8Array(0x20)
		
		this.controllerStrobe = 0
		this.controllerInput = 0
	}
	
	
	connect(outputFn, getInputFn)
	{
		this.output = outputFn
		this.getInput = getInputFn
	}


	loadINES(buffer)
	{
		this.rom = new ROM(buffer)
		this.cartridge = this.rom.makeCartridge()
	}
	
	
	reset()
	{
		this.cpu.reset()
		this.ppu.reset()
		
		this.ram = new Uint8Array(0x800)
		this.vram = new Uint8Array(0x800)
		this.palram = new Uint8Array(0x20)
	}
	
	
	run()
	{
		this.cpu.run()
		this.ppu.run()
		this.ppu.run()
		this.ppu.run()
		this.clock += 3
	}
	
	
	cpuRead(addr)
	{
		let cartridgeRead = this.cartridge.cpuRead(addr)
		
		if (addr < 0x2000)
			return this.ram[addr % 0x800]
		
		else if (addr < 0x3000)
		{
			switch (addr % 8)
			{
				case 2: return this.ppu.readRegSTATUS()
				case 4: return this.ppu.readRegOAMDATA()
				case 7: return this.ppu.readRegDATA()
				default: return 0
			}
		}
		
		else if (addr == 0x4016)
		{
			const bit = this.controllerInput & 1
			this.controllerInput >>= 1
			this.controllerInput |= 0x80
			return bit
		}
		
		else
			return cartridgeRead
	}
	
	
	cpuWrite(addr, val)
	{
		this.cartridge.cpuWrite(addr, val)
		
		if (addr < 0x2000)
			this.ram[addr % 0x800] = val
		
		else if (addr < 0x3000)
		{
			switch (addr % 8)
			{
				case 0: this.ppu.writeRegCTRL(val); break
				case 1: this.ppu.writeRegMASK(val); break
				case 3: this.ppu.writeRegOAMADDR(val); break
				case 4: this.ppu.writeRegOAMDATA(val); break
				case 5: this.ppu.writeRegSCROLL(val); break
				case 6: this.ppu.writeRegADDR(val); break
				case 7: this.ppu.writeRegDATA(val); break
			}
		}
		
		else if (addr == 0x4014)
		{
			if (val != 0x40)
			{
				for (let i = 0; i < 256; i++)
					this.ppu.oam[i] = this.ram[(val << 8) + i]
			}
		}
		
		else if (addr == 0x4016)
		{
			if ((val & 1) != 0)
			{
				const input = this.getInput(0)
				this.controllerStrobe = 0
				this.controllerInput = (
					(input[0] ? 0x01 : 0) |
					(input[1] ? 0x02 : 0) |
					(input[2] ? 0x04 : 0) |
					(input[3] ? 0x08 : 0) |
					(input[4] ? 0x10 : 0) |
					(input[5] ? 0x20 : 0) |
					(input[6] ? 0x40 : 0) |
					(input[7] ? 0x80 : 0)
				)
			}
			else
				this.controllerStrobe = 1
		}
	}
	
	
	ppuRead(addr)
	{
		let cartridgeRead = this.cartridge.ppuRead(addr)
		
		if (addr < 0x2000)
			return cartridgeRead
		
		else if (addr < 0x3000)
		{
			let mirror = (this.cartridge.ppuCIRAMMirror(addr) ? 0x400 : 0)
			return this.vram[((addr & 0x3ff) | mirror) & 0x7ff]
		}
		
		else if (addr >= 0x3f00 && addr < 0x4000)
			return this.palram[(addr - 0x3f00) & 0x1f]
		
		else
			return 0
	}
	
	
	ppuWrite(addr, val)
	{
		this.cartridge.ppuWrite(addr, val)
		
		if (addr >= 0x2000 && addr < 0x3000)
		{
			let mirror = (this.cartridge.ppuCIRAMMirror(addr | 0x8000) ? 0x400 : 0)
			this.vram[((addr & 0x3ff) | mirror) & 0x7ff] = val
		}
		
		else if (addr >= 0x3f00 && addr < 0x4000)
		{
			if ((addr & 0xf) == 0)
			{
				for (let i = 0; i < 0x20; i += 0x4)
					this.palram[i] = val
			}
			else
				this.palram[(addr - 0x3f00) & 0x1f] = val
		}
	}
}