import * as opcode from "./cpu_opcodes.js"


export class Disassembler
{
	static disassembleInstruction(addr, byte1, byte2, byte3)
	{
		switch (byte1)
		{
			case opcode.BRK    : return instrIMP("brk", byte2, byte3)

			case opcode.PLA    : return instrIMP("pla", byte2, byte3)
			case opcode.PLP    : return instrIMP("plp", byte2, byte3)
			case opcode.PHA    : return instrIMP("pha", byte2, byte3)
			case opcode.PHP    : return instrIMP("php", byte2, byte3)

			case opcode.JMP_ABS: return instrABS("jmp", byte2, byte3)
			case opcode.JMP_IND: return instrIND("jmp", byte2, byte3)
			case opcode.BPL    : return instrREL("bpl", byte2, byte3, addr)
			case opcode.BMI    : return instrREL("bmi", byte2, byte3, addr)
			case opcode.BVC    : return instrREL("bvc", byte2, byte3, addr)
			case opcode.BVS    : return instrREL("bvs", byte2, byte3, addr)
			case opcode.BCC    : return instrREL("bcc", byte2, byte3, addr)
			case opcode.BCS    : return instrREL("bcs", byte2, byte3, addr)
			case opcode.BNE    : return instrREL("bne", byte2, byte3, addr)
			case opcode.BEQ    : return instrREL("beq", byte2, byte3, addr)

			case opcode.JSR    : return instrABS("jsr", byte2, byte3)
			case opcode.RTI    : return instrIMP("rti", byte2, byte3)
			case opcode.RTS    : return instrIMP("rts", byte2, byte3)

			case opcode.LDA_IMM: return instrIMM("lda", byte2, byte3)
			case opcode.LDA_ZER: return instrZER("lda", byte2, byte3)
			case opcode.LDA_ZRX: return instrZRX("lda", byte2, byte3)
			case opcode.LDA_ABS: return instrABS("lda", byte2, byte3)
			case opcode.LDA_ABX: return instrABX("lda", byte2, byte3)
			case opcode.LDA_ABY: return instrABY("lda", byte2, byte3)
			case opcode.LDA_PTX: return instrPTX("lda", byte2, byte3)
			case opcode.LDA_PTY: return instrPTY("lda", byte2, byte3)

			case opcode.LDX_IMM: return instrIMM("ldx", byte2, byte3)
			case opcode.LDX_ZER: return instrZER("ldx", byte2, byte3)
			case opcode.LDX_ZRY: return instrZRY("ldx", byte2, byte3)
			case opcode.LDX_ABS: return instrABS("ldx", byte2, byte3)
			case opcode.LDX_ABY: return instrABY("ldx", byte2, byte3)

			case opcode.LDY_IMM: return instrIMM("ldy", byte2, byte3)
			case opcode.LDY_ZER: return instrZER("ldy", byte2, byte3)
			case opcode.LDY_ZRX: return instrZRX("ldy", byte2, byte3)
			case opcode.LDY_ABS: return instrABS("ldy", byte2, byte3)
			case opcode.LDY_ABX: return instrABX("ldy", byte2, byte3)

			case opcode.STA_ZER: return instrZER("sta", byte2, byte3)
			case opcode.STA_ZRX: return instrZRX("sta", byte2, byte3)
			case opcode.STA_ABS: return instrABS("sta", byte2, byte3)
			case opcode.STA_ABX: return instrABX("sta", byte2, byte3)
			case opcode.STA_ABY: return instrABY("sta", byte2, byte3)
			case opcode.STA_PTX: return instrPTX("sta", byte2, byte3)
			case opcode.STA_PTY: return instrPTY("sta", byte2, byte3)

			case opcode.STX_ZER: return instrZER("stx", byte2, byte3)
			case opcode.STX_ZRY: return instrZRY("stx", byte2, byte3)
			case opcode.STX_ABS: return instrABS("stx", byte2, byte3)

			case opcode.STY_ZER: return instrZER("sty", byte2, byte3)
			case opcode.STY_ZRX: return instrZRX("sty", byte2, byte3)
			case opcode.STY_ABS: return instrABS("sty", byte2, byte3)

			case opcode.AND_IMM: return instrIMM("and", byte2, byte3)
			case opcode.AND_ZER: return instrZER("and", byte2, byte3)
			case opcode.AND_ZRX: return instrZRX("and", byte2, byte3)
			case opcode.AND_ABS: return instrABS("and", byte2, byte3)
			case opcode.AND_ABX: return instrABX("and", byte2, byte3)
			case opcode.AND_ABY: return instrABY("and", byte2, byte3)
			case opcode.AND_PTX: return instrPTX("and", byte2, byte3)
			case opcode.AND_PTY: return instrPTY("and", byte2, byte3)

			case opcode.ORA_IMM: return instrIMM("ora", byte2, byte3)
			case opcode.ORA_ZER: return instrZER("ora", byte2, byte3)
			case opcode.ORA_ZRX: return instrZRX("ora", byte2, byte3)
			case opcode.ORA_ABS: return instrABS("ora", byte2, byte3)
			case opcode.ORA_ABX: return instrABX("ora", byte2, byte3)
			case opcode.ORA_ABY: return instrABY("ora", byte2, byte3)
			case opcode.ORA_PTX: return instrPTX("ora", byte2, byte3)
			case opcode.ORA_PTY: return instrPTY("ora", byte2, byte3)

			case opcode.EOR_IMM: return instrIMM("eor", byte2, byte3)
			case opcode.EOR_ZER: return instrZER("eor", byte2, byte3)
			case opcode.EOR_ZRX: return instrZRX("eor", byte2, byte3)
			case opcode.EOR_ABS: return instrABS("eor", byte2, byte3)
			case opcode.EOR_ABX: return instrABX("eor", byte2, byte3)
			case opcode.EOR_ABY: return instrABY("eor", byte2, byte3)
			case opcode.EOR_PTX: return instrPTX("eor", byte2, byte3)
			case opcode.EOR_PTY: return instrPTY("eor", byte2, byte3)

			case opcode.ADC_IMM: return instrIMM("adc", byte2, byte3)
			case opcode.ADC_ZER: return instrZER("adc", byte2, byte3)
			case opcode.ADC_ZRX: return instrZRX("adc", byte2, byte3)
			case opcode.ADC_ABS: return instrABS("adc", byte2, byte3)
			case opcode.ADC_ABX: return instrABX("adc", byte2, byte3)
			case opcode.ADC_ABY: return instrABY("adc", byte2, byte3)
			case opcode.ADC_PTX: return instrPTX("adc", byte2, byte3)
			case opcode.ADC_PTY: return instrPTY("adc", byte2, byte3)

			case opcode.SBC_IMM: return instrIMM("sbc", byte2, byte3)
			case opcode.SBC_ZER: return instrZER("sbc", byte2, byte3)
			case opcode.SBC_ZRX: return instrZRX("sbc", byte2, byte3)
			case opcode.SBC_ABS: return instrABS("sbc", byte2, byte3)
			case opcode.SBC_ABX: return instrABX("sbc", byte2, byte3)
			case opcode.SBC_ABY: return instrABY("sbc", byte2, byte3)
			case opcode.SBC_PTX: return instrPTX("sbc", byte2, byte3)
			case opcode.SBC_PTY: return instrPTY("sbc", byte2, byte3)

			case opcode.CMP_IMM: return instrIMM("cmp", byte2, byte3)
			case opcode.CMP_ZER: return instrZER("cmp", byte2, byte3)
			case opcode.CMP_ZRX: return instrZRX("cmp", byte2, byte3)
			case opcode.CMP_ABS: return instrABS("cmp", byte2, byte3)
			case opcode.CMP_ABX: return instrABX("cmp", byte2, byte3)
			case opcode.CMP_ABY: return instrABY("cmp", byte2, byte3)
			case opcode.CMP_PTX: return instrPTX("cmp", byte2, byte3)
			case opcode.CMP_PTY: return instrPTY("cmp", byte2, byte3)

			case opcode.CPX_IMM: return instrIMM("cpx", byte2, byte3)
			case opcode.CPX_ZER: return instrZER("cpx", byte2, byte3)
			case opcode.CPX_ABS: return instrABS("cpx", byte2, byte3)

			case opcode.CPY_IMM: return instrIMM("cpy", byte2, byte3)
			case opcode.CPY_ZER: return instrZER("cpy", byte2, byte3)
			case opcode.CPY_ABS: return instrABS("cpy", byte2, byte3)

			case opcode.INX    : return instrIMP("inx", byte2, byte3)
			case opcode.DEX    : return instrIMP("dex", byte2, byte3)
			case opcode.INY    : return instrIMP("iny", byte2, byte3)
			case opcode.DEY    : return instrIMP("dey", byte2, byte3)

			case opcode.INC_ZER: return instrZER("inc", byte2, byte3)
			case opcode.INC_ZRX: return instrZRX("inc", byte2, byte3)
			case opcode.INC_ABS: return instrABS("inc", byte2, byte3)
			case opcode.INC_ABX: return instrABX("inc", byte2, byte3)

			case opcode.DEC_ZER: return instrZER("dec", byte2, byte3)
			case opcode.DEC_ZRX: return instrZRX("dec", byte2, byte3)
			case opcode.DEC_ABS: return instrABS("dec", byte2, byte3)
			case opcode.DEC_ABX: return instrABX("dec", byte2, byte3)

			case opcode.ASL_IMP: return instrIMP("asl a", byte2, byte3)
			case opcode.ASL_ZER: return instrZER("asl", byte2, byte3)
			case opcode.ASL_ZRX: return instrZRX("asl", byte2, byte3)
			case opcode.ASL_ABS: return instrABS("asl", byte2, byte3)
			case opcode.ASL_ABX: return instrABX("asl", byte2, byte3)

			case opcode.LSR_IMP: return instrIMP("lsr a", byte2, byte3)
			case opcode.LSR_ZER: return instrZER("lsr", byte2, byte3)
			case opcode.LSR_ZRX: return instrZRX("lsr", byte2, byte3)
			case opcode.LSR_ABS: return instrABS("lsr", byte2, byte3)
			case opcode.LSR_ABX: return instrABX("lsr", byte2, byte3)

			case opcode.ROL_IMP: return instrIMP("rol a", byte2, byte3)
			case opcode.ROL_ZER: return instrZER("rol", byte2, byte3)
			case opcode.ROL_ZRX: return instrZRX("rol", byte2, byte3)
			case opcode.ROL_ABS: return instrABS("rol", byte2, byte3)
			case opcode.ROL_ABX: return instrABX("rol", byte2, byte3)

			case opcode.ROR_IMP: return instrIMP("ror a", byte2, byte3)
			case opcode.ROR_ZER: return instrZER("ror", byte2, byte3)
			case opcode.ROR_ZRX: return instrZRX("ror", byte2, byte3)
			case opcode.ROR_ABS: return instrABS("ror", byte2, byte3)
			case opcode.ROR_ABX: return instrABX("ror", byte2, byte3)

			case opcode.BIT_ZER: return instrZER("bit", byte2, byte3)
			case opcode.BIT_ABS: return instrABS("bit", byte2, byte3)

			case opcode.TXA    : return instrIMP("txa", byte2, byte3)
			case opcode.TAX    : return instrIMP("tax", byte2, byte3)
			case opcode.TYA    : return instrIMP("tya", byte2, byte3)
			case opcode.TAY    : return instrIMP("tay", byte2, byte3)
			case opcode.TXS    : return instrIMP("txs", byte2, byte3)
			case opcode.TSX    : return instrIMP("tsx", byte2, byte3)

			case opcode.CLC    : return instrIMP("clc", byte2, byte3)
			case opcode.SEC    : return instrIMP("sec", byte2, byte3)
			case opcode.CLI    : return instrIMP("cli", byte2, byte3)
			case opcode.SEI    : return instrIMP("sei", byte2, byte3)
			case opcode.CLD    : return instrIMP("cld", byte2, byte3)
			case opcode.SED    : return instrIMP("sed", byte2, byte3)
			case opcode.CLV    : return instrIMP("clv", byte2, byte3)

			case opcode.NOP    : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_2  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_3  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_4  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_5  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_6  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_7  : return instrIMP("nop", byte2, byte3)
			case opcode.NOP_8  : return instrIMP("nop", byte2, byte3)
		}
	}
}


