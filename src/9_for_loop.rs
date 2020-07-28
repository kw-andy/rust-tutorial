
fn main() {

    //1st -> n in 1..11
    //2nd -> let numbers = 30..51;
    //3rd -> vector -> array/list vec!["Rabbit","Dog","Cat"]

    let animals = vec!["Rabbit","Dog","Cat"];

    for (idx,ani) in animals.iter().enumerate() {

        println!("the index of the animal is {} and the animal is the {}",idx,ani);
    }

}