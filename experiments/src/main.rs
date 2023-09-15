struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("running experiment on mut and ref types");
    /* this is a mutable variable , it means later on can drop the owenership and hold a new
     ownership
    */ 
    let mut pt = Point { x: 20, y: 32 };
    pt.x = 32;
    pt = Point { x: 53, y: 33 };

    /*
    * Now lets make an unmutable
    * variable and pass its ownership to a new vairable
    * */
    let ptr = Point {x: 34, y: 43};
    let ztr = ptr;
    //ownerhip is moved to a parameter in the function being called here 
    //log_a_point( ztr);
  //  log(ztr.x);

    /*
    * now we can create refrences and see how
    * they behave when the owenrship is changed
    * */
   let ref_ptr = &ztr; 
   let but_ptr = &ztr;

//   println!("{}",ref_ptr == but_ptr);

    println!("{}", ref_ptr.x);
    //println!("{}",&ztr);
    let ref_ztr = ref_ptr;
    println!("{}",ref_ptr.x);

}


// here the values are copied
fn log(x: i32) {
    println!("{}",x);
}

//here the ownership is passed on to the new variable 
fn log_a_point(ptr: Point){
    println!("{}",ptr.x);
}
