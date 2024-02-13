#![allow(unused)]
use std::env::args;

use crate::{cli::generate_cli, constant::print_constant, functions::print_function, types::print_data_types};

mod constant;
mod types;
mod functions;
mod cli;
//

fn main() {
    println!("Hello, world!");
    print_constant();
    print_data_types();
    print_function();
generate_cli();
}
