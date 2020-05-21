#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_variables)]
pub use crate::implementation::data::State;

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

//Add memory to accumulator w/ carry
pub fn adc(currState: &State, mode: u8){

}