fn main() {

    let mut x = 11;


    //let xr = &x;
    {

        let dom = &mut x;

        *dom += 1;
    

    }


    println!("x is {}",x);

}
