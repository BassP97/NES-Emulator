#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_variables)]
mod implementation;
pub use crate::implementation::data;
pub use crate::implementation::ops;
pub use crate::implementation::bus;
pub use crate::implementation::simulate;

fn main() {
    let processorState = data::build6502();
    //Current processor status - true is a-ok, false means
    //something has gone wrong
    let mut status = true;
    //load all necessary data into memory
    //run until we stop runnin!
    loop:
        simulate::checkInterrupt(&processorState)

        let status = sumulate::simulateInstruction(&processorState)
        if not status{
            break
        }
}
