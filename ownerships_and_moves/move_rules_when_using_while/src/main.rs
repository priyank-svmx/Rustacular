fn main() {
    println!("move rules when using loops");
    let mut x = vec![23, 43, 53];
    //let mut t = Vec::new();
    while true {
        identity(x);
    }
}

fn identity(param: Vec<i32>) -> Vec<i32> {
    param
}
