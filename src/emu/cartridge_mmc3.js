import { Cartridge } from "./cartridge.js"


export class CartridgeMMC3 extends Cartridge
{
	constructor(rom)
	{
		super(rom)
		
		this.regBankSelect = 0
		this.regBankData = [0, 0, 0, 0, 0, 0, 0, 0]
		this.regMirroring = false
		
		this.regIRQEnabled = false
		this.regIRQReload = false
		this.regIRQLatch = 0
		this.regIRQCounter = 0
		
		this.irqDriven = false
		
		this.prgROM = rom.prgROM
		this.prgRAM = new Uint8Array(0x2000)
		this.chrROM = rom.chrROM
		
		this.prgBankNum = rom.prgByteNum / 0x2000
		this.chrBankNum = rom.chrByteNum / 0x400
		
		this.prevPPUAddr = 0
		
		this.chrAddr = [0, 0, 0, 0, 0, 0, 0, 0]
		this.prgAddr = [0, 0, 0, 0]
		
		this.refreshBankAddr()
	}
	
	
	getBoardName()
	{
		return "MMC3"
	}
	
	
	getINESMapperCode()
	{
		return 4
	}
	
	
	refreshBankAddr()
	{
		if ((this.regBankSelect & 0x40) != 0)
		{
			this.prgAddr[0] = (this.prgBankNum - 2) * 0x2000;
			this.prgAddr[1] = (this.regBankData[7] % this.prgBankNum) * 0x2000;
			this.prgAddr[2] = (this.regBankData[6] % this.prgBankNum) * 0x2000;
			this.prgAddr[3] = (this.prgBankNum - 1) * 0x2000;
		}
		else
		{
			this.prgAddr[0] = (this.regBankData[6] % this.prgBankNum) * 0x2000;
			this.prgAddr[1] = (this.regBankData[7] % this.prgBankNum) * 0x2000;
			this.prgAddr[2] = (this.prgBankNum - 2) * 0x2000;
			this.prgAddr[3] = (this.prgBankNum - 1) * 0x2000;
		}

		if ((this.regBankSelect & 0x80) != 0)
		{
			this.chrAddr[0] = (this.regBankData[2] % this.chrBankNum) * 0x400;
			this.chrAddr[1] = (this.regBankData[3] % this.chrBankNum) * 0x400;
			this.chrAddr[2] = (this.regBankData[4] % this.chrBankNum) * 0x400;
			this.chrAddr[3] = (this.regBankData[5] % this.chrBankNum) * 0x400;
			this.chrAddr[4] = ((this.regBankData[0] & 0xfe) % this.chrBankNum) * 0x400;
			this.chrAddr[5] = ((this.regBankData[0] | 0x01) % this.chrBankNum) * 0x400;
			this.chrAddr[6] = ((this.regBankData[1] & 0xfe) % this.chrBankNum) * 0x400;
			this.chrAddr[7] = ((this.regBankData[1] | 0x01) % this.chrBankNum) * 0x400;
		}
		else
		{
			this.chrAddr[0] = ((this.regBankData[0] & 0xfe) % this.chrBankNum) * 0x400;
			this.chrAddr[1] = ((this.regBankData[0] | 0x01) % this.chrBankNum) * 0x400;
			this.chrAddr[2] = ((this.regBankData[1] & 0xfe) % this.chrBankNum) * 0x400;
			this.chrAddr[3] = ((this.regBankData[1] | 0x01) % this.chrBankNum) * 0x400;
			this.chrAddr[4] = (this.regBankData[2] % this.chrBankNum) * 0x400;
			this.chrAddr[5] = (this.regBankData[3] % this.chrBankNum) * 0x400;
			this.chrAddr[6] = (this.regBankData[4] % this.chrBankNum) * 0x400;
			this.chrAddr[7] = (this.regBankData[5] % this.chrBankNum) * 0x400;
		}
	}
	
	
	handleIRQCounter(addr)
	{
		if (addr >= 0x3000)
			return
		
		if ((addr & 0x1000) != 0 && (this.prevPPUAddr & 0x1000) == 0)
		{
			this.regIRQCounter -= 1

			if (this.regIRQReload)
				this.regIRQCounter = this.regIRQLatch

			this.regIRQReload = false

			if (this.regIRQCounter == 0 && this.regIRQEnabled)
			{
				this.irqDriven = true
				this.regIRQCounter = this.regIRQLatch
			}
		}
		
		this.prevPPUAddr = addr
	}
	
	
	driveIRQ()
	{
		const driven = this.irqDriven
		this.irqDriven = false
		return driven
	}

	
	cpuRead(addr)
	{
		if (addr < 0x8000)
			return this.prgRAM[addr % 0x2000]
		else
			return this.prgROM[this.prgAddr[(addr >> 13) & 0x3] + addr % 0x2000]
	}
	
	
	cpuWrite(addr, val)
	{
		// PRG-RAM write
		if (addr >= 0x6000 && addr < 0x8000)
			this.prgRAM[addr % 0x2000] = val
		
		// Bank Select/Bank Data
		else if ((addr & 0xe000) == 0x8000)
		{
			if ((addr & 0x1) == 0)
				this.regBankSelect = val
			else
			{
				this.regBankData[this.regBankSelect & 0x7] = val
				this.refreshBankAddr()
			}
		}
		
		// Mirroring
		else if ((addr & 0xe000) == 0xa000)
		{
			if ((addr & 0x1) == 0)
				this.regMirroring = ((val & 0x1) != 0)
		}
		
		// IRQ Latch
		else if ((addr & 0xe000) == 0xc000)
		{
			if ((addr & 0x1) == 0)
				this.regIRQLatch = val
			else
				this.regIRQReload = true
		}
		
		// IRQ Enable
		else if ((addr & 0xe000) == 0xe000)
			this.regIRQEnabled = ((addr & 0x1) != 0)
	}

	
	ppuRead(addr)
	{
		this.handleIRQCounter(addr)

		return this.chrROM[this.chrAddr[(addr >> 10) & 0x7] + addr % 0x400]
	}

	
	ppuWrite(addr, val)
	{
		this.handleIRQCounter(addr)
	}

	
	ppuCIRAMMirror(addr)
	{
		if (this.regMirroring)
			return (addr & (1 << 11)) != 0
		else
			return (addr & (1 << 10)) != 0
	}
}