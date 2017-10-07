import { Cartridge } from "./cartridge.js"


export class CartridgeMMC1 extends Cartridge
{
	constructor(rom)
	{
		super(rom)
		
		this.regShift = 0x10
		this.regControl = 0x0c
		this.regCHR0 = 0
		this.regCHR1 = 0
		this.regPRG = 0
		
		this.prgAddrL = 0
		this.prgAddrH = 0
		this.chrAddrL = 0
		this.chrAddrH = 0
		
		this.prgROM = rom.prgROM
		this.chrROM = rom.chrROM
		this.prgByteNum = rom.prgByteNum
		this.chrByteNum = rom.chrByteNum
		
		this.prgRAM = new Uint8Array(0x2000)
		
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
		
		this.refreshBankAddr()
	}
	
	
	getBoardName()
	{
		return "MMC1"
	}
	
	
	getINESMapperCode()
	{
		return 1
	}
	
	
	refreshBankAddr()
	{
		switch ((this.regControl >> 2) & 0x3)
		{
			case 0:
			case 1:
				this.prgAddrL = (this.regPRG & 0xe) * 0x4000
				this.prgAddrH = this.prgAddrL + 0x4000
				break
			case 2:
				this.prgAddrL = 0
				this.prgAddrH = (this.regPRG & 0xf) * 0x4000
				break
			case 3:
				this.prgAddrL = (this.regPRG & 0xf) * 0x4000
				this.prgAddrH = this.prgByteNum - 0x4000
				break			
		}

		switch ((this.regControl >> 4) & 0x1)
		{
			case 0:
				this.chrAddrL = (this.regCHR0 & 0x1e) * 0x1000
				this.chrAddrH = this.chrAddrL + 0x1000
				break
			case 1:
				this.chrAddrL = (this.regCHR0 & 0x1f) * 0x1000
				this.chrAddrH = (this.regCHR1 & 0x1f) * 0x1000
				break
		}
	}
	
	
	cpuRead(addr)
	{
		if (addr < 0x8000)
			return this.prgRAM[addr % 0x2000]
		else if (addr < 0xc000)
			return this.prgROM[this.prgAddrL + addr % 0x4000]
		else
			return this.prgROM[this.prgAddrH + addr % 0x4000]
	}

	
	cpuWrite(addr, val)
	{
		if (addr >= 0x6000 && addr < 0x8000)
			this.prgRAM[addr % 0x2000] = val
		
		else if (addr >= 0x8000)
		{
			if ((val & 0x80) != 0)
			{
				this.regControl = this.regControl | 0x0c
				this.regShift = 0x10
				this.refreshBankAddr()
			}
			else
			{
				const full = this.regShift & 0x1
				
				this.regShift >>= 1
				this.regShift |= (val & 1) << 4

				if (full)
				{
					switch (addr & 0x6000)
					{
						case 0x0000: this.regControl = this.regShift; break
						case 0x2000: this.regCHR0 = this.regShift; break
						case 0x4000: this.regCHR1 = this.regShift; break
						case 0x6000: this.regPRG = this.regShift; break
					}

					this.regShift = 0x10
				}

				this.refreshBankAddr()
			}
		}
	}

	
	ppuReadCHRROM(addr)
	{
		if (addr < 0x1000)
			return this.chrROM[this.chrAddrL + addr % 0x1000]
		else
			return this.chrROM[this.chrAddrH + addr % 0x1000]
	}

	
	ppuReadCHRRAM(addr)
	{
		return this.chrRAM[addr % 0x2000]
	}

	
	ppuWriteCHRRAM(addr, val)
	{
		if (addr < 0x2000)
			this.chrRAM[addr % 0x2000] = val
	}
	

	ppuCIRAMEnable(addr)
	{
		return true
	}
	

	ppuCIRAMMirror(addr)
	{
		switch (this.regControl & 0x3)
		{
			case 0: return false
			case 1: return true
			case 2: return (addr & (0x1 << 10)) != 0
			case 3: return (addr & (0x1 << 11)) != 0
		}
	}
}