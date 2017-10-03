// Opcode mnemonics
const BRK     = 0x00

const PLA     = 0x68
const PLP     = 0x28
const PHA     = 0x48
const PHP     = 0x08

const JMP_ABS = 0x4c
const JMP_IND = 0x6c
const BPL     = 0x10
const BMI     = 0x30
const BVC     = 0x50
const BVS     = 0x70
const BCC     = 0x90
const BCS     = 0xb0
const BNE     = 0xd0
const BEQ     = 0xf0

const JSR     = 0x20
const RTI     = 0x40
const RTS     = 0x60

const LDA_IMM = 0xa9
const LDA_ZER = 0xa5
const LDA_ZRX = 0xb5
const LDA_ABS = 0xad
const LDA_ABX = 0xbd
const LDA_ABY = 0xb9
const LDA_PTX = 0xa1
const LDA_PTY = 0xb1

const LDX_IMM = 0xa2
const LDX_ZER = 0xa6
const LDX_ZRY = 0xb6
const LDX_ABS = 0xae
const LDX_ABY = 0xbe

const LDY_IMM = 0xa0
const LDY_ZER = 0xa4
const LDY_ZRX = 0xb4
const LDY_ABS = 0xac
const LDY_ABX = 0xbc

const STA_ZER = 0x85
const STA_ZRX = 0x95
const STA_ABS = 0x8d
const STA_ABX = 0x9d
const STA_ABY = 0x99
const STA_PTX = 0x81
const STA_PTY = 0x91

const STX_ZER = 0x86
const STX_ZRY = 0x96
const STX_ABS = 0x8e

const STY_ZER = 0x84
const STY_ZRX = 0x94
const STY_ABS = 0x8c

const AND_IMM = 0x29
const AND_ZER = 0x25
const AND_ZRX = 0x35
const AND_ABS = 0x2d
const AND_ABX = 0x3d
const AND_ABY = 0x39
const AND_PTX = 0x21
const AND_PTY = 0x31

const ORA_IMM = 0x09
const ORA_ZER = 0x05
const ORA_ZRX = 0x15
const ORA_ABS = 0x0d
const ORA_ABX = 0x1d
const ORA_ABY = 0x19
const ORA_PTX = 0x01
const ORA_PTY = 0x11

const EOR_IMM = 0x49
const EOR_ZER = 0x45
const EOR_ZRX = 0x55
const EOR_ABS = 0x4d
const EOR_ABX = 0x5d
const EOR_ABY = 0x59
const EOR_PTX = 0x41
const EOR_PTY = 0x51

const ADC_IMM = 0x69
const ADC_ZER = 0x65
const ADC_ZRX = 0x75
const ADC_ABS = 0x6d
const ADC_ABX = 0x7d
const ADC_ABY = 0x79
const ADC_PTX = 0x61
const ADC_PTY = 0x71

const SBC_IMM = 0xe9
const SBC_ZER = 0xe5
const SBC_ZRX = 0xf5
const SBC_ABS = 0xed
const SBC_ABX = 0xfd
const SBC_ABY = 0xf9
const SBC_PTX = 0xe1
const SBC_PTY = 0xf1

const CMP_IMM = 0xc9
const CMP_ZER = 0xc5
const CMP_ZRX = 0xd5
const CMP_ABS = 0xcd
const CMP_ABX = 0xdd
const CMP_ABY = 0xd9
const CMP_PTX = 0xc1
const CMP_PTY = 0xd1

const CPX_IMM = 0xe0
const CPX_ZER = 0xe4
const CPX_ABS = 0xec

const CPY_IMM = 0xc0
const CPY_ZER = 0xc4
const CPY_ABS = 0xcc

const INX     = 0xe8
const DEX     = 0xca
const INY     = 0xc8
const DEY     = 0x88

const INC_ZER = 0xe6
const INC_ZRX = 0xf6
const INC_ABS = 0xee
const INC_ABX = 0xfe

const DEC_ZER = 0xc6
const DEC_ZRX = 0xd6
const DEC_ABS = 0xce
const DEC_ABX = 0xde

const ASL_IMP = 0x0a
const ASL_ZER = 0x06
const ASL_ZRX = 0x16
const ASL_ABS = 0x0e
const ASL_ABX = 0x1e

const LSR_IMP = 0x4a
const LSR_ZER = 0x46
const LSR_ZRX = 0x56
const LSR_ABS = 0x4e
const LSR_ABX = 0x5e

const ROL_IMP = 0x2a
const ROL_ZER = 0x26
const ROL_ZRX = 0x36
const ROL_ABS = 0x2e
const ROL_ABX = 0x3e

