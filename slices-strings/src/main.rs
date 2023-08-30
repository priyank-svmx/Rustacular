use regex::Regex;

fn main() {
    println!("working with the slices and vectors and arrays!");
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(&v);
    print(&a);
    println!("new line before we jump to slice of slices");
    print(&sv[0..3]);
    println!("new line before we jumbp to another slice");
    print(&sa[0..]);

    printing_different_kind_of_strings();
    letSeeByteString();
    stringsInMemory();
    lets_change_a_mutable_string();
    join_and_concat();
using_string_literals();



}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

fn printing_different_kind_of_strings() {
    let speech = "\"ouch!\" what am i doing here.\n";
    let multilineStr = "so i was thinging about if we can have
multiline string our not.";

    let rgexPattern = Regex::new(r"\d+(\.\d+)*");

    let constructingaraw_string = r"c:\home\delta\gama";
}

fn letSeeByteString() {
    let method = b"GET";
    println!("{:?}", method);
}

fn stringsInMemory() {
    let noodles = "noodles";
    let oodles = &noodles[1..];
    let joodles = noodles.to_string();

    println!("{}", noodles);
    println!("{}", joodles);
}

fn lets_change_a_mutable_string() {
    let mut str = "help me";
    //str[0] = 'g';
    let m = str.bytes().nth(0);
    println!("printing the byte captured {:?}", m);
    //println!("{:?}",  str.make_ascii_uppercase());
}

// new stuff related to join and concat on vectors
//
//
fn join_and_concat() {
    let bits = vec!["bit1", "bit2", "bit3"];
    println!("{}", bits.join(","));
    println!("{}", bits.concat());
}



fn using_string_literals(){

println!("{}","peanut".contains("nut"));
    println!("{}","peanut".replace("p","sea"));
    println!("{}","    peanut".trim());
    println!("{}","peanut".starts_with("v"));


}
