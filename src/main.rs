#![allow(unused)]
use std::env::args;

use crate::{constant::print_constant, functions::print_function, types::print_data_types};

mod constant;
mod types;
mod functions;
// mod cli;
//

struct Cli {
    pattern : String,
    path:std::path::PathBuf,
}
fn main() {
    println!("Hello, world!");
    print_constant();
    print_data_types();
    print_function();
    // generate_cli();
    //
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("pattern: {:?},path:{:?}",pattern,path);

    let args = Cli {
        pattern :pattern,
        path:std::path::PathBuf::from(path)
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
