pub use crate::implementation::data::State;

//This is a relatively simple file that acts as a bus
//It simply contains read and write functions that read to, 
//and write from, our 6502's memory

pub fn read(currState: &State, location: u16)->u8{
    return currState.memory[location]
}

//TODO: maybe add out of range checks to avoid panics on invalid writes?
pub fn write(currState: &State, location: u16, data: u8){
    currState.memory[location]=data
}