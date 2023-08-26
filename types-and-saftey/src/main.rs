fn main() {
    println!("Hello, world!");

    let x: (char, u32, i32) = ('n', 234, -1);
    println!("the first tuple type {:?}", x);
    let emptyTuple: () = ();
    /*
    The {:?} format specifier is for debugging,
    and it works if all the types inside
    the tuple implement the Debug trait
    */
    println!("printing empty tuple {:?}", emptyTuple);

    #[derive(Debug)]
    struct Custom {
        x: f32,
        y: f32,
    };

    let instance_of_custom_struct: Custom = Custom { x: 34.4, y: 45.4 };
    println!("print a custom struct {:?}", instance_of_custom_struct);

    #[derive(Debug)]
    struct T(i32, char);

    let tuple_type: T = T(234, 'e');
    println!("printing the tuple type {:?}", tuple_type);

    #[derive(Debug)]
    struct E;

    let m: E = E;
    println!("{:?}", m);

    #[derive(Debug)]
    enum Attend {
        OnTime,
        Late(u32),
    }

    let enumm: Attend = Attend::OnTime;
    let enumm2: Attend = Attend::Late(34);
    println!("enum printing {:?}, {:?}", enumm, enumm2);

    let x: Attend = Attend::Late(15);
    let boxx = Box::new(x);
    println!("printing Box.Attend {:?}", boxx);

    let int_val = 12_i8;
    println!("printing 12_i8 {}", int_val);

    println!("so the abs type is {:?}", (-4_i32).abs());
}
