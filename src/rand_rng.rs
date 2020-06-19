use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}",format!("{}",rng.gen_bool(0.7)));

}
