struct S<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    println!("lifetimes and the struct");

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    println!{"{:?}", r};

}
