

struct Cli {
    pattern : String,
    path:std::path::PathBuf,
}

pub fn generate_cli(){
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("pattern: {:?},path:{:?}",pattern,path);

    let args = Cli {
        pattern :pattern,
        path:std::path::PathBuf::from(path)
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

}
