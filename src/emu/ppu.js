const FLAG_VBLANK = 0b10000000


export class PPU
{
	constructor()
	{
		this.scanline = 0
		this.dot = 0
		this.frame = 0
	
		this.regCTRL = 0
		this.regMASK = 0
		this.regSTATUS = 0
		
		this.scrollV = 0
		this.scrollT = 0
		this.scrollX = 0
		
		this.addressNibble = false
		this.internalLatch = 0
		
		this.oam = new Uint8Array(0x100)
		this.oamAddress = 0
		
		this.internalPatternL = 0
		this.internalPatternH = 0
		this.internalPalette = 0
		
		this.internalCandidateObjects = []
		for (let i = 0; i < 8; i++)
		{
			this.internalCandidateObjects.push({
				id: -1,
				patternAddr: 0,
				x: 0,
				y: 0,
				paletteIndex: 0,
				priority: 0,
				patternL: 0,
				patternH: 0
			})
		}
		
		this.output = (scanline, dot, bkgPal, mask) => { }
		this.read = (addr) => 0
		this.write = (addr, val) => { }
		this.driveNMI = () => { }
	}
	
	
	connect(outputFn, readFn, writeFn, driveNMIFn)
	{
		this.output = outputFn
		this.read = readFn
		this.write = writeFn
		this.driveNMI = driveNMIFn
	}
	
	
	reset()
	{
		this.scanline = 240
		this.dot = 0
		this.frame = 0
	
		this.regCTRL = 0
		this.regMASK = 0
		this.regSTATUS = 0
		
		this.scrollV = 0
		this.scrollT = 0
		this.scrollX = 0
		
		this.addressNibble = false
		this.internalLatch = 0
		
		this.oamAddress = 0
	}
	
	
	writeRegCTRL(val)
	{
		this.regCTRL = val
		this.scrollT &= ~(0x3 << 10)
		this.scrollT |= (val & 0x3) << 10
	}
	
	
	writeRegMASK(val)
	{
		this.regMASK = val
	}
	
	
	writeRegSCROLL(val)
	{
		if (!this.addressNibble)
		{
			this.scrollX = val & 0x7;

			this.scrollT &= ~0x1f;
			this.scrollT |= (val >> 3) & 0x1f;
		}
		else
		{
			this.scrollT &= ~(0x7 << 12);
			this.scrollT |= (val & 0x7) << 12;

			this.scrollT &= ~(0xf8 >> 3 << 5);
			this.scrollT |= (val & 0xf8) >> 3 << 5;
		}
		
		this.addressNibble = !this.addressNibble;
	}
	
	
	writeRegADDR(val)
	{
		if (!this.addressNibble)
		{
			this.scrollT &= ~(0x1 << 14);
			this.scrollT &= ~(0x3f << 8);
			this.scrollT |= (val & 0x3f) << 8;
		}
		else
		{
			this.scrollT &= ~0xff;
			this.scrollT |= val & 0xff;
			this.scrollV = this.scrollT;
		}
		
		this.addressNibble = !this.addressNibble;
	}
	
	
	writeRegDATA(val)
	{
		this.write(this.scrollV, val)
		this.scrollV += ((this.regCTRL & 0x04) == 0 ? 1 : 32)
		this.scrollV &= 0xffff
	}
	
	
	writeRegOAMADDR(val)
	{
		this.oamAddress = val
	}
	
	
	writeRegOAMDATA(val)
	{
		this.oam[this.oamAddress] = val
		this.oamAddress = (this.oamAddress + 1) & 0xff
	}
	
	
	readRegSTATUS()
	{
		this.addressNibble = false
		let val = this.regSTATUS
		this.regSTATUS &= 0x7f
		return val
	}
	
	
	readRegDATA()
	{
		let val = this.internalLatch
		
		if (this.scrollV >= 0x3f00 && this.scrollV < 0x4000)
		{
			this.internalLatch = this.read(this.scrollV - 0x1000)
			let val = this.read(this.scrollV)
			if ((this.regMASK & 1) != 0)
				val &= 0x30
		}
		else
			this.internalLatch = this.read(this.scrollV)
		
		this.scrollV += ((this.regCTRL & 0x04) != 0) ? 32 : 1
		return val
	}
	
	
	readRegOAMDATA()
	{
		return this.oam[this.oamAddress] & (((this.oamAddress & 0x3) == 0x2) ? 0xe3 : 0xff)
	}
	
	
	run()
	{
		if (this.scanline >= 0 && this.scanline < 240)
			this.runVisibleScanline()
		
		else if (this.scanline == 241)
		{
			if (this.dot == 0)
				this.regSTATUS |= FLAG_VBLANK
		}
		
		else if (this.scanline == 261)
		{
			if (this.dot == 1)
			{
				this.regSTATUS &= ~FLAG_VBLANK
				this.regSTATUS &= 0x1f
			}
			
			else if (this.dot >= 280 && this.dot < 305)
			{
				if (this.regMASK & 0x18)
				{
					this.scrollV &= ~0x7be0
					this.scrollV |= this.scrollT & 0x7be0
				}
			}
		}
		
		this.driveNMI((this.regCTRL & 0x80) != 0 && (this.regSTATUS & 0x80) != 0)
		
		this.dot += 1
		if (this.dot == 341)
		{
			this.dot = 0
			this.scanline += 1
			if (this.scanline == 262)
			{
				this.scanline = 0
				this.frame += 1
			}
		}
	}
	
	
	runVisibleScanline()
	{
		if (this.dot < 256)
		{
			if ((this.regMASK & 0x18) == 0)
			{
				let bkgPixelColor
				if (this.scrollV >= 0x3f00 && this.scrollV < 0x4000)
					bkgPixelColor = 0x3f & this.read(this.scrollV)
				else
					bkgPixelColor = 0x3f & this.read(0x3f00)
				
				this.output(this.scanline, this.dot, bkgPixelColor, this.regMASK)
			}
			else
			{
				const dotIntoTile = (this.scrollX + this.dot) % 8
				
				if (this.dot == 0 || dotIntoTile == 0)
				{
					let pal = this.read(0x23c0 | (this.scrollV & 0xc00) | ((this.scrollV >> 4) & 0x38) | ((this.scrollV >> 2) & 0x07))
					pal = (pal >> ((this.scrollV & 0x2) | ((this.scrollV >> 4) & 0x4))) & 0x3
					
					let patternIndex = this.read(0x2000 | (this.scrollV & 0xfff))
					let patternAddr = ((this.regCTRL & 0x10) << 8) | (patternIndex << 4) | (this.scrollV >> 12)
					
					this.internalPalette = pal
					this.internalPatternL = this.read(patternAddr)
					this.internalPatternH = this.read(patternAddr + 8)
				}
				
				let bkgPixel = ((this.internalPatternL & (0x80 >> dotIntoTile)) != 0 ? 1 : 0) | ((this.internalPatternH & (0x80 >> dotIntoTile)) != 0 ? 2 : 0)
				let bkgPixelColor = 0x3f & this.read(0x3f00 | (this.internalPalette << 2) | bkgPixel)
				
				if ((this.regMASK & 1) != 0)
					bkgPixelColor &= 0x30
				
				this.blendBkgWithSprToOutput(bkgPixel, bkgPixelColor)
				
				if (dotIntoTile == 7)
				{
					if ((this.scrollV & 0x1f) == 0x1f)
					{
						this.scrollV &= ~0x1f
						this.scrollV ^= 0x400
					}
					else
						this.scrollV = (this.scrollV + 1) & 0xffff
				}
			}
		}
		
		else if (this.dot == 256)
		{
			if ((this.regMASK & 0x10) != 0)
				this.runSpriteFetch()
		}
		
		else if (this.dot == 257)
		{			
			if ((this.regMASK & 0x18) != 0)
			{
				if ((this.scrollV & 0x7000) != 0x7000)
					this.scrollV = (this.scrollV + 0x1000) & 0xffff
				else
				{
					this.scrollV &= ~0x7000
					
					let y = (this.scrollV & 0x3e0) >> 5
					if (y == 29)
					{
						y = 0
						this.scrollV ^= 0x800
					}
					else if (y == 31)
						y = 0
					else
						y += 1

					this.scrollV = (this.scrollV & ~0x3e0) | (y << 5)
				}
			}
		}
		
		else if (this.dot == 258)
		{
			if ((this.regMASK & 0x18) != 0)
			{
				this.scrollV &= ~0x41f
				this.scrollV |= this.scrollT & 0x41f
			}
		}
	}
	
	
	blendBkgWithSprToOutput(bkgPixel, bkgPixelColor)
	{
		if ((this.regMASK & 0x10) != 0 && ((this.regMASK & 0x4) != 0 || this.dot >= 8))
		{
			for (let i = 0; i < 8; i++)
			{
				if (this.internalCandidateObjects[i].id < 0)
					break
				
				const id = this.internalCandidateObjects[i].id
				const sprX = this.internalCandidateObjects[i].x
				const sprPalette = this.internalCandidateObjects[i].paletteIndex
				const flipH = ((this.internalCandidateObjects[i].priority & 0x2) != 0)
				const priority = ((this.internalCandidateObjects[i].priority & 0x1) != 0)
				const patternL = this.internalCandidateObjects[i].patternL
				const patternH = this.internalCandidateObjects[i].patternH
				
				const dotIntoSpr = this.dot - sprX
				if (dotIntoSpr < 0 || dotIntoSpr >= 8)
					continue
				
				let sprPixel
				if (flipH)
					sprPixel = (((patternL >> dotIntoSpr) & 1) != 0 ? 1 : 0) | (((patternH >> dotIntoSpr) & 1) != 0 ? 2 : 0)
				else
					sprPixel = (((patternL << dotIntoSpr) & 0x80) != 0 ? 1 : 0) | (((patternH << dotIntoSpr) & 0x80) != 0 ? 2 : 0)
			
				if (sprPixel == 0)
					continue
				
				if (id == 0 && bkgPixel != 0)
					this.regSTATUS |= 0x40
				
				if (priority && bkgPixel != 0)
					break
				
				let sprPixelColor = 0x3f & this.read(0x3f10 | (sprPalette << 2) | sprPixel)
				if ((this.regMASK & 1) != 0)
					sprPixelColor &= 0x30
				
				this.output(this.scanline, this.dot, sprPixelColor, this.regMASK)
				return
			}
		}
		
		this.output(this.scanline, this.dot, bkgPixelColor, this.regMASK)
	}
	
	
	runSpriteFetch()
	{
		for (let i = 0; i < 8; i++)
			this.internalCandidateObjects[i].id = -1
		
		const sprHeight = ((this.regCTRL & 0x20) != 0 ? 16 : 8)
		const defaultPatternTable = ((this.regCTRL & 0x8) != 0 ? 0x1000 : 0)
		
		let candidateSlot = 0
		for (let i = 0; i < 64 && candidateSlot < 8; i++)
		{
			const oamAddr      = i * 4
			const sprY         = this.oam[oamAddr]
			const sprTile      = this.oam[oamAddr + 1]
			const sprAttribute = this.oam[oamAddr + 2]
			const sprX         = this.oam[oamAddr + 3]
			
			const scanlineIntoSpr = this.scanline - sprY
			if (scanlineIntoSpr < 0 || scanlineIntoSpr >= sprHeight)
			{
				if (sprHeight == 16)
				{
					this.read(0x1000)
					this.read(0x1000)
				}
				else
				{
					this.read(defaultPatternTable)
					this.read(defaultPatternTable)
				}
				continue
			}
			
			const paletteIndex = sprAttribute & 0x3
			const priority = (sprAttribute & 0x20) != 0
			const flipH = (sprAttribute & 0x40) != 0
			const flipV = (sprAttribute & 0x80) != 0
			
			let finalPatternTable
			let finalPatternIndex
			let finalPatternRow
			if (sprHeight == 16)
			{
				finalPatternTable = ((sprTile & 1) != 0 ? 0x1000 : 0)
				finalPatternIndex = sprTile & 0xfe
				
				if (flipV)
				{
					if (scanlineIntoSpr >= 8)
						finalPatternRow = 15 - scanlineIntoSpr
					else
					{
						finalPatternIndex += 1
						finalPatternRow = 7 - scanlineIntoSpr
					}
				}
				else
				{
					if (scanlineIntoSpr >= 8)
					{
						finalPatternIndex += 1
						finalPatternRow = scanlineIntoSpr - 8
					}
					else
						finalPatternRow = scanlineIntoSpr
				}
			}
			else
			{
				finalPatternTable = defaultPatternTable
				finalPatternIndex = sprTile
				
				if (flipV)
					finalPatternRow = 7 - scanlineIntoSpr
				else
					finalPatternRow = scanlineIntoSpr
			}
			
			const finalPatternAddr = finalPatternTable + (finalPatternIndex << 4) + finalPatternRow
			
			this.internalCandidateObjects[candidateSlot].id = i
			this.internalCandidateObjects[candidateSlot].patternAddr = finalPatternAddr
			this.internalCandidateObjects[candidateSlot].x = sprX
			this.internalCandidateObjects[candidateSlot].y = sprY
			this.internalCandidateObjects[candidateSlot].paletteIndex = paletteIndex
			this.internalCandidateObjects[candidateSlot].priority = priority | (flipH ? 2 : 0)
			this.internalCandidateObjects[candidateSlot].patternL = this.read(finalPatternAddr)
			this.internalCandidateObjects[candidateSlot].patternH = this.read(finalPatternAddr + 8)
			candidateSlot += 1
		}
	}
}