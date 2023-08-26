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
    assert_eq!(100_u8.checked_add(200), Some(200));
}
