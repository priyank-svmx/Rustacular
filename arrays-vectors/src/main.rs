fn main() {
    println!("arrays and vectors in rust");

    let lazy_carter: [u32; 6] = [1, 3, 5, 33, 4, 3];
    let taxonomy = ["animale", "arthropoda"];

    let mut sieve = [true; 1000];

    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 1000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    //println!("printing the array {:?}", sieve);
    testing_the_vectors();
}

fn testing_the_vectors() {
    let mut primes = vec![1, 3, 5, 7, 11];

    primes.push(11);
    primes.push(13);

    let mut vek = Vec::new();

    vek.push(23);
    vek.push(43);
    vek.push(45);
    println!("printing a vector {:?}", vek);

    let mut vv: Vec<u32> = (0..5).collect();
    println!("printing a vector made using iterator {:?}", vv);
}
