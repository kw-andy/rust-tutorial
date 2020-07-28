// loop -> you don't need a condition to stop
// it can carry on as long as they are no break

fn main() {

    let mut n = 0;

    loop{

        n += 1;

        if n == 7 {

            continue;
            
        }

        if n >= 10 {

            break;   
         

        }

        println!("the value of n is {}",n);
    }

    println!("n reached {}",n);

}
