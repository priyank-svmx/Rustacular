fn main() {
    println!("Hello, world!");
    let mut pal = vec!["ekllo", "delat", "meta"];
    pal.reverse();

    let mut vec_with_capacity = Vec::with_capacity(2);
    println!("the length of the vector is {}", vec_with_capacity.len());
    println!(
        "the capacity of the vector {}",
        vec_with_capacity.capacity()
    );
    println!("the reverse of a vector is {:?}", pal);
    // if the below line of code is missig then line no 6 would give a type missing error
    // as there is no information to interpret the type of the vector
    vec_with_capacity.push(2);

    let piece = vec_with_capacity.remove(0);
    println!(
        "checking if the removal returns the value removed {}",
        piece
    );

    println!(
        "after removing the last item checking the capacity {}",
        vec_with_capacity.capacity()
    );
    let added_piece = vec_with_capacity.insert(0, 23);

    // reading from the command line and processing the datat

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
