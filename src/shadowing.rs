fn main() {

    let mut x = 10;

    { //block that is called shadowing
        x = 2;

    }

    println!("the number is {}",x);


}