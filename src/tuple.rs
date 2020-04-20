/* 

multi line comments 

things learnt 
05/04/20

multi line comments 
using cargo to initiate new project : cargo new <project_name> --bin
using cargo to run the project without rustc to compile (cargo run)
cargo fmt : to format the code
Tu as le bouton "rustfmt" dans le playground (le lien que je t'ai donné) dans le menu "Tools"
println! means print line next , to print everything on the same line, print!

08/04/20

let mut x
let x

signed and unsigned variables https://learning-rust.github.io/docs/a8.primitive_data_types.html

11/04/20

&&	expr && expr	Logical AND
||	expr || expr	Logical OR

loop

https://www.youtube.com/watch?v=udrjtjKuVh8&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=14

// 14

*/

fn main() {

    let tup1 = (45,7,"Computer");

    let (a,b,c) = tup1;

    println!("a is {}",a);
    println!("b is {}",b);
    println!("c is {}",c);

    let tup1 = (10,"Rust",true,(1,3,4));

    println!(" this test is {}ing", (tup1.3).1);


}

