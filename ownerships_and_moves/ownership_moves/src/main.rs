fn main() {
    println!("ownerships and moves");
    let val_to_be_boxed = print_padovan();
    boxing_the_value(val_to_be_boxed);
}

fn print_padovan() -> Vec<i32> {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    boxing_the_value(padovan);
    println!("Padovan(1..10) = {:?}", padovan);
    return padovan;
}

fn boxing_the_value(val_to_be_boxed: Vec<i32>) {
    let mut box_val = Box::new(val_to_be_boxed);
    box_val.push(45);
    println!("the boxed val is {:?}", box_val);
}
