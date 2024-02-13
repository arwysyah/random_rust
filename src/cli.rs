use std::{error::Error, fs::read_to_string};

pub fn generate_cli()->Result<(), Box> {

    let args = Cli::parse();
    read_to_string(&args.file)?.lines().take(args.num).for_each(|line| println!("{}",line));
    Ok(());
}
