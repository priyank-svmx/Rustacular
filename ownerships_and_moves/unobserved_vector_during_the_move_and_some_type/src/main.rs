fn main() {
    println!("unobserved vector and some types");
    let x = vec![1, 3, 4];
    for m in x {
        println!("printing the vector piece by piece{:?}", m);
        // println!(
        //     "observing the vector losing its ownership of items; {:?}",
        //     x
        // );
    }
    /*
       lets make a vector with some type values and
    then check the for in loop and also observe the
    vector at the same time
    *
    *
    *
    * */

    let v = vec![Some(1), Some(2), Some(3)];
    for s in v {
        println!("accessing the vector here piece by piece{:?}", s);
    }
    //println!("{:?}",v);
    /*using std::mem::replace on an unmutable vector*/

    let vx = vec![Some("help"), Some("meta")];
    std::mem::replace(&mut vx[0], None);
}
