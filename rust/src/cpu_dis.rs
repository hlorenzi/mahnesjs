use cpu_opcodes;


pub fn disassemble_instruction(addr: u16, byte1: u8, byte2: u8, byte3: u8) -> String
{
	match byte1
	{
		cpu_opcodes::BRK     => dis_instr_imp("brk", byte2, byte3),
                             
		cpu_opcodes::PLA     => dis_instr_imp("pla", byte2, byte3),
		cpu_opcodes::PLP     => dis_instr_imp("plp", byte2, byte3),
		cpu_opcodes::PHA     => dis_instr_imp("pha", byte2, byte3),
		cpu_opcodes::PHP     => dis_instr_imp("php", byte2, byte3),
                             
		cpu_opcodes::JMP_ABS => dis_instr_abs("jmp", byte2, byte3),
		cpu_opcodes::JMP_IND => dis_instr_ind("jmp", byte2, byte3),
		cpu_opcodes::BPL     => dis_instr_rel("bpl", byte2, byte3, addr),
		cpu_opcodes::BMI     => dis_instr_rel("bmi", byte2, byte3, addr),
		cpu_opcodes::BVC     => dis_instr_rel("bvc", byte2, byte3, addr),
		cpu_opcodes::BVS     => dis_instr_rel("bvs", byte2, byte3, addr),
		cpu_opcodes::BCC     => dis_instr_rel("bcc", byte2, byte3, addr),
		cpu_opcodes::BCS     => dis_instr_rel("bcs", byte2, byte3, addr),
		cpu_opcodes::BNE     => dis_instr_rel("bne", byte2, byte3, addr),
		cpu_opcodes::BEQ     => dis_instr_rel("beq", byte2, byte3, addr),
                             
		cpu_opcodes::JSR     => dis_instr_abs("jsr", byte2, byte3),
		cpu_opcodes::RTI     => dis_instr_imp("rti", byte2, byte3),
		cpu_opcodes::RTS     => dis_instr_imp("rts", byte2, byte3),
                             
		cpu_opcodes::LDA_IMM => dis_instr_imm("lda", byte2, byte3),
		cpu_opcodes::LDA_ZER => dis_instr_zer("lda", byte2, byte3),
		cpu_opcodes::LDA_ZRX => dis_instr_zrx("lda", byte2, byte3),
		cpu_opcodes::LDA_ABS => dis_instr_abs("lda", byte2, byte3),
		cpu_opcodes::LDA_ABX => dis_instr_abx("lda", byte2, byte3),
		cpu_opcodes::LDA_ABY => dis_instr_aby("lda", byte2, byte3),
		cpu_opcodes::LDA_PTX => dis_instr_ptx("lda", byte2, byte3),
		cpu_opcodes::LDA_PTY => dis_instr_pty("lda", byte2, byte3),
                             
		cpu_opcodes::LDX_IMM => dis_instr_imm("ldx", byte2, byte3),
		cpu_opcodes::LDX_ZER => dis_instr_zer("ldx", byte2, byte3),
		cpu_opcodes::LDX_ZRY => dis_instr_zry("ldx", byte2, byte3),
		cpu_opcodes::LDX_ABS => dis_instr_abs("ldx", byte2, byte3),
		cpu_opcodes::LDX_ABY => dis_instr_aby("ldx", byte2, byte3),
                             
		cpu_opcodes::LDY_IMM => dis_instr_imm("ldy", byte2, byte3),
		cpu_opcodes::LDY_ZER => dis_instr_zer("ldy", byte2, byte3),
		cpu_opcodes::LDY_ZRX => dis_instr_zrx("ldy", byte2, byte3),
		cpu_opcodes::LDY_ABS => dis_instr_abs("ldy", byte2, byte3),
		cpu_opcodes::LDY_ABX => dis_instr_abx("ldy", byte2, byte3),
                             
		cpu_opcodes::STA_ZER => dis_instr_zer("sta", byte2, byte3),
		cpu_opcodes::STA_ZRX => dis_instr_zrx("sta", byte2, byte3),
		cpu_opcodes::STA_ABS => dis_instr_abs("sta", byte2, byte3),
		cpu_opcodes::STA_ABX => dis_instr_abx("sta", byte2, byte3),
		cpu_opcodes::STA_ABY => dis_instr_aby("sta", byte2, byte3),
		cpu_opcodes::STA_PTX => dis_instr_ptx("sta", byte2, byte3),
		cpu_opcodes::STA_PTY => dis_instr_pty("sta", byte2, byte3),
                             
		cpu_opcodes::STX_ZER => dis_instr_zer("stx", byte2, byte3),
		cpu_opcodes::STX_ZRY => dis_instr_zry("stx", byte2, byte3),
		cpu_opcodes::STX_ABS => dis_instr_abs("stx", byte2, byte3),
                             
		cpu_opcodes::STY_ZER => dis_instr_zer("sty", byte2, byte3),
		cpu_opcodes::STY_ZRX => dis_instr_zrx("sty", byte2, byte3),
		cpu_opcodes::STY_ABS => dis_instr_abs("sty", byte2, byte3),
                             
		cpu_opcodes::AND_IMM => dis_instr_imm("and", byte2, byte3),
		cpu_opcodes::AND_ZER => dis_instr_zer("and", byte2, byte3),
		cpu_opcodes::AND_ZRX => dis_instr_zrx("and", byte2, byte3),
		cpu_opcodes::AND_ABS => dis_instr_abs("and", byte2, byte3),
		cpu_opcodes::AND_ABX => dis_instr_abx("and", byte2, byte3),
		cpu_opcodes::AND_ABY => dis_instr_aby("and", byte2, byte3),
		cpu_opcodes::AND_PTX => dis_instr_ptx("and", byte2, byte3),
		cpu_opcodes::AND_PTY => dis_instr_pty("and", byte2, byte3),
                             
		cpu_opcodes::ORA_IMM => dis_instr_imm("ora", byte2, byte3),
		cpu_opcodes::ORA_ZER => dis_instr_zer("ora", byte2, byte3),
		cpu_opcodes::ORA_ZRX => dis_instr_zrx("ora", byte2, byte3),
		cpu_opcodes::ORA_ABS => dis_instr_abs("ora", byte2, byte3),
		cpu_opcodes::ORA_ABX => dis_instr_abx("ora", byte2, byte3),
		cpu_opcodes::ORA_ABY => dis_instr_aby("ora", byte2, byte3),
		cpu_opcodes::ORA_PTX => dis_instr_ptx("ora", byte2, byte3),
		cpu_opcodes::ORA_PTY => dis_instr_pty("ora", byte2, byte3),
                             
		cpu_opcodes::EOR_IMM => dis_instr_imm("eor", byte2, byte3),
		cpu_opcodes::EOR_ZER => dis_instr_zer("eor", byte2, byte3),
		cpu_opcodes::EOR_ZRX => dis_instr_zrx("eor", byte2, byte3),
		cpu_opcodes::EOR_ABS => dis_instr_abs("eor", byte2, byte3),
		cpu_opcodes::EOR_ABX => dis_instr_abx("eor", byte2, byte3),
		cpu_opcodes::EOR_ABY => dis_instr_aby("eor", byte2, byte3),
		cpu_opcodes::EOR_PTX => dis_instr_ptx("eor", byte2, byte3),
		cpu_opcodes::EOR_PTY => dis_instr_pty("eor", byte2, byte3),
                             
		cpu_opcodes::ADC_IMM => dis_instr_imm("adc", byte2, byte3),
		cpu_opcodes::ADC_ZER => dis_instr_zer("adc", byte2, byte3),
		cpu_opcodes::ADC_ZRX => dis_instr_zrx("adc", byte2, byte3),
		cpu_opcodes::ADC_ABS => dis_instr_abs("adc", byte2, byte3),
		cpu_opcodes::ADC_ABX => dis_instr_abx("adc", byte2, byte3),
		cpu_opcodes::ADC_ABY => dis_instr_aby("adc", byte2, byte3),
		cpu_opcodes::ADC_PTX => dis_instr_ptx("adc", byte2, byte3),
		cpu_opcodes::ADC_PTY => dis_instr_pty("adc", byte2, byte3),
                             
		cpu_opcodes::SBC_IMM => dis_instr_imm("sbc", byte2, byte3),
		cpu_opcodes::SBC_ZER => dis_instr_zer("sbc", byte2, byte3),
		cpu_opcodes::SBC_ZRX => dis_instr_zrx("sbc", byte2, byte3),
		cpu_opcodes::SBC_ABS => dis_instr_abs("sbc", byte2, byte3),
		cpu_opcodes::SBC_ABX => dis_instr_abx("sbc", byte2, byte3),
		cpu_opcodes::SBC_ABY => dis_instr_aby("sbc", byte2, byte3),
		cpu_opcodes::SBC_PTX => dis_instr_ptx("sbc", byte2, byte3),
		cpu_opcodes::SBC_PTY => dis_instr_pty("sbc", byte2, byte3),
                             
		cpu_opcodes::CMP_IMM => dis_instr_imm("cmp", byte2, byte3),
		cpu_opcodes::CMP_ZER => dis_instr_zer("cmp", byte2, byte3),
		cpu_opcodes::CMP_ZRX => dis_instr_zrx("cmp", byte2, byte3),
		cpu_opcodes::CMP_ABS => dis_instr_abs("cmp", byte2, byte3),
		cpu_opcodes::CMP_ABX => dis_instr_abx("cmp", byte2, byte3),
		cpu_opcodes::CMP_ABY => dis_instr_aby("cmp", byte2, byte3),
		cpu_opcodes::CMP_PTX => dis_instr_ptx("cmp", byte2, byte3),
		cpu_opcodes::CMP_PTY => dis_instr_pty("cmp", byte2, byte3),
                             
		cpu_opcodes::CPX_IMM => dis_instr_imm("cpx", byte2, byte3),
		cpu_opcodes::CPX_ZER => dis_instr_zer("cpx", byte2, byte3),
		cpu_opcodes::CPX_ABS => dis_instr_abs("cpx", byte2, byte3),
                             
		cpu_opcodes::CPY_IMM => dis_instr_imm("cpy", byte2, byte3),
		cpu_opcodes::CPY_ZER => dis_instr_zer("cpy", byte2, byte3),
		cpu_opcodes::CPY_ABS => dis_instr_abs("cpy", byte2, byte3),
                             
		cpu_opcodes::INX     => dis_instr_imp("inx", byte2, byte3),
		cpu_opcodes::DEX     => dis_instr_imp("dex", byte2, byte3),
		cpu_opcodes::INY     => dis_instr_imp("iny", byte2, byte3),
		cpu_opcodes::DEY     => dis_instr_imp("dey", byte2, byte3),
                             
		cpu_opcodes::INC_ZER => dis_instr_zer("inc", byte2, byte3),
		cpu_opcodes::INC_ZRX => dis_instr_zrx("inc", byte2, byte3),
		cpu_opcodes::INC_ABS => dis_instr_abs("inc", byte2, byte3),
		cpu_opcodes::INC_ABX => dis_instr_abx("inc", byte2, byte3),
                             
		cpu_opcodes::DEC_ZER => dis_instr_zer("dec", byte2, byte3),
		cpu_opcodes::DEC_ZRX => dis_instr_zrx("dec", byte2, byte3),
		cpu_opcodes::DEC_ABS => dis_instr_abs("dec", byte2, byte3),
		cpu_opcodes::DEC_ABX => dis_instr_abx("dec", byte2, byte3),
                             
		cpu_opcodes::ASL_IMP => dis_instr_imp("asl a", byte2, byte3),
		cpu_opcodes::ASL_ZER => dis_instr_zer("asl", byte2, byte3),
		cpu_opcodes::ASL_ZRX => dis_instr_zrx("asl", byte2, byte3),
		cpu_opcodes::ASL_ABS => dis_instr_abs("asl", byte2, byte3),
		cpu_opcodes::ASL_ABX => dis_instr_abx("asl", byte2, byte3),
                             
		cpu_opcodes::LSR_IMP => dis_instr_imp("lsr a", byte2, byte3),
		cpu_opcodes::LSR_ZER => dis_instr_zer("lsr", byte2, byte3),
		cpu_opcodes::LSR_ZRX => dis_instr_zrx("lsr", byte2, byte3),
		cpu_opcodes::LSR_ABS => dis_instr_abs("lsr", byte2, byte3),
		cpu_opcodes::LSR_ABX => dis_instr_abx("lsr", byte2, byte3),
                             
		cpu_opcodes::ROL_IMP => dis_instr_imp("rol a", byte2, byte3),
		cpu_opcodes::ROL_ZER => dis_instr_zer("rol", byte2, byte3),
		cpu_opcodes::ROL_ZRX => dis_instr_zrx("rol", byte2, byte3),
		cpu_opcodes::ROL_ABS => dis_instr_abs("rol", byte2, byte3),
		cpu_opcodes::ROL_ABX => dis_instr_abx("rol", byte2, byte3),
                             
		cpu_opcodes::ROR_IMP => dis_instr_imp("ror a", byte2, byte3),
		cpu_opcodes::ROR_ZER => dis_instr_zer("ror", byte2, byte3),
		cpu_opcodes::ROR_ZRX => dis_instr_zrx("ror", byte2, byte3),
		cpu_opcodes::ROR_ABS => dis_instr_abs("ror", byte2, byte3),
		cpu_opcodes::ROR_ABX => dis_instr_abx("ror", byte2, byte3),
                             
		cpu_opcodes::BIT_ZER => dis_instr_zer("bit", byte2, byte3),
		cpu_opcodes::BIT_ABS => dis_instr_abs("bit", byte2, byte3),
                             
		cpu_opcodes::TXA     => dis_instr_imp("txa", byte2, byte3),
		cpu_opcodes::TAX     => dis_instr_imp("tax", byte2, byte3),
		cpu_opcodes::TYA     => dis_instr_imp("tya", byte2, byte3),
		cpu_opcodes::TAY     => dis_instr_imp("tay", byte2, byte3),
		cpu_opcodes::TXS     => dis_instr_imp("txs", byte2, byte3),
		cpu_opcodes::TSX     => dis_instr_imp("tsx", byte2, byte3),
                             
		cpu_opcodes::CLC     => dis_instr_imp("clc", byte2, byte3),
		cpu_opcodes::SEC     => dis_instr_imp("sec", byte2, byte3),
		cpu_opcodes::CLI     => dis_instr_imp("cli", byte2, byte3),
		cpu_opcodes::SEI     => dis_instr_imp("sei", byte2, byte3),
		cpu_opcodes::CLD     => dis_instr_imp("cld", byte2, byte3),
		cpu_opcodes::SED     => dis_instr_imp("sed", byte2, byte3),
		cpu_opcodes::CLV     => dis_instr_imp("clv", byte2, byte3),
                             
		cpu_opcodes::NOP     => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_2   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_3   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_4   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_5   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_6   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_7   => dis_instr_imp("nop", byte2, byte3),
		cpu_opcodes::NOP_8   => dis_instr_imp("nop", byte2, byte3),
		
		_ => "???".to_string()
	}
}


