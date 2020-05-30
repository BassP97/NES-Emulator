#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_variables)]
pub use crate::implementation::data::State;
pub use crate::implementation::bus;

//This file emulates the 56 valid instructions in the 6502's ISA

/*
Some instructions can be mutated by the processor's *addressing mode* - 
for instance, LDA loads a value into the accumulator. However, LDA can,
depending on the addressing mode, either load data from an immediate source
or from memory. Which one it does depends on the aforementioned *addressing
mode*, which we can determine based on the op-code. Therefore, for the
instructions that require it, we pass in a *mode* value that tells us 
which addressing mode a given instruction is using. 
*/

//addressing modes - what these mean exactly is detailed in simulate.rs
const accu: u8 = 0x00;
const abso: u8 = 0x01;
const absX: u8 = 0x02;
const absY: u8 = 0x03;
const imme: u8 = 0x04;
const impi: u8 = 0x05;
const indr: u8 = 0x06;
const Xind: u8 = 0x07;
const indY: u8 = 0x08;
const rela: u8 = 0x09;
const zpag: u8 = 0x0A;
const zpgX: u8 = 0x0B;
const zpgY: u8 = 0x0C;

//Addressing mode guide:
    /*
    * Accumulator
        * operates directly on the accumulator - EG LSR A 
        * logically shifts the values in the A register right
    * Absolute
        * Uses an absolute address given in the next two bytes
        * Eg JMP 0x1234 jumps the PC to location 1234
    * Absolute X
        * uses absolute address offset by the value in the X register
        * Eg if X = 0xFF, then STA $3000,X stores A at address 0x30FF
    * Absolute Y
        * Uses absolute address offset by the value in the Y register 
    * Immediate 
        * Perform an operation using an 8 bit constant
        * Syntactically uses a # - so LDA #$10 loads 0x10 into the accumulator
    * Implicit
        * The easiest to deal with - the source or destination is implied 
        * by the op code itself
        * For instance, CLC simply clears the carry flag - no memory shenanigans required
   * Indirect
        * Syntactically, resembles JMP ($ZZZZ)
        * JMP is the only instruction that uses this addressing mode
        * The op code is followed by a 16 big address that points to the location of the 
        * LSB of *another* 16 bit address which is the real target of the instruction
        * As an example, if $0120 contains 0xFC, and $0121 contains 0xBA, then
        * JMP ($0120) will jump the PC to 0xBAFC
    * Indexed indirect
        * This is, functionally, a lookup - given a 1 byte number and the X registe
        * We then, using our 1 byte number, do a lookup in our zero page to get a 2 byte address
        * We then operate upon the 2 bytes of memory pointed to by said 2 byte address
        * For instance, consider LDA ($20,X) where X contains 0x04 and the memory at $0024 contains 0x7420 
        * $0024 contains 0x74 and $0025 contains 0x20 - recall that the 6502 is 
        * little endian, so this value is `really' 0x2074
        * Therefore, this instruction loads A with the contents of memory at location 0x2074 
        * This can be thought of as a double dereference of a pointer to a pointer
    * Indirect Indexed
        * Similar to indexed indirect, but differs in order
        * Given a 1 byte address, we do our zero page lookup, retrieving our 2 byte address (XXXX) as above
        * We then add Y to our 2 byte address, giving us our final lookup address, XXXX+Y
        * We then perform our operation on the data found at $(XXXX+Y)
        * Consider a similar example to the above, LDA ($20),Y, where the address stored at 0x0020 is 
        * $4028 and Y equals 0x10
        * We first retrieve the address stored at zero page location 0x20, where we find 
        * 28 40 (remember, little endian!)
        * We then increment it by the value of Y, giving us 4038
        * We then load the value at 0x4038 into A
    * Relative
        * This is only used for branch operations - the byte found after the op code is the branch offset
        * If the branch is taken, the new address equals the current PC plus the offset - offset is signed
    * Zero Page
        * Given one byte, ZZ, retrieves data at location $00ZZ
        * Eg LDA $22 loads the accumulator with the byte at $0022
    * Zero page X
        * Given a byte ZZ, accesses the data at location $00(ZZ+X)
        * Eg if X contains 4, and we write LDA $22,X, we load the accumulator with the byte at $0026
    * Zero page Y
        * Same thing as zero page X, but using the Y register
    * 
    */

fn checkAndUpdateCarry(currState: &State, result: u16)->bool{

}

//Add memory to accumulator w/ carry - in other words, 
//A + M + C -> A, C - sets the N, Z, C, and V flags
pub fn adc(currState: &mut State, mode: u8){
    let temp: u16 = currState.accumulator;
    let priorValue: u8 = currState.accumulator;
    currState.statusRegister.carry = 0;
    if mode = imme{
        std::print("immediate")
    }else if mode = zpag{
        std::print("zero page!")
    }else if mode = zpgX{
        std::print("zero page X!")
    }else if mode = abso{
        std::print("absolute")
    }else if mode = absX{
        std::print("absolute X")
    }else if mode = absY{
        std::print("absolute Y")
    }else if mode = Xind{
        std::print("indirect X")
    }else if mode = indY{
        std::print("indirect Y")
    }else 
        std::panic!("Illegal addressing mode for instruction ADC");
    }
    checkAndUpdateCarry(&currState, temp);
    checkAndUpdateOverflow(&currState, temp);
    checkAndUpdateNegative(&currState, temp);
    checkAndUpdateZero(&currState, temp);
}

pub fn and(currState: &mut State, mode: u8){
    //placeholder value
    let mem:u8 = 0x00;
    let mut temp:u8 = 0x00;
    if mode = imme{
        std::print("immediate")
    }else if mode = zpag{
        std::print("zero page!")
    }else if mode = zpgX{
        std::print("zero page X!")
    }else if mode = abso{
        std::print("absolute")
    }else if mode = absX{
        std::print("absolute X")
    }else if mode = absY{
        std::print("absolute Y")
    }else if mode = Xind{
        std::print("indirect X")
    }else if mode = indY{
        std::print("indirect Y")
    }else 
        std::panic!("Illegal addressing mode for instruction ADC");
    }
    checkAndUpdateZero(&currState, temp)
}