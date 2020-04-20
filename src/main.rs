/* 

https://www.youtube.com/watch?v=udrjtjKuVh8&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=14

// 20 imple key

*/


//struct alawas outside of the function?

struct Rectangle {

    width: u32,
    height:u32

}

impl Rectangle {

    fn print_description(&self) {
        println!("Rectangle {} x {}",self.width,self.height);

    }

    fn is_square(&self) -> bool {

        self.width == self.height
    }

}

fn main() {

    let my_rect = Rectangle {width : 10, height: 20};

    my_rect.print_description();
    
    println!("my rectangle is {}",my_rect.is_square());


}

