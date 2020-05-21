#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_variables)]
pub use crate::implementation::data::State;
pub use crate::implementation::data::ops;

/*
Another simple file - this emulates the instruction at the 
current program counter, meaning it decodes the instruction
and calls the appropriate op function

We accomplish this using the classic, tried and true
big-ol-switch-statement method 

Documentation of the 6502 ISA: 
https://www.masswerk.at/6502/6502_instruction_set.html#BRK
*/

/*
All the addressing modes - this looks like a lot,
but most instructions only use a couple

Also, I made them all 4 characters long for the 
*aesthetic*, which I hope the reader appreciates
*/

//operand is AC (implied single byte instruction)
const accu: u8 = 0x00;
//operand is address $HHLL *
const abso: u8 = 0x01;
//operand is address; effective address is address incremented by X WITH carry 
const absX: u8 = 0x02;
//operand is address; effective address is address incremented by Y WITH carry
const absY: u8 = 0x03;
//operand is the immediate byte
const imme: u8 = 0x04;
//operand is implied
const impi: u8 = 0x05;//can't name it `impl' cuz that's a rust keyword - boo!
//operand is address; effective address is contents of word at address: C.w($HHLL)
const indr: u8 = 0x06;
//operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
const Xind: u8 = 0x07;
//operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
const indY: u8 = 0x08;
//branch target is PC + signed offset byte, which is the byte immediately following the instruction
const rela: u8 = 0x09;
//operand is zeropage address (hi-byte is zero, address = $00LL) where LL is the byte immediately following the instruction
const zpag: u8 = 0x0A;
//operand is zeropage address; effective address is address incremented by X without carry 
const zpgX: u8 = 0x0B;
//operand is zeropage address; effective address is address incremented by Y without carry **
const zpgY: u8 = 0x0C;

pub fn checkInterrupt(currState: &State){

}

pub fn simulateInstruction(currState: &State)->bool{
    let opCode = currState.memory[currState.programCounter];
    match opCode{
        //0x00 to 0x0F in the documentation
        0x00 => ops::brk(&currState, impi),
        0x01 => ops::ord(&currState, Xind),
        0x05 => ops::ora(&currState, zpag),
        0x06 => ops::asl(&currState, zpag),
        0x08 => ops::php(&currState, impi),
        0x09 => ops::ora(&currState, imme),
        0x0A => ops::asl(&currState, accu),
        0x0D => ops::ora(&currState, abso),
        0x0E => ops::asl(&currState, abso),
        
        //0x10 to 0x1F
        0x10 => ops::bpl(&currState, rela),
        0x11 => ops::ora(&currState, indY),
        0x15 => ops::ora(&currState, zpgX),
        0x16 => ops::asl(&currState, zpgX),
        0x18 => ops::clc(&currState, impi),
        0x19 => ops::ora(&currState, absY),
        0x1D => ops::ora(&currState, absX),
        0x1E => ops::asl(&currState, absX),

        //0x20 to 0x2F
        0x20 => ops::jsr(&currState, abso),
        0x21 => ops::and(&currState, Xind),
        0x24 => ops::bit(&currState, zpag),
        0x25 => ops::and(&currState, zpag),
        0x26 => ops::rol(&currState, zpag),
        0x28 => ops::plp(&currState, impi),
        0x29 => ops::and(&currState, imme),
        0x2A => ops::rol(&currState, accu),
        0x2C => ops::bit(&currState, abso),
        0x2D => ops::and(&currState, abso),
        0x2E => ops::rol(&currState, abso),

        //0x30 to 0x3F
        0x30 => ops::bmi(&currState, rela),
        0x31 => ops::and(&currState, indY),
        0x35 => ops::and(&currState, zpgX),
        0x36 => ops::rol(&currState, zpgX),
        0x38 => ops::sec(&currState, impi),
        0x39 => ops::and(&currState, absY),
        0x3D => ops::and(&currState, absX),
        0x3E => ops::rol(&currState, absX),
        
        //0x40 to 0x4F
        0x40 => ops::rti(&currState, impi),
        0x41 => ops::eor(&currState, Xind),
        0x45 => ops::eor(&currState, zpag),
        0x46 => ops::lsr(&currState, zpag),
        0x48 => ops::pha(&currState, impi),
        0x49 => ops::eor(&currState, imme),
        0x4A => ops::lsr(&currState, accu),
        0x4C => ops::jmp(&currState, abso),
        0x4D => ops::eor(&currState, abso),
        0x4E => ops::lsr(&currState, abso),
        
        _ => return false,
    }
}

