import * as opcode from "./cpu_opcodes.js"

// Processor status flags
const FLAG_C = 0b00000001 // Carry
const FLAG_Z = 0b00000010 // Zero
const FLAG_I = 0b00000100 // Interrupt
const FLAG_D = 0b00001000 // Decimal
const FLAG_B = 0b00010000 // Break
const FLAG_U = 0b00100000 // Unused
const FLAG_V = 0b01000000 // Overflow
const FLAG_N = 0b10000000 // Negative

// Opcode adressing modes
const IMP = 0  // Implied
const IMM = 1  // Immediate
const ZER = 2  // Zero-page
const ZRX = 3  // Zero-page, X
const ZRY = 4  // Zero-page, Y
const ABS = 5  // Absolute
const ABX = 6  // Absolute, X
const ABY = 7  // Absolute, Y
const PTX = 8  // (Indirect, X)
const PTY = 9  // (Indirect), Y
const REL = 10 // Relative
const IND = 11 // Indirect
const STK = 12 // Stack

// Opcode memory function modes
const READ = 0 // Read
const MDFY = 1 // Read-modify-write
const WRIT = 2 // Write
const NONE = 3 // Other


export class CPU
{
	constructor()
	{
		this.read = (addr) => 0
		this.write = (addr, val) => { }
		
		this.hookExecuteInstruction = (addr, byte1, byte2, byte3) => { }
		
		this.signalNMI = false
		this.acknowledgeNMI = false
		this.signalIRQ = false
		
		this.opcode = 0
		this.opcodeStep = 0
		
		this.resetRoutine = false
		this.nmiRoutine = false
		
		this.regPC = 0
		this.regA = 0
		this.regX = 0
		this.regY = 0
		this.regS = 0
		
		this.flagC = false
		this.flagZ = false
		this.flagI = false
		this.flagD = false
		this.flagB = false
		this.flagV = false
		this.flagN = false
		
		this.internalAddr = 0
		this.internalData = 0
		
		this.opcodeAddressModes =
		[
			//0   1   2   3    4   5   6   7    8   9   a   b    c   d   e   f
			STK,PTX,IMM,PTX, ZER,ZER,ZER,ZER, STK,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // 0
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX, // 1
			STK,PTX,IMM,PTX, ZER,ZER,ZER,ZER, STK,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // 2
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX, // 3

			STK,PTX,IMM,PTX, ZER,ZER,ZER,ZER, STK,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // 4
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX, // 5
			STK,PTX,IMM,PTX, ZER,ZER,ZER,ZER, STK,IMM,IMP,IMM, IND,ABS,ABS,ABS, // 6
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX, // 7

			IMP,PTX,IMM,PTX, ZER,ZER,ZER,ZER, IMP,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // 8
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRY,ZRY, IMP,ABY,IMP,ABY, ABX,ABX,ABY,ABY, // 9
			IMM,PTX,IMM,PTX, ZER,ZER,ZER,ZER, IMP,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // a
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRY,ZRY, IMP,ABY,IMP,ABY, ABX,ABX,ABY,ABY, // b

			IMM,PTX,IMM,PTX, ZER,ZER,ZER,ZER, IMP,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // c
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX, // d
			IMM,PTX,IMM,PTX, ZER,ZER,ZER,ZER, IMP,IMM,IMP,IMM, ABS,ABS,ABS,ABS, // e
			REL,PTY,IMP,PTY, ZRX,ZRX,ZRX,ZRX, IMP,ABY,IMP,ABY, ABX,ABX,ABX,ABX  // f
			//0   1   2   3    4   5   6   7    8   9   a   b    c   d   e   f
		]
		
		this.opcodeFunctionModes =
		[
			// 0    1    2    3     4    5    6    7     8    9    a    b     c    d    e    f
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,MDFY,NONE, READ,READ,MDFY,MDFY, // 0
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, // 1
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,MDFY,NONE, READ,READ,MDFY,MDFY, // 2
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, // 3

			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,MDFY,NONE, NONE,READ,MDFY,MDFY, // 4
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, // 5
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,MDFY,NONE, NONE,READ,MDFY,MDFY, // 6
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, // 7

			READ,WRIT,NONE,WRIT, WRIT,WRIT,WRIT,WRIT, NONE,READ,NONE,NONE, WRIT,WRIT,WRIT,WRIT, // 8
			NONE,WRIT,NONE,NONE, WRIT,WRIT,WRIT,WRIT, NONE,WRIT,NONE,NONE, NONE,WRIT,NONE,NONE, // 9
			READ,READ,READ,NONE, READ,READ,READ,NONE, NONE,READ,NONE,NONE, READ,READ,READ,READ, // a
			NONE,READ,NONE,NONE, READ,READ,READ,NONE, NONE,READ,NONE,NONE, READ,READ,READ,NONE, // b

			READ,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,NONE, READ,READ,MDFY,MDFY, // c
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, // d
			READ,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,READ, READ,READ,MDFY,MDFY, // e
			NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY, NONE,READ,NONE,MDFY, READ,READ,MDFY,MDFY  // f
			// 0    1    2    3     4    5    6    7     8    9    a    b     c    d    e    f
		]
	}
	
	
	connect(readFn, writeFn)
	{
		this.read = readFn
		this.write = writeFn
	}
	
	
	reset()
	{
		this.resetRoutine = true
		this.opcodeStep = 0
		
		this.regA = 0
		this.regX = 0
		this.regY = 0
		this.regS = 0xfd
		
		this.flagC = false
		this.flagZ = false
		this.flagI = false
		this.flagD = false
		this.flagB = false
		this.flagV = false
		this.flagN = false
		
		this.internalAddr = 0
		this.internalData = 0
	}
	
	
	driveNMI(active)
	{
		if (!this.signalNMI && active)
			this.acknowledgeNMI = true
		
		this.signalNMI = active
	}
	
	
	driveIRQ()
	{
		this.signalIRQ = true
	}
	
	
	run()
	{
		this.opcodeStep += 1
		
		if (this.resetRoutine)
			this.runReset()
		
		else if (this.nmiRoutine)
			this.runNMI()
		
		else switch (this.opcodeStep)
		{
			case 1:
				this.runOpcodeStep1()
				break
			case 2:
				this.runOpcodeStep2()
				break
			case 3:
				this.runOpcodeStep3()
				break
			case 4:
				this.runOpcodeStep4()
				break
			case 5:
				this.runOpcodeStep5()
				break
			case 6:
				this.runOpcodeStep6()
				break
			case 7:
				this.runOpcodeStep7()
				break
			case 8:
				this.runOpcodeStep8()
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runReset()
	{
		switch (this.opcodeStep)
		{
			case 5:
				this.regPC = this.read(0xfffc)
				break
			case 6:
				this.regPC |= this.read(0xfffd) << 8
				this.resetRoutine = false
				this.endOpcode()
				break
		}
	}
	
	
	runNMI()
	{
		switch (this.opcodeStep)
		{
			case 2:
				this.pushStack(this.regPC >> 8)
				break
			case 3:
				this.pushStack(this.regPC & 0xff)
				break
			case 4:
				this.pushStack(this.packP())
				break
			case 6:
				this.regPC = this.read(0xfffa)
				break
			case 7:
				this.regPC |= this.read(0xfffb) << 8
				this.nmiRoutine = false
				this.endOpcode()
				break
		}
	}
	
	
	runOpcodeStep1()
	{
		if (this.acknowledgeNMI)
		{
			this.acknowledgeNMI = false
			this.nmiRoutine = true
		}
		
		else
		{
			this.opcode = this.read(this.regPC)
			
			const pcPlus1 = this.increment16Bit(this.regPC)
			const pcPlus2 = this.increment16Bit(pcPlus1)
			this.hookExecuteInstruction(this.regPC, this.opcode, this.read(pcPlus1), this.read(pcPlus2))
			
			this.incrementPC()
		}
	}
	
	
	runOpcodeStep2()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case IMM:
				this.internalData = this.read(this.regPC)
				this.incrementPC()
				this.runOperationRead()
				this.endOpcode()
				break
			case IMP:
				this.internalData = this.read(this.regPC)
				this.runOperationImplied()
				this.endOpcode()
				break
			case REL:
				this.internalData = this.read(this.regPC)
				this.incrementPC()
				break
			case ZER:
			case ZRX:
			case ZRY:
			case ABS:
			case ABX:
			case ABY:
			case PTX:
			case PTY:
			case IND:
				this.internalAddr = this.read(this.regPC)
				this.incrementPC()
				break
			case STK:
				this.internalData = this.read(this.regPC)
				if (this.opcode == opcode.BRK || this.opcode == opcode.JSR)
					this.incrementPC()
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep3()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case ZER:
				switch (this.opcodeFunctionModes[this.opcode])
				{
					case READ:
						this.internalData = this.read(this.internalAddr)
						this.runOperationRead()
						this.endOpcode()
						break
					case MDFY:
						this.internalData = this.read(this.internalAddr)
						break
					case WRIT:
						this.internalData = this.read(this.internalAddr)
						this.runOperationWrite()
						this.endOpcode()
						break
					default:
						this.throwUnhandledStep()
				}
				break
			case ZRX:
				this.read(this.internalAddr) // Dummy read
				this.internalAddr = this.calculateEffectiveAddr(this.internalAddr, this.regX, false)
				break
			case ZRY:
				this.read(this.internalAddr) // Dummy read
				this.internalAddr = this.calculateEffectiveAddr(this.internalAddr, this.regY, false)
				break
			case ABS:
			case ABX:
			case ABY:
			case IND:
				this.internalAddr |= this.read(this.regPC) << 8
				this.incrementPC()
				if (this.opcode == opcode.JMP_ABS)
				{
					this.regPC = this.internalAddr
					this.endOpcode()
					return
				}
				break
			case PTX:
				this.read(this.internalAddr) // Dummy read
				this.internalData = (this.internalAddr + this.regX) & 0xff
				break
			case PTY:
				this.internalData = this.read(this.internalAddr)
				break
			case REL:
			{
				var branchTaken = false
				switch (this.opcode)
				{
					case opcode.BPL: branchTaken = !this.flagN; break
					case opcode.BMI: branchTaken =  this.flagN; break
					case opcode.BVC: branchTaken = !this.flagV; break
					case opcode.BVS: branchTaken =  this.flagV; break
					case opcode.BCC: branchTaken = !this.flagC; break
					case opcode.BCS: branchTaken =  this.flagC; break
					case opcode.BNE: branchTaken = !this.flagZ; break
					case opcode.BEQ: branchTaken =  this.flagZ; break
					default: this.throwUnhandledStep()
				}
				
				if (!branchTaken)
					this.endOpcodePrefetch()
				else
					this.read(this.regPC) // Dummy read
				
				break
			}
			case STK:
				switch (this.opcode)
				{
					case opcode.BRK:
						this.pushStack(this.regPC >> 8)
						break
					case opcode.RTI:
					case opcode.RTS:
					case opcode.PLA:
					case opcode.PLP:
						this.incrementS()
						break
					case opcode.PHA:
						this.pushStack(this.regA)
						this.endOpcode()
						break
					case opcode.PHP:
						this.pushStack(this.packP() | FLAG_B | FLAG_U)
						this.endOpcode()
						break
					case opcode.JSR:
						break
					default:
						this.throwUnhandledStep()
				}
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep4()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case ZER:
				this.write(this.internalAddr, this.internalData)
				this.runOperationModify()
				break
			case ZRX:
			case ZRY:
			case ABS:
				switch (this.opcodeFunctionModes[this.opcode])
				{
					case READ:
						this.internalData = this.read(this.internalAddr)
						this.runOperationRead()
						this.endOpcode()
						break
					case MDFY:
						this.internalData = this.read(this.internalAddr)
						break
					case WRIT:
						this.runOperationWrite()
						this.endOpcode()
						break
					default:
						this.throwUnhandledStep()
				}
				break
			case ABX:
			{
				let addrWithoutCarry = this.calculateEffectiveAddr(this.internalAddr, this.regX, false)
				let addrWithCarry = this.calculateEffectiveAddr(this.internalAddr, this.regX, true)
				
				this.internalAddr = addrWithCarry
				this.internalData = this.read(addrWithoutCarry) // Wrong read if address needs carry
				
				if (addrWithoutCarry == addrWithCarry && this.opcodeFunctionModes[this.opcode] == READ)
				{
					this.runOperationRead()
					this.endOpcode()
				}
				break
			}
			case ABY:
			{
				let addrWithoutCarry = this.calculateEffectiveAddr(this.internalAddr, this.regY, false)
				let addrWithCarry = this.calculateEffectiveAddr(this.internalAddr, this.regY, true)
				
				this.internalAddr = addrWithCarry
				this.internalData = this.read(addrWithoutCarry) // Wrong read if address needs carry
				
				if (addrWithoutCarry == addrWithCarry && this.opcodeFunctionModes[this.opcode] == READ)
				{
					this.runOperationRead()
					this.endOpcode()
				}
				break
			}
			case PTX:
				this.internalAddr = this.read(this.internalData)
				break
			case PTY:
				this.internalAddr = this.read((this.internalAddr + 1) & 0xff) << 8
				this.internalAddr |= this.internalData
				break
			case IND:
				this.internalData = this.read(this.internalAddr)
				break
			case REL:
				// Only taken branches reach this
				let addrWithoutCarry = this.calculateAddrForBranch(this.regPC, this.internalData, false)
				let addrWithCarry = this.calculateAddrForBranch(this.regPC, this.internalData, true)
				
				if (addrWithCarry == addrWithoutCarry)
				{
					this.regPC = addrWithCarry
					this.endOpcodePrefetch()
				}
				else
					this.read(addrWithoutCarry) // Dummy read
				
				break
			case STK:
				switch (this.opcode)
				{
					case opcode.BRK:
						this.pushStack(this.regPC & 0xff)
						break
					case opcode.RTI:
						this.unpackP(this.readStack())
						this.incrementS()
						break
					case opcode.RTS:
						this.regPC = this.readStack()
						this.incrementS()
						break
					case opcode.PLA:
						this.regA = this.readStack()
						this.setFlagZero(this.regA)
						this.setFlagNegative(this.regA)
						this.endOpcode()
						break
					case opcode.PLP:
						this.unpackP(this.readStack())
						this.endOpcode()
						break
					case opcode.JSR:
						this.pushStack(this.regPC >> 8)
						break
					default:
						this.throwUnhandledStep()
				}
				break				
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep5()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case ZER:
				this.write(this.internalAddr, this.internalData)
				this.endOpcode()
				break
			case ZRX:
			case ZRY:
			case ABS:
				this.write(this.internalAddr, this.internalData)
				this.runOperationModify()
				break
			case ABX:
			case ABY:
				switch (this.opcodeFunctionModes[this.opcode])
				{
					case READ:
						this.internalData = this.read(this.internalAddr)
						this.runOperationRead()
						this.endOpcode()
						break
					case MDFY:
						this.internalData = this.read(this.internalAddr)
						break
					case WRIT:
						this.runOperationWrite()
						this.endOpcode()
						break
					default:
						this.throwUnhandledStep()						
				}
				break
			case PTX:
				this.internalAddr |= this.read((this.internalData + 1) & 0xff) << 8
				break
			case PTY:
			{
				let addrWithoutCarry = this.calculateEffectiveAddr(this.internalAddr, this.regY, false)
				let addrWithCarry = this.calculateEffectiveAddr(this.internalAddr, this.regY, true)
				
				this.internalAddr = addrWithCarry
				this.internalData = this.read(addrWithoutCarry)
				
				if (addrWithoutCarry == addrWithCarry && this.opcodeFunctionModes[this.opcode] == READ)
				{
					this.runOperationRead()
					this.endOpcode()
				}
				
				break
			}
			case REL:
				this.regPC = this.calculateAddrForBranch(this.regPC, this.internalData, true)
				this.endOpcodePrefetch()
				break
			case IND:
				this.regPC = this.internalData
				this.regPC |= this.read((this.internalAddr & 0xff00) | ((this.internalAddr + 1) & 0xff)) << 8
				this.endOpcode()
				break
			case STK:
				switch (this.opcode)
				{
					case opcode.BRK:
						this.pushStack(this.packP())
						break
					case opcode.RTI:
						this.regPC = this.readStack()
						this.incrementS()
						break
					case opcode.RTS:
						this.regPC |= this.readStack() << 8
						break
					case opcode.JSR:
						this.pushStack(this.regPC & 0xff)
						break
					default:
						this.throwUnhandledStep()
				}
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep6()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case ZRX:
			case ZRY:
			case ABS:
				this.write(this.internalAddr, this.internalData)
				this.endOpcode()
				break
			case ABX:
			case ABY:
				this.write(this.internalAddr, this.internalData)
				this.runOperationModify()
				break
			case PTX:
			case PTY:
				switch (this.opcodeFunctionModes[this.opcode])
				{
					case READ:
						this.internalData = this.read(this.internalAddr)
						this.runOperationRead()
						this.endOpcode()
						break
					case MDFY:
						this.internalData = this.read(this.internalAddr)
						break
					case WRIT:
						this.runOperationWrite()
						this.endOpcode()
						break
					default:
						this.throwUnhandledStep()						
				}
				break
			case STK:
				switch (this.opcode)
				{
					case opcode.BRK:
						this.regPC = this.read(0xfffe)
						break
					case opcode.RTI:
						this.regPC |= this.readStack() << 8
						this.endOpcode()
						break
					case opcode.RTS:
						this.incrementPC()
						this.endOpcode()
						break
					case opcode.JSR:
						this.regPC = this.internalData | (this.read(this.regPC) << 8)
						this.endOpcode()
						break
					default:
						this.throwUnhandledStep()
				}
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep7()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case ABX:
			case ABY:
				this.write(this.internalAddr, this.internalData)
				this.endOpcode()
				break
			case PTX:
			case PTY:
				this.write(this.internalAddr, this.internalData)
				this.runOperationModify()
				break
			case STK:
				this.regPC |= this.read(0xffff) << 8
				this.endOpcode()
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOpcodeStep8()
	{
		switch (this.opcodeAddressModes[this.opcode])
		{
			case PTX:
			case PTY:
				this.write(this.internalAddr, this.internalData)
				this.endOpcode()
				break
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOperationImplied()
	{
		switch (this.opcode)
		{
			case opcode.NOP:
			case opcode.NOP_2:
			case opcode.NOP_3:
			case opcode.NOP_4:
			case opcode.NOP_5:
			case opcode.NOP_6:
			case opcode.NOP_7:
			case opcode.NOP_8:
			case opcode.PHA:
			case opcode.PLA:
			case opcode.PHP:
			case opcode.PLP:
				break
				
			case opcode.CLC:
				this.flagC = false
				break
			case opcode.SEC:
				this.flagC = true
				break
			case opcode.CLI:
				this.flagI = false
				break
			case opcode.SEI:
				this.flagI = true
				break
			case opcode.CLD:
				this.flagD = false
				break
			case opcode.SED:
				this.flagD = true
				break
			case opcode.CLV:
				this.flagV = false
				break
				
			case opcode.TXA:
				this.regA = this.regX
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.TAX:
				this.regX = this.regA
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case opcode.TYA:
				this.regA = this.regY
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.TAY:
				this.regY = this.regA
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
			case opcode.TXS:
				this.regS = this.regX
				break
			case opcode.TSX:
				this.regX = this.regS
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
				
			case opcode.INX:
				this.regX = this.increment8Bit(this.regX)
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case opcode.DEX:
				this.regX = this.decrement8Bit(this.regX)
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case opcode.INY:
				this.regY = this.increment8Bit(this.regY)
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
			case opcode.DEY:
				this.regY = this.decrement8Bit(this.regY)
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
				
			case opcode.ASL_IMP:
				this.flagC = ((this.regA & 0b10000000) != 0)
				this.regA = (this.regA << 1) & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.LSR_IMP:
				this.flagC = ((this.regA & 1) != 0)
				this.regA = (this.regA >> 1) & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
				
			case opcode.ROL_IMP:
			{
				const val = (this.regA << 1) | (this.flagC ? 1 : 0)
				this.flagC = (val > 0xff)
				this.regA = val & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			}
			case opcode.ROR_IMP:
			{
				const val = this.regA | (this.flagC ? 0x100 : 0)
				this.flagC = ((val & 1) != 0)
				this.regA = (val >> 1) & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			}
			
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOperationRead()
	{
		switch (this.opcode)
		{
			case opcode.LDA_IMM:
			case opcode.LDA_ZER:
			case opcode.LDA_ZRX:
			case opcode.LDA_ABS:
			case opcode.LDA_ABX:
			case opcode.LDA_ABY:
			case opcode.LDA_PTX:
			case opcode.LDA_PTY:
				this.regA = this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.LDX_IMM:
			case opcode.LDX_ZER:
			case opcode.LDX_ZRY:
			case opcode.LDX_ABS:
			case opcode.LDX_ABY:
				this.regX = this.internalData
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case opcode.LDY_IMM:
			case opcode.LDY_ZER:
			case opcode.LDY_ZRX:
			case opcode.LDY_ABS:
			case opcode.LDY_ABX:
				this.regY = this.internalData
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
				
			case opcode.ADC_IMM:
			case opcode.ADC_ZER:
			case opcode.ADC_ZRX:
			case opcode.ADC_ABS:
			case opcode.ADC_ABX:
			case opcode.ADC_ABY:
			case opcode.ADC_PTX:
			case opcode.ADC_PTY:
			{
				const val = this.regA + this.internalData + (this.flagC ? 1 : 0)
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagV = (((this.regA ^ this.internalData) & 0x80) == 0) && (((this.regA ^ val) & 0x80) != 0)
				this.flagC = (val > 0xff)
				this.regA = val & 0xff
				break
			}
			case opcode.SBC_IMM:
			case opcode.SBC_ZER:
			case opcode.SBC_ZRX:
			case opcode.SBC_ABS:
			case opcode.SBC_ABX:
			case opcode.SBC_ABY:
			case opcode.SBC_PTX:
			case opcode.SBC_PTY:
			{
				const val = this.regA + 0x100 - this.internalData - (this.flagC ? 0 : 1)
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagV = (((this.regA ^ this.internalData) & 0x80) != 0) && (((this.regA ^ val) & 0x80) != 0)
				this.flagC = (val > 0xff)
				this.regA = val & 0xff
				break
			}
			
			case opcode.CMP_IMM:
			case opcode.CMP_ZER:
			case opcode.CMP_ZRX:
			case opcode.CMP_ABS:
			case opcode.CMP_ABX:
			case opcode.CMP_ABY:
			case opcode.CMP_PTX:
			case opcode.CMP_PTY:
			{
				const val = this.regA + 0x100 - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagC = (val > 0xff)
				break
			}
			case opcode.CPX_IMM:
			case opcode.CPX_ZER:
			case opcode.CPX_ABS:
			{
				const val = this.regX + 0x100 - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagC = (val > 0xff)
				break
			}
			case opcode.CPY_IMM:
			case opcode.CPY_ZER:
			case opcode.CPY_ABS:
			{
				const val = this.regY + 0x100 - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagC = (val > 0xff)
				break
			}
			
			case opcode.AND_IMM:
			case opcode.AND_ZER:
			case opcode.AND_ZRX:
			case opcode.AND_ABS:
			case opcode.AND_ABX:
			case opcode.AND_ABY:
			case opcode.AND_PTX:
			case opcode.AND_PTY:
				this.regA &= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.ORA_IMM:
			case opcode.ORA_ZER:
			case opcode.ORA_ZRX:
			case opcode.ORA_ABS:
			case opcode.ORA_ABX:
			case opcode.ORA_ABY:
			case opcode.ORA_PTX:
			case opcode.ORA_PTY:
				this.regA |= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case opcode.EOR_IMM:
			case opcode.EOR_ZER:
			case opcode.EOR_ZRX:
			case opcode.EOR_ABS:
			case opcode.EOR_ABX:
			case opcode.EOR_ABY:
			case opcode.EOR_PTX:
			case opcode.EOR_PTY:
				this.regA ^= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
				
			case opcode.BIT_ZER:
			case opcode.BIT_ABS:
				this.setFlagZero(this.regA & this.internalData)
				this.setFlagNegative(this.internalData)
				this.flagV = ((this.internalData & 0x40) != 0)
				break
			
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOperationModify()
	{
		switch (this.opcode)
		{
			case opcode.ASL_ZER:
			case opcode.ASL_ZRX:
			case opcode.ASL_ABS:
			case opcode.ASL_ABX:
				this.flagC = ((this.internalData & 0x80) != 0)
				this.internalData = (this.internalData << 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			case opcode.LSR_ZER:
			case opcode.LSR_ZRX:
			case opcode.LSR_ABS:
			case opcode.LSR_ABX:
				this.flagC = ((this.internalData & 1) != 0)
				this.internalData = (this.internalData >> 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
				
			case opcode.ROL_ZER:
			case opcode.ROL_ZRX:
			case opcode.ROL_ABS:
			case opcode.ROL_ABX:
			{
				const val = (this.internalData << 1) | (this.flagC ? 1 : 0)
				this.flagC = (val > 0xff)
				this.internalData = val & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			}
			case opcode.ROR_ZER:
			case opcode.ROR_ZRX:
			case opcode.ROR_ABS:
			case opcode.ROR_ABX:
			{
				const val = this.internalData | (this.flagC ? 0x100 : 0)
				this.flagC = ((val & 1) != 0)
				this.internalData = (val >> 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			}
			
			case opcode.INC_ZER:
			case opcode.INC_ZRX:
			case opcode.INC_ABS:
			case opcode.INC_ABX:
				this.internalData = this.increment8Bit(this.internalData)
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			
			case opcode.DEC_ZER:
			case opcode.DEC_ZRX:
			case opcode.DEC_ABS:
			case opcode.DEC_ABX:
				this.internalData = this.decrement8Bit(this.internalData)
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
				
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	runOperationWrite()
	{
		switch (this.opcode)
		{
			case opcode.STA_ZER:
			case opcode.STA_ZRX:
			case opcode.STA_ABS:
			case opcode.STA_ABX:
			case opcode.STA_ABY:
			case opcode.STA_PTX:
			case opcode.STA_PTY:
				this.write(this.internalAddr, this.regA)
				break
			case opcode.STX_ZER:
			case opcode.STX_ZRY:
			case opcode.STX_ABS:
				this.write(this.internalAddr, this.regX)
				break
			case opcode.STY_ZER:
			case opcode.STY_ZRX:
			case opcode.STY_ABS:
				this.write(this.internalAddr, this.regY)
				break
				
			default:
				this.throwUnhandledStep()
		}
	}
	
	
	endOpcode()
	{
		this.opcodeStep = 0
	}
	
	
	endOpcodePrefetch()
	{
		this.opcodeStep = 1
		this.runOpcodeStep1()
	}
	
	
	increment8Bit(val)
	{
		return (val + 1) & 0xff
	}
	
	
	decrement8Bit(val)
	{
		return (val + 0x100 - 1) & 0xff
	}
	
	
	increment16Bit(val)
	{
		return (val + 1) & 0xffff
	}
	
	
	decrement16Bit(val)
	{
		return (val + 0x10000 - 1) & 0xffff
	}
	
	
	incrementPC()
	{
		this.regPC = (this.regPC + 1) & 0xffff
	}
	
	
	incrementS()
	{
		this.regS = (this.regS + 1) & 0xff
	}
	
	
	decrementS()
	{
		this.regS = (this.regS + 0x100 - 1) & 0xff
	}
	
	
	packP()
	{
		return (
			(this.flagC ? FLAG_C : 0) |
			(this.flagZ ? FLAG_Z : 0) |
			(this.flagI ? FLAG_I : 0) |
			(this.flagD ? FLAG_D : 0) |
			(this.flagB ? FLAG_B : 0) |
			(this.flagV ? FLAG_V : 0) |
			(this.flagN ? FLAG_N : 0)
		)
	}
	
	
	unpackP(val)
	{
		this.flagC = ((val & FLAG_C) != 0)
		this.flagZ = ((val & FLAG_Z) != 0)
		this.flagI = ((val & FLAG_I) != 0)
		this.flagD = ((val & FLAG_D) != 0)
		this.flagB = ((val & FLAG_B) != 0)
		this.flagV = ((val & FLAG_V) != 0)
		this.flagN = ((val & FLAG_N) != 0)
	}
	
	
	calculateAddrForBranch(addr, offset, withCarry)
	{
		let signedOffset = ((offset & 0x80) == 0) ? offset : -(256 - offset)
		
		if (withCarry)
			return (addr + 0x10000 + signedOffset) & 0xffff
		else
			return (addr & 0xff00) | ((addr + 0x100 + signedOffset) & 0xff)
	}
	
	
	calculateEffectiveAddr(base, offset, withCarry)
	{
		if (withCarry)
			return (base + offset) & 0xffff
		else
			return (base & 0xff00) | ((base + offset) & 0xff)
	}
	
	
	pushStack(val)
	{
		this.write(0x100 + this.regS, val)
		this.decrementS()
	}
	
	
	readStack()
	{
		return this.read(0x100 + this.regS)
	}
	
	
	setFlagZero(val)
	{
		this.flagZ = (val == 0)
	}
	
	
	setFlagNegative(val)
	{
		this.flagN = ((val & 0b10000000) != 0)
	}
	
	
	throwUnhandledStep()
	{
		throw "unhandled opcode " + this.opcode.toString(16) + " step " + this.opcodeStep
	}
}