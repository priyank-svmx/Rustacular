fn main() {
    println!("moves in the control structures");
    let x = vec![10, 20, 39];
    /*
    *   from the if expression we can return a value
    * */ 

    let t = if true { some_fn(x) } else { some_fn(x) };
    println!("{:?}", t);
    while true {
        some_fn(x);
    }
}

fn some_fn(param: Vec<i32>) -> Vec<i32> {
    param
}