const ROR_IMP = 0x6a
const ROR_ZER = 0x66
const ROR_ZRX = 0x76
const ROR_ABS = 0x6e
const ROR_ABX = 0x7e

const BIT_ZER = 0x24
const BIT_ABS = 0x2c

const TXA     = 0x8a
const TAX     = 0xaa
const TYA     = 0x98
const TAY     = 0xa8
const TXS     = 0x9a
const TSX     = 0xba

const CLC     = 0x18
const SEC     = 0x38
const CLI     = 0x58
const SEI     = 0x78
const CLD     = 0xd8
const SED     = 0xf8
const CLV     = 0xb8

const NOP     = 0xea
const NOP_2   = 0x1a
const NOP_3   = 0x3a
const NOP_4   = 0x5a
const NOP_5   = 0x7a
const NOP_6   = 0xda
const NOP_7   = 0xfa
const NOP_8   = 0x80

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
		
		this.signalNMI = false
		this.signalIRQ = false
		
		this.opcode = 0
		this.opcodeStep = 0
		
		this.resetRoutine = false
		
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
	
	
	run()
	{
		this.opcodeStep += 1
		
		if (this.resetRoutine)
			this.runReset()
		
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
	
	
	runOpcodeStep1()
	{
		this.opcode = this.read(this.regPC)
		this.incrementPC()
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
				this.internalAddr = this.read(this.regPC)
				if (this.opcode == BRK || this.opcode == JSR)
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
				this.calculateAddrForZeroPageIndexed(this.regX)
				break
			case ZRY:
				this.read(this.internalAddr) // Dummy read
				this.calculateAddrForZeroPageIndexed(this.regY)
				break
			case ABS:
			case ABX:
			case ABY:
			case IND:
				this.internalAddr |= this.read(this.regPC) << 8
				this.incrementPC()
				if (this.opcode == JMP_ABS)
				{
					this.regPC = this.internalAddr
					this.endOpcode()
					return
				}
				break
			case PTX:
				this.read(this.internalAddr) // Dummy read
				this.calculateAddrForPTX()
				break
			case PTY:
				// VERIFY: Data?
				this.internalData = this.read(this.internalAddr)
				break
			case REL:
			{
				var branchTaken = false
				switch (this.opcode)
				{
					case BPL: branchTaken = !this.flagN; break
					case BMI: branchTaken =  this.flagN; break
					case BVC: branchTaken = !this.flagV; break
					case BVS: branchTaken =  this.flagV; break
					case BCC: branchTaken = !this.flagC; break
					case BCS: branchTaken =  this.flagC; break
					case BNE: branchTaken = !this.flagZ; break
					case BEQ: branchTaken =  this.flagZ; break
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
					case BRK:
						this.pushStack(this.regPC >> 8)
						break
					case RTI:
					case RTS:
					case PLA:
					case PLP:
						this.incrementS()
						break
					case PHA:
						this.pushStack(this.regA)
						this.endOpcode()
						break
					case PHP:
						this.pushStack(this.packP() | FLAG_B | FLAG_U)
						this.endOpcode()
						break
					case JSR:
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
				let addrWithoutCarry = this.calculateAddrForAbsoluteIndexed(this.regX)
				this.internalData = this.read(addrWithoutCarry) // Dummy read if address needs carry
				if (this.internalAddr == addrWithoutCarry && this.opcodeFunctionModes[this.opcode] == READ)
				{
					this.runOperationRead()
					this.endOpcode()
				}
				break
			}
			case ABY:
			{
				let addrWithoutCarry = this.calculateAddrForAbsoluteIndexed(this.regY)
				this.internalData = this.read(addrWithoutCarry) // Dummy read if address needs carry
				if (this.internalAddr == addrWithoutCarry && this.opcodeFunctionModes[this.opcode] == READ)
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
					case BRK:
						this.pushStack(this.regPC & 0xff)
						break
					case RTI:
						this.unpackP(this.popStack())
						break
					case RTS:
						this.regPC = this.popStack()
						break
					case PLA:
						this.regA = this.popStack()
						this.setFlagZero(this.regA)
						this.setFlagNegative(this.regA)
						this.endOpcode()
						break
					case PLP:
						this.unpackP(this.popStack())
						this.endOpcode()
						break
					case JSR:
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
				let addrWithoutCarry = this.calculateAddrForPTY()
				this.read(addrWithoutCarry) // Dummy read if address needs carry
				
				if (this.internalAddr == addrWithoutCarry && this.opcodeFunctionModes[this.opcode] == READ)
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
			case STK:
				switch (this.opcode)
				{
					case BRK:
						this.pushStack(this.packP())
						break
					case RTI:
						this.regPC = this.popStack()
						break
					case RTS:
						this.regPC |= this.popStack() << 8
						break
					case JSR:
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
					case BRK:
						this.regPC = this.read(0xfffe)
						break
					case RTI:
						this.regPC |= this.popStack() << 8
						this.endOpcode()
						break
					case RTS:
						this.incrementPC()
						this.endOpcode()
						break
					case JSR:
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
			case NOP:
			case NOP_2:
			case NOP_3:
			case NOP_4:
			case NOP_5:
			case NOP_6:
			case NOP_7:
			case NOP_8:
			case PHA:
			case PLA:
			case PHP:
			case PLP:
				break
				
			case CLC:
				this.flagC = false
				break
			case SEC:
				this.flagC = true
				break
			case CLI:
				this.flagI = false
				break
			case SEI:
				this.flagI = true
				break
			case CLD:
				this.flagD = false
				break
			case SED:
				this.flagD = true
				break
			case CLV:
				this.flagV = false
				break
				
			case TXA:
				this.regA = this.regX
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case TAX:
				this.regX = this.regA
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case TYA:
				this.regA = this.regY
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case TAY:
				this.regY = this.regA
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
			case TXS:
				this.regS = this.regX
				break
			case TSX:
				this.regX = this.regS
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
				
			case INX:
				this.regX = this.increment8Bit(this.regX)
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case DEX:
				this.regX = this.decrement8Bit(this.regX)
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case INY:
				this.regY = this.increment8Bit(this.regY)
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
			case DEY:
				this.regY = this.decrement8Bit(this.regY)
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
				
			case ASL_IMP:
				this.flagC = ((this.regA & 0b10000000) != 0)
				this.regA = (this.regA << 1) & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case LSR_IMP:
				this.flagC = ((this.regA & 1) != 0)
				this.regA = (this.regA >> 1) & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
				
			case ROL_IMP:
			{
				const val = (this.regA << 1) | (this.flagC ? 1 : 0)
				this.flagC = (val > 0xff)
				this.regA = val & 0xff
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			}
			case ROR_IMP:
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
			case LDA_IMM:
			case LDA_ZER:
			case LDA_ZRX:
			case LDA_ABS:
			case LDA_ABX:
			case LDA_ABY:
			case LDA_PTX:
			case LDA_PTY:
				this.regA = this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case LDX_IMM:
			case LDX_ZER:
			case LDX_ZRY:
			case LDX_ABS:
			case LDX_ABY:
				this.regX = this.internalData
				this.setFlagZero(this.regX)
				this.setFlagNegative(this.regX)
				break
			case LDY_IMM:
			case LDY_ZER:
			case LDY_ZRX:
			case LDY_ABS:
			case LDY_ABX:
				this.regY = this.internalData
				this.setFlagZero(this.regY)
				this.setFlagNegative(this.regY)
				break
				
			case ADC_IMM:
			case ADC_ZER:
			case ADC_ZRX:
			case ADC_ABS:
			case ADC_ABX:
			case ADC_ABY:
			case ADC_PTX:
			case ADC_PTY:
			{
				const val = this.regA + this.internalData + (this.flagC ? 1 : 0)
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagV = (((this.regA ^ this.internalData) & 0x80) == 0) && (((this.regA ^ val) & 0x80) != 0)
				this.flagC = (val > 0xff)
				this.regA = val & 0xff
				break
			}
			case SBC_IMM:
			case SBC_ZER:
			case SBC_ZRX:
			case SBC_ABS:
			case SBC_ABX:
			case SBC_ABY:
			case SBC_PTX:
			case SBC_PTY:
			{
				const val = this.regA + 0x100 - this.internalData - (this.flagC ? 0 : 1)
				this.setFlagZero(val & 0xff)
				this.setFlagNegative(val & 0xff)
				this.flagV = (((this.regA ^ this.internalData) & 0x80) != 0) && (((this.regA ^ val) & 0x80) != 0)
				this.flagC = (val < 0x100)
				this.regA = val & 0xff
				break
			}
			
			case CMP_IMM:
			case CMP_ZER:
			case CMP_ZRX:
			case CMP_ABS:
			case CMP_ABX:
			case CMP_ABY:
			case CMP_PTX:
			case CMP_PTY:
			{
				const val = this.regA - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative((val + 0x100) & 0xff)
				this.flagC = (val >= 0)
				break
			}
			case CPX_IMM:
			case CPX_ZER:
			case CPX_ABS:
			{
				const val = this.regX - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative((val + 0x100) & 0xff)
				this.flagC = (val >= 0)
				break
			}
			case CPY_IMM:
			case CPY_ZER:
			case CPY_ABS:
			{
				const val = this.regY - this.internalData
				this.setFlagZero(val & 0xff)
				this.setFlagNegative((val + 0x100) & 0xff)
				this.flagC = (val >= 0)
				break
			}
			
			case AND_IMM:
			case AND_ZER:
			case AND_ZRX:
			case AND_ABS:
			case AND_ABX:
			case AND_ABY:
			case AND_PTX:
			case AND_PTY:
				this.regA &= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case ORA_IMM:
			case ORA_ZER:
			case ORA_ZRX:
			case ORA_ABS:
			case ORA_ABX:
			case ORA_ABY:
			case ORA_PTX:
			case ORA_PTY:
				this.regA |= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
			case EOR_IMM:
			case EOR_ZER:
			case EOR_ZRX:
			case EOR_ABS:
			case EOR_ABX:
			case EOR_ABY:
			case EOR_PTX:
			case EOR_PTY:
				this.regA ^= this.internalData
				this.setFlagZero(this.regA)
				this.setFlagNegative(this.regA)
				break
				
			case BIT_ZER:
			case BIT_ABS:
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
			case ASL_ZER:
			case ASL_ZRX:
			case ASL_ABS:
			case ASL_ABX:
				this.flagC = ((this.internalData & 0x80) != 0)
				this.internalData = (this.internalData << 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			case LSR_ZER:
			case LSR_ZRX:
			case LSR_ABS:
			case LSR_ABX:
				this.flagC = ((this.internalData & 1) != 0)
				this.internalData = (this.internalData >> 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
				
			case ROL_ZER:
			case ROL_ZRX:
			case ROL_ABS:
			case ROL_ABX:
			{
				const val = (this.internalData << 1) | (this.flagC ? 1 : 0)
				this.flagC = (val > 0xff)
				this.internalData = val & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			}
			case ROR_ZER:
			case ROR_ZRX:
			case ROR_ABS:
			case ROR_ABX:
			{
				const val = this.internalData | (this.flagC ? 0x100 : 0)
				this.flagC = ((val & 1) != 0)
				this.internalData = (val >> 1) & 0xff
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			}
			
			case INC_ZER:
			case INC_ZRX:
			case INC_ABS:
			case INC_ABX:
				this.internalData = this.increment8Bit(this.internalData)
				this.setFlagZero(this.internalData)
				this.setFlagNegative(this.internalData)
				break
			
			case DEC_ZER:
			case DEC_ZRX:
			case DEC_ABS:
			case DEC_ABX:
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
			case STA_ZER:
			case STA_ZRX:
			case STA_ABS:
			case STA_ABX:
			case STA_ABY:
			case STA_PTX:
			case STA_PTY:
				this.write(this.internalAddr, this.regA)
				break
			case STX_ZER:
			case STX_ZRY:
			case STX_ABS:
				this.write(this.internalAddr, this.regX)
				break
			case STY_ZER:
			case STY_ZRX:
			case STY_ABS:
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
	
	
	calculateAddrForZeroPageIndexed(reg)
	{
		this.internalAddr &= 0xff00
		this.internalAddr |= (this.internalAddr + reg) & 0xff
	}
	
	
	calculateAddrForPTX()
	{
		// VERIFY: Data?
		this.internalData = (this.internalAddr + this.regX) & 0xff
	}
	
	
	calculateAddrForPTY()
	{
		let addrWithoutCarry = this.internalAddr & 0xff00
		addrWithoutCarry |= ((this.internalAddr + this.regY) & 0xff)
		
		this.internalAddr = (this.internalAddr + this.regY) & 0xffff
		
		return addrWithoutCarry
	}
	
	
	calculateAddrForAbsoluteIndexed(reg)
	{
		let addrWithoutCarry = this.internalAddr & 0xff00
		addrWithoutCarry |= ((this.internalAddr + reg) & 0xff)
		
		this.internalAddr = (this.internalAddr + reg) & 0xffff
		
		return addrWithoutCarry
	}
	
	
	pushStack(val)
	{
		this.write(0x100 + this.regS, this.regPC >> 8)
		this.decrementS()
	}
	
	
	popStack()
	{
		let val = this.read(0x100 + this.regS)
		this.incrementS()
		return val
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