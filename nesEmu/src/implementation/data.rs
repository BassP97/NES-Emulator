#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct statusReg{
    carry: bool,
    zero: bool,
    IRQ: bool,
    decimal: bool,
    BRK: bool,
    alwaysSet: bool,
    overflow: bool,
    negative: bool,
}

pub struct State{
    //Program counter: stores current instruction location
    programCounter: u16,
    //Accumulator: stores the result of arithmatic and logical ops
    accumulator: u8,
    /*Index registers: store a whole bunch of stuff, including 
    memory locations and data for arithmatic and logical ops*/
    xRegister: u8,
    yRegister: u8,
    //Stack pointer: stores our current position on the stack
    stackPointer: u8,
    /*Status register: a series of 8 bits that track our processor's status. For simplicty, we abstract these to bools
    From lo to hi (bit 0 to bit 7) these are:
        - Carry flag
        - Zero flag
        - IRQ disable
        - Decimal mode
        - BRK flag
        - 1 - this bit is always set to 1 for some reason; nobody (least of all me) knows why!
        - Overflow
        - Negative
    Documentation for what these flags mean can be found in the "misc" 
    section at the bottom of this file
    */
    statusRegister: statusReg,
}
//------------------6502 Constructor-----------------
pub fn build6502()->State{
    let temp = statusReg{
        carry: false,
        zero: false,
        IRQ: false,
        decimal: false,
        BRK: false,
        alwaysSet: true,
        overflow: false,
        negative: false,
    };

    let res = State{
        programCounter: 0x0000,
        accumulator: 0x00,
        xRegister: 0x00,
        yRegister: 0x00,
        stackPointer: 0xff,
        statusRegister: temp,
    };
    return res
}

/*
Misc Documentation
    Flags, copied from the NES dev wiki:
        Carry:
            After ADC, this is the carry result of the addition.
            After SBC or CMP, this flag will be set if no borrow was the result, or alternatively a "greater than or equal" result.
            After a shift instruction (ASL, LSR, ROL, ROR), this contains the bit that was shifted out.
            Increment and decrement instructions do not affect the carry flag.
            Can be set or cleared directly with SEC, CLC.
        Zero:
            After most instructions that have a value result, if that value is zero, this flag will be set.
        IRQ:
            When set, all interrupts except the NMI are inhibited.
            Can be set or cleared directly with SEI, CLI.
            Automatically set by the CPU when an IRQ is triggered, and restored to its previous state by RTI.
            If the /IRQ line is low (IRQ pending) when this flag is cleared, an interrupt will immediately be triggered.
        Decimal:
            Not used on the NES, so we don't really care!
        Overflow:
            DC and SBC will set this flag if the signed result would be invalid[1], necessary for making signed comparisons[2].
            BIT will load bit 6 of the addressed value directly into the V flag.
            Can be cleared directly with CLV. There is no corresponding set instruction.
        Negative:
            After most instructions that have a value result, this flag will contain bit (the 'is this signed or unsigned' bit) of that result.
            BIT will load bit 7 of the addressed value directly into the N flag.
        Brk:
            While there are only six flags in the processor status register within the CPU, when transferred to the stack, there are two additional bits. These do not represent a register that can hold a value but can be used to distinguish how the flags were pushed.

            Some 6502 references call this the "B flag", though it does not represent an actual CPU register.

            Two interrupts (/IRQ and /NMI) and two instructions (PHP and BRK) push the flags to the stack. In the byte pushed, bit 5 is always set to 1, and bit 4 is 1 if from an instruction (PHP or BRK) or 0 if from an interrupt line being pulled low (/IRQ or /NMI). This is the only time and place where the B flag actually exists: not in the status register itself, but in bit 4 of the copy that is written to the stack. 
*/