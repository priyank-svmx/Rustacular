static AN_INTEGER: i32 = 45;

fn main() {
    println!("checking functions how they behave with the refs");
    i_will_accept_static(&AN_INTEGER);
    let x: i32 = 23;
    g(&x);
    let y: &i32 = smallest(&[21, 75, 89, 09, 45, 11]);
    println!("{:?}", y);
}

// a function that accepts only refs of static i32 values
static mut STASH: &i32 = &34;
fn i_will_accept_static(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// defining a function with the lifetime parameter
fn g<'a>(variable: &'a i32) {
    println! {"{:?}", variable};
}

//ref passing to the function
fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s: &i32 = &v[0];
    for r in &v[1..] {
        if (*s > *r) {
            s = r;
        }
    }
    s
}
