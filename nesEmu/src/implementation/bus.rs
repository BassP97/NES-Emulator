pub use crate::implementation::data::State;

//This is a relatively simple function that acts as a bus interface
//It simply contains read and write functions that read to, 
//and write from, our 6502's memory
pub fn read(currState: &State, location: u16)->u8{
    return currState.memory[location as usize]
}

pub fn write(currState: &mut State, location: u16, data: u8){
    currState.memory[location as usize]=data
}

/*
Info about memory:
- Zero Page refers to addresses in the range $0000-$00FF, that is 
the first page in memory and is used by certain addressing modes to allow 
quicker execution. 

- Memory locations $0000-$07FF are mirrored three times at $0800-$1FFF. T
This means that, for example, any data written to $0000 will also be 
written to $0800, $1000 and $1800.

- The memory mapped to the I/O registers is located between $2000-$401F

- 
*/