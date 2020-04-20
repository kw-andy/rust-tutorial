fn main() {

    print_numbers(50)


}

fn print_numbers(nums: u32) {

    for num in 1..nums {

        if is_even(num) {
            println!("{} is even",num);
        }

        else {
            println!("{} is odd",num);

        }

    }

}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}



