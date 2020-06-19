/*

https://www.youtube.com/watch?v=udrjtjKuVh8&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=14

// outside of tutorial. Learning to use rand

*/
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    //let mut rng = rand::thread_rng();
    if rng.gen() {
        // random bool
        println!("i32 is the following: {}, u32 is the following: {}", rng.gen_range(0, 7), rng.gen_range(0, 7))
    }
}
