
struct Person { name: String, birth: i32,}
fn main() {
    println!("some examples than actually move");

    let mut s = "gibralterj".to_string();
    let t = s;
    s = "a very new stringj".to_string();
    //t = "some more value".to_string();

    let mut some_vec = Vec::new();
    some_vec.push(Person {name: s, birth: 1978});
    some_vec.push(Person {name: t, birth: 1987});
}


