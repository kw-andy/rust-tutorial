fn main() {

    let dishes = vec!["Pizza","tacos","dim sums"];

    for (index,dish) in dishes.iter().enumerate() {
        println!("the dish name is the following {} and it's index number is that one {}",dish,index);
    }


    let mut num_range = 1..50;

    for val in num_range {
        println!("this is the {} time",val);
    }
     

}