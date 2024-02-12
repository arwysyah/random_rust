
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
    println!("END OF DAATA TYPES =======> ")
}







