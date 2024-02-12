use crate::{constant::print_constant, functions::print_function, types::print_data_types};

mod constant;
mod types;
mod functions;
fn main() {
    println!("Hello, world!");
    print_constant();
    print_data_types();
    print_function();
}
