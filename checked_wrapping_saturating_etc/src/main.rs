fn main() {
    let mut i = 1;
    /*    loop {
         i *= 10;
     }
    */
    println!("Hello, world!");

    //    checkedOperation();
    checkingCheckedOperation();
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
}
