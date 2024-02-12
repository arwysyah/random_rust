use crate::{constant::print_constant, types::print_data_types};

mod constant;
mod types;
fn main() {
    println!("Hello, world!");
    print_constant();
    print_data_types();
}
