use std::io;

#[derive(Debug)]
struct Rectangle {
    length : i32,
    breadth : i32,
}

impl Rectangle {
    fn area(&self) -> i32{
        self.length * self.breadth
    }

    fn does_it_fit(&self, rect : &Rectangle) -> bool{
        rect.length > self.length && rect.breadth > self.breadth
    }

    fn square(size : i32) -> Self {
        Self {
            length : size,
            breadth : size
        }
    }
}

fn main() {
    let mut length = String::new();
    let mut breadth = String::new();
    
    println!("<<<< Enter the Length of the rectangle >>>>");
    io::stdin()
    .read_line(&mut length)
    .expect("Failure to read the length");

    println!("<<<< Enter the Breadth of the rectangle >>>>");
    io::stdin()
    .read_line(&mut breadth)
    .expect("Failure to read the breadth");
    
    let length = length.trim().parse::<i32>().expect("Failure to parse length");
    let breadth = breadth.trim().parse::<i32>().expect("Failure to parse breadth");
    
    let rectangle = Rectangle {length,breadth};
    dbg!(&rectangle);
    let square = Rectangle::square(20);
    dbg!(&square);
    println!("Area of the rectangle is : {}",rectangle.area());
}