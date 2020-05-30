pub use crate::implementation::data::State;

//This is a relatively simple function that acts as a bus interface
//It simply contains read and write functions that read to, 
//and write from, our 6502's memory
pub fn read(currState: &State, location: u16)->u8{
    return currState.memory[location]
}

pub fn write(currState: &mut State, location: u16, data: u8){
    currState.memory[location]=data
}

/*
The above binary arithmetic looks weird and confusing - let's de-mystify it

Since x86 is 
*/