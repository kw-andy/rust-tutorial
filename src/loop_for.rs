fn main() {

    let mut n =0;

    loop {
        n += 1;

        if n == 7 {
            continue;
        }

        else if n > 15 {
            break;
        }

        println!("The value of of n is {}",n);
    }

    let mut x = 45;

    for num_time in 0..x {
        print!("Hello world! {} ", num_time);
    }

    let x: i8 = -50;

    println!("hello that number --> {}!",x);

}
