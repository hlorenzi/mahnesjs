pub const BRK     : u8 = 0x00;

pub const PLA     : u8 = 0x68;
pub const PLP     : u8 = 0x28;
pub const PHA     : u8 = 0x48;
pub const PHP     : u8 = 0x08;

pub const JMP_ABS : u8 = 0x4c;
pub const JMP_IND : u8 = 0x6c;
pub const BPL     : u8 = 0x10;
pub const BMI     : u8 = 0x30;
pub const BVC     : u8 = 0x50;
pub const BVS     : u8 = 0x70;
pub const BCC     : u8 = 0x90;
pub const BCS     : u8 = 0xb0;
pub const BNE     : u8 = 0xd0;
pub const BEQ     : u8 = 0xf0;

pub const JSR     : u8 = 0x20;
pub const RTI     : u8 = 0x40;
pub const RTS     : u8 = 0x60;

pub const LDA_IMM : u8 = 0xa9;
pub const LDA_ZER : u8 = 0xa5;
pub const LDA_ZRX : u8 = 0xb5;
pub const LDA_ABS : u8 = 0xad;
pub const LDA_ABX : u8 = 0xbd;
pub const LDA_ABY : u8 = 0xb9;
pub const LDA_PTX : u8 = 0xa1;
pub const LDA_PTY : u8 = 0xb1;

pub const LDX_IMM : u8 = 0xa2;
pub const LDX_ZER : u8 = 0xa6;
pub const LDX_ZRY : u8 = 0xb6;
pub const LDX_ABS : u8 = 0xae;
pub const LDX_ABY : u8 = 0xbe;

pub const LDY_IMM : u8 = 0xa0;
pub const LDY_ZER : u8 = 0xa4;
pub const LDY_ZRX : u8 = 0xb4;
pub const LDY_ABS : u8 = 0xac;
pub const LDY_ABX : u8 = 0xbc;

pub const STA_ZER : u8 = 0x85;
pub const STA_ZRX : u8 = 0x95;
pub const STA_ABS : u8 = 0x8d;
pub const STA_ABX : u8 = 0x9d;
pub const STA_ABY : u8 = 0x99;
pub const STA_PTX : u8 = 0x81;
pub const STA_PTY : u8 = 0x91;

pub const STX_ZER : u8 = 0x86;
pub const STX_ZRY : u8 = 0x96;
pub const STX_ABS : u8 = 0x8e;

pub const STY_ZER : u8 = 0x84;
pub const STY_ZRX : u8 = 0x94;
pub const STY_ABS : u8 = 0x8c;

pub const AND_IMM : u8 = 0x29;
pub const AND_ZER : u8 = 0x25;
pub const AND_ZRX : u8 = 0x35;
pub const AND_ABS : u8 = 0x2d;
pub const AND_ABX : u8 = 0x3d;
pub const AND_ABY : u8 = 0x39;
pub const AND_PTX : u8 = 0x21;
pub const AND_PTY : u8 = 0x31;

pub const ORA_IMM : u8 = 0x09;
pub const ORA_ZER : u8 = 0x05;
pub const ORA_ZRX : u8 = 0x15;
pub const ORA_ABS : u8 = 0x0d;
pub const ORA_ABX : u8 = 0x1d;
pub const ORA_ABY : u8 = 0x19;
pub const ORA_PTX : u8 = 0x01;
pub const ORA_PTY : u8 = 0x11;

pub const EOR_IMM : u8 = 0x49;
pub const EOR_ZER : u8 = 0x45;
pub const EOR_ZRX : u8 = 0x55;
pub const EOR_ABS : u8 = 0x4d;
pub const EOR_ABX : u8 = 0x5d;
pub const EOR_ABY : u8 = 0x59;
pub const EOR_PTX : u8 = 0x41;
pub const EOR_PTY : u8 = 0x51;

pub const ADC_IMM : u8 = 0x69;
pub const ADC_ZER : u8 = 0x65;
pub const ADC_ZRX : u8 = 0x75;
pub const ADC_ABS : u8 = 0x6d;
pub const ADC_ABX : u8 = 0x7d;
pub const ADC_ABY : u8 = 0x79;
pub const ADC_PTX : u8 = 0x61;
pub const ADC_PTY : u8 = 0x71;

