// && = and
//  || = or 
// list of operators here -> https://doc.rust-lang.org/book/appendix-02-operators.html
// flow control : if/else if/else -> https://doc.rust-lang.org/book/appendix-02-operators.html
// <=,=,==,=>,>,>=

fn main() {

    let x:u32 = 2;

    if x <= 4 {

        println!("{} is equal or smaller than 4",x);
    } 
    else if x >4 && x < 8 {

        println!("{} is between 4 and 8",x);
    }
    
    else {

        println!("{} is greater than 8",x);

    }

}