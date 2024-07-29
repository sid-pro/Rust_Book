#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T
}

impl<T> Point<T>{
    fn x(&self)->&T{
       &self.x 
    }
}

// strcut with multiple generics
#[derive(Debug)]
struct Point1<T,U>{
    x:T,
    y:U
}

struct Pointer<X1,Y1>{
    x:X1,
    y:Y1
}

impl<X1,Y1> Pointer<X1,Y1>{
    // because pointer 1 is of type X1 and Y1 which is in self now Pointer 2 can be of any type so we define X2 and Y2.
    fn mixup<X2, Y2>(self,other:Pointer<X2,Y2>) -> Pointer<X1,Y2>{
        Pointer{
            x:self.x,
            y:other.y,
        }
    }
}
fn main() {

    // code to find largest number
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    println!("The largest number is {}",largest_number(number_list));

    // but suppose now we have the task a largest number from two list than we than again write the same code for second list
    // which will be duplicated so we can create a function which accepts the list and return largest number

    // now the larget_number will always give us the largest number but what happen if we want the largest character
    let char_list = vec!['y', 'm', 'a', 'q','z'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // so here we can use the generics

    println!("The largest char is {}",get_largest(char_list));


    // generics with structs

    let p1 = Point{
        x:1,y:2
    };

    let p2 = Point{
        x:5.0,y:4.2
    };
    // it will throw an error  because generics should be of same type in Point Struct
    // let p3 = Point{
    //     x:5,y:4.2
    // };

    println!("the point is {:?}",p1);
    println!("the point is {:?}",p2);

    let p3 = Point1{
        x:5,y:4.2
    };

    println!("the point is {:?}",p3);

    println!("p1.x = {}", p1.x());

    let p1 = Pointer { x: 5, y: 10.4 };
    let p2 = Pointer { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

fn largest_number(list:Vec<i32>)->i32{
    let mut largest = list[0];

    for number in list{
        if number >largest{
            largest = number;
        }

       
    }
    largest
}

fn largest_char(list: &Vec<char>) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// it will give error because list can be of any type which is non comapable also so to fix this use traits
fn get_largest<T:PartialOrd+Copy>(list:Vec<T>)-> T{
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}