fn dis_instr_imp(mnemonic: &str, _byte2: u8, _byte3: u8) -> String
{
	mnemonic.to_string()
}


fn dis_instr_imm(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} #${:02x}", mnemonic, byte2)
}


fn dis_instr_zer(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} <${:02x}", mnemonic, byte2)
}


fn dis_instr_zrx(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} <${:02x}", mnemonic, byte2)
}


fn dis_instr_zry(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} <${:02x}, y", mnemonic, byte2)
}


fn dis_instr_abs(mnemonic: &str, byte2: u8, byte3: u8) -> String
{
	format!("{} ${:02x}", mnemonic, byte2 as u16 | ((byte3 as u16) << 8))
}


fn dis_instr_abx(mnemonic: &str, byte2: u8, byte3: u8) -> String
{
	format!("{} ${:02x}, x", mnemonic, byte2 as u16 | ((byte3 as u16) << 8))
}


fn dis_instr_aby(mnemonic: &str, byte2: u8, byte3: u8) -> String
{
	format!("{} ${:02x}, y", mnemonic, byte2 as u16 | ((byte3 as u16) << 8))
}


fn dis_instr_ind(mnemonic: &str, byte2: u8, byte3: u8) -> String
{
	format!("{} (${:02x})", mnemonic, byte2 as u16 | ((byte3 as u16) << 8))
}


fn dis_instr_ptx(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} (<${:02x}, x)", mnemonic, byte2)
}


fn dis_instr_pty(mnemonic: &str, byte2: u8, _byte3: u8) -> String
{
	format!("{} (<${:02x}), y", mnemonic, byte2)
}


fn dis_instr_rel(mnemonic: &str, byte2: u8, _byte3: u8, addr: u16) -> String
{
	let extended_offset = if (byte2 & 0x80) == 0 { byte2 as u16 } else { 0xff00 | byte2 as u16 };
	let final_addr = addr.wrapping_add(extended_offset);
	
	format!("{} ${:04x}", mnemonic, final_addr)
}