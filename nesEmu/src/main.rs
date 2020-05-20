#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod implementation;
pub use crate::implementation::data;

fn main() {
    let processorState = data::build6502();

}
