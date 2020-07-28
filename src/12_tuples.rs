//12 tuples


fn main () {

    let tup1 =(20,25,35,58);
    let tup2 =("Rusty",1,5,4,75);
    let tup3 =(1,2,2,(4,"Linux"));

    let (a,b,c,d) = tup1;

    println!("{} and {}",tup1.3,tup2.0);
    println!("This is {}",(tup3.3).1);

    println!("a is {}, b is {} and c is {}",a,b,c);



}