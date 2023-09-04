fn main() {
    println!("better example of how moves fail");
    let s = vec!["so", "damn", "right"];
    let v = s;
    s = Vec::new();
    let u = s;
}

fn better_move() {
    println!("here the ownership is kept in one variable at each instance");

    let s = vec!["sector", "vector", "creator"];
    let _u = s.clone();
    let _v = s.clone();
}