pub const SBC_IMM : u8 = 0xe9;
pub const SBC_ZER : u8 = 0xe5;
pub const SBC_ZRX : u8 = 0xf5;
pub const SBC_ABS : u8 = 0xed;
pub const SBC_ABX : u8 = 0xfd;
pub const SBC_ABY : u8 = 0xf9;
pub const SBC_PTX : u8 = 0xe1;
pub const SBC_PTY : u8 = 0xf1;

pub const CMP_IMM : u8 = 0xc9;
pub const CMP_ZER : u8 = 0xc5;
pub const CMP_ZRX : u8 = 0xd5;
pub const CMP_ABS : u8 = 0xcd;
pub const CMP_ABX : u8 = 0xdd;
pub const CMP_ABY : u8 = 0xd9;
pub const CMP_PTX : u8 = 0xc1;
pub const CMP_PTY : u8 = 0xd1;

pub const CPX_IMM : u8 = 0xe0;
pub const CPX_ZER : u8 = 0xe4;
pub const CPX_ABS : u8 = 0xec;

pub const CPY_IMM : u8 = 0xc0;
pub const CPY_ZER : u8 = 0xc4;
pub const CPY_ABS : u8 = 0xcc;

pub const INX     : u8 = 0xe8;
pub const DEX     : u8 = 0xca;
pub const INY     : u8 = 0xc8;
pub const DEY     : u8 = 0x88;

pub const INC_ZER : u8 = 0xe6;
pub const INC_ZRX : u8 = 0xf6;
pub const INC_ABS : u8 = 0xee;
pub const INC_ABX : u8 = 0xfe;

pub const DEC_ZER : u8 = 0xc6;
pub const DEC_ZRX : u8 = 0xd6;
pub const DEC_ABS : u8 = 0xce;
pub const DEC_ABX : u8 = 0xde;

pub const ASL_IMP : u8 = 0x0a;
pub const ASL_ZER : u8 = 0x06;
pub const ASL_ZRX : u8 = 0x16;
pub const ASL_ABS : u8 = 0x0e;
pub const ASL_ABX : u8 = 0x1e;

pub const LSR_IMP : u8 = 0x4a;
pub const LSR_ZER : u8 = 0x46;
pub const LSR_ZRX : u8 = 0x56;
pub const LSR_ABS : u8 = 0x4e;
pub const LSR_ABX : u8 = 0x5e;

pub const ROL_IMP : u8 = 0x2a;
pub const ROL_ZER : u8 = 0x26;
pub const ROL_ZRX : u8 = 0x36;
pub const ROL_ABS : u8 = 0x2e;
pub const ROL_ABX : u8 = 0x3e;

pub const ROR_IMP : u8 = 0x6a;
pub const ROR_ZER : u8 = 0x66;
pub const ROR_ZRX : u8 = 0x76;
pub const ROR_ABS : u8 = 0x6e;
pub const ROR_ABX : u8 = 0x7e;

pub const BIT_ZER : u8 = 0x24;
pub const BIT_ABS : u8 = 0x2c;

pub const TXA     : u8 = 0x8a;
pub const TAX     : u8 = 0xaa;
pub const TYA     : u8 = 0x98;
pub const TAY     : u8 = 0xa8;
pub const TXS     : u8 = 0x9a;
pub const TSX     : u8 = 0xba;

pub const CLC     : u8 = 0x18;
pub const SEC     : u8 = 0x38;
pub const CLI     : u8 = 0x58;
pub const SEI     : u8 = 0x78;
pub const CLD     : u8 = 0xd8;
pub const SED     : u8 = 0xf8;
pub const CLV     : u8 = 0xb8;

pub const NOP     : u8 = 0xea;
pub const NOP_2   : u8 = 0x1a;
pub const NOP_3   : u8 = 0x3a;
pub const NOP_4   : u8 = 0x5a;
pub const NOP_5   : u8 = 0x7a;
pub const NOP_6   : u8 = 0xda;
pub const NOP_7   : u8 = 0xfa;
pub const NOP_8   : u8 = 0x80;