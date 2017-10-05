export const BRK     = 0x00

export const PLA     = 0x68
export const PLP     = 0x28
export const PHA     = 0x48
export const PHP     = 0x08

export const JMP_ABS = 0x4c
export const JMP_IND = 0x6c
export const BPL     = 0x10
export const BMI     = 0x30
export const BVC     = 0x50
export const BVS     = 0x70
export const BCC     = 0x90
export const BCS     = 0xb0
export const BNE     = 0xd0
export const BEQ     = 0xf0

export const JSR     = 0x20
export const RTI     = 0x40
export const RTS     = 0x60

export const LDA_IMM = 0xa9
export const LDA_ZER = 0xa5
export const LDA_ZRX = 0xb5
export const LDA_ABS = 0xad
export const LDA_ABX = 0xbd
export const LDA_ABY = 0xb9
export const LDA_PTX = 0xa1
export const LDA_PTY = 0xb1

export const LDX_IMM = 0xa2
export const LDX_ZER = 0xa6
export const LDX_ZRY = 0xb6
export const LDX_ABS = 0xae
export const LDX_ABY = 0xbe

export const LDY_IMM = 0xa0
export const LDY_ZER = 0xa4
export const LDY_ZRX = 0xb4
export const LDY_ABS = 0xac
export const LDY_ABX = 0xbc

export const STA_ZER = 0x85
export const STA_ZRX = 0x95
export const STA_ABS = 0x8d
export const STA_ABX = 0x9d
export const STA_ABY = 0x99
export const STA_PTX = 0x81
export const STA_PTY = 0x91

export const STX_ZER = 0x86
export const STX_ZRY = 0x96
export const STX_ABS = 0x8e

export const STY_ZER = 0x84
export const STY_ZRX = 0x94
export const STY_ABS = 0x8c

export const AND_IMM = 0x29
export const AND_ZER = 0x25
export const AND_ZRX = 0x35
export const AND_ABS = 0x2d
export const AND_ABX = 0x3d
export const AND_ABY = 0x39
export const AND_PTX = 0x21
export const AND_PTY = 0x31

export const ORA_IMM = 0x09
export const ORA_ZER = 0x05
export const ORA_ZRX = 0x15
export const ORA_ABS = 0x0d
export const ORA_ABX = 0x1d
export const ORA_ABY = 0x19
export const ORA_PTX = 0x01
export const ORA_PTY = 0x11

export const EOR_IMM = 0x49
export const EOR_ZER = 0x45
export const EOR_ZRX = 0x55
export const EOR_ABS = 0x4d
export const EOR_ABX = 0x5d
export const EOR_ABY = 0x59
export const EOR_PTX = 0x41
export const EOR_PTY = 0x51

export const ADC_IMM = 0x69
export const ADC_ZER = 0x65
export const ADC_ZRX = 0x75
export const ADC_ABS = 0x6d
export const ADC_ABX = 0x7d
export const ADC_ABY = 0x79
export const ADC_PTX = 0x61
export const ADC_PTY = 0x71

export const SBC_IMM = 0xe9
export const SBC_ZER = 0xe5
export const SBC_ZRX = 0xf5
export const SBC_ABS = 0xed
export const SBC_ABX = 0xfd
export const SBC_ABY = 0xf9
export const SBC_PTX = 0xe1
export const SBC_PTY = 0xf1

export const CMP_IMM = 0xc9
export const CMP_ZER = 0xc5
export const CMP_ZRX = 0xd5
export const CMP_ABS = 0xcd
export const CMP_ABX = 0xdd
export const CMP_ABY = 0xd9
export const CMP_PTX = 0xc1
export const CMP_PTY = 0xd1

export const CPX_IMM = 0xe0
export const CPX_ZER = 0xe4
export const CPX_ABS = 0xec

export const CPY_IMM = 0xc0
export const CPY_ZER = 0xc4
export const CPY_ABS = 0xcc

export const INX     = 0xe8
export const DEX     = 0xca
export const INY     = 0xc8
export const DEY     = 0x88

export const INC_ZER = 0xe6
export const INC_ZRX = 0xf6
export const INC_ABS = 0xee
export const INC_ABX = 0xfe

export const DEC_ZER = 0xc6
export const DEC_ZRX = 0xd6
export const DEC_ABS = 0xce
export const DEC_ABX = 0xde

export const ASL_IMP = 0x0a
export const ASL_ZER = 0x06
export const ASL_ZRX = 0x16
export const ASL_ABS = 0x0e
export const ASL_ABX = 0x1e

export const LSR_IMP = 0x4a
export const LSR_ZER = 0x46
export const LSR_ZRX = 0x56
export const LSR_ABS = 0x4e
export const LSR_ABX = 0x5e

export const ROL_IMP = 0x2a
export const ROL_ZER = 0x26
export const ROL_ZRX = 0x36
export const ROL_ABS = 0x2e
export const ROL_ABX = 0x3e

export const ROR_IMP = 0x6a
export const ROR_ZER = 0x66
export const ROR_ZRX = 0x76
export const ROR_ABS = 0x6e
export const ROR_ABX = 0x7e

export const BIT_ZER = 0x24
export const BIT_ABS = 0x2c

export const TXA     = 0x8a
export const TAX     = 0xaa
export const TYA     = 0x98
export const TAY     = 0xa8
export const TXS     = 0x9a
export const TSX     = 0xba

export const CLC     = 0x18
export const SEC     = 0x38
export const CLI     = 0x58
export const SEI     = 0x78
export const CLD     = 0xd8
export const SED     = 0xf8
export const CLV     = 0xb8

export const NOP     = 0xea
export const NOP_2   = 0x1a
export const NOP_3   = 0x3a
export const NOP_4   = 0x5a
export const NOP_5   = 0x7a
export const NOP_6   = 0xda
export const NOP_7   = 0xfa
export const NOP_8   = 0x80