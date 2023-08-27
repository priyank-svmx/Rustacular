fn main() {
    let mut i = 1;
    /*    loop {
         i *= 10;
     }
    */
    println!("Hello, world!");

    //    checkedOperation();
    checkingCheckedOperation();
    testing_string_functions();
    testing_pointers_rust();
}

fn checkedOperation() {
    let mut i: i32 = 1;

    loop {
        i = i.checked_mul(10).expect("multiplication over flowed");
    }
}

fn checkingCheckedOperation() {
    assert_eq!(10_u8.checked_add(30), Some(40));
    assert_eq!(100_u8.checked_add(200), None);

    // wrapping operation on the unsigned ones
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // wrapping operations on the signed ones
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // saturating operations
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    //overflowing operations
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    //the bool type
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    //
    assert_eq!('*' as i32, 42);
}

fn testing_string_functions() {
    let text = "some text which is given to be split";
    let (head, tail) = text.split_at(20);

    println!("head is {}", head);
    println!("tail is {}", tail);
}

fn testing_pointers_rust() {
    let mut num = 78_u32;
    // let pointer: &u32 = &num;
    let pointer_mut: &mut u32 = &mut num;

    *pointer_mut = 788_u32;
    println!("printing the pointer {}", *pointer_mut);

    //lets box the values
    let num_box = Box::new(num);
    println!("printing the boxed values {}", num_box);

    //checking boxed value using tuple
    let some_tuple = (32, "elixer", 'r');
    let boxed_val = Box::new(some_tuple);
    println!("{:?}", boxed_val);
}