function instrIMP(mnemonic, byte2, byte3)
{
	return mnemonic
}


function instrIMM(mnemonic, byte2, byte3)
{
	return mnemonic + " #$" + byte2.toString(16)
}


function instrZER(mnemonic, byte2, byte3)
{
	return mnemonic + " <$" + byte2.toString(16)
}


function instrZRX(mnemonic, byte2, byte3)
{
	return mnemonic + " <$" + byte2.toString(16) + ", x"
}


function instrZRY(mnemonic, byte2, byte3)
{
	return mnemonic + " <$" + byte2.toString(16) + ", y"
}


function instrABS(mnemonic, byte2, byte3)
{
	return mnemonic + " $" + (byte2 | (byte3 << 8)).toString(16)
}


function instrABX(mnemonic, byte2, byte3)
{
	return mnemonic + " $" + (byte2 | (byte3 << 8)).toString(16) + ", x"
}


function instrABY(mnemonic, byte2, byte3)
{
	return mnemonic + " $" + (byte2 | (byte3 << 8)).toString(16) + ", y"
}


function instrIND(mnemonic, byte2, byte3)
{
	return mnemonic + " ($" + (byte2 | (byte3 << 8)).toString(16) + ")"
}


function instrPTX(mnemonic, byte2, byte3)
{
	return mnemonic + " (<$" + byte2.toString(16) + ", x)"
}


function instrPTY(mnemonic, byte2, byte3)
{
	return mnemonic + " (<$" + byte2.toString(16) + "), y"
}


function instrREL(mnemonic, byte2, byte3, addr)
{
	const signedOffset = ((byte2 & 0x80) == 0) ? byte2 : -(256 - byte2)
	const finalAddr = (addr + 0x10000 + signedOffset) & 0xffff
	
	return mnemonic + " $" + finalAddr.toString(16)
}