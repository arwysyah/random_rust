use std::io;
pub fn print_data_types () {
    println!("DATA_TYPES  ==========");
    let guess :u32 = 42;
    println!("here is {guess}");

    
  let x  = 33.0;
    println!("here is  not declared but as defaultf it is f64  33 ==> {x}");

let x :f32 = 32.0;
    println!("here is f32 of 32 ==> {x} ");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    println!("SUM {sum} difference {difference}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    println!("here is tupple 500 index 0 {}",five_hundred);


println!("ARRAY ===>");



invalid_array_generator();
    println!("END OF DAATA TYPES =======> ")
}
 fn invalid_array_generator (){
let a:[i32;5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
println!("here is the index {index}");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}






