// A struct, or structure, is a custom data type that lets you package together and name multiple 
// related values that make up a meaningful group. If you’re familiar with an object-oriented language, 
// a struct is like an object’s data attributes.

struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}
#[derive(Debug)]
struct Rectangle {
    height:u32,
    width:u32
}

// method in struct
impl Rectangle {
    fn area1(&self)->u32{
       self.height * self.width
    }

    fn can_hold(&self,other:&Rectangle) -> bool{
        self.height > other.height && self.width > other.width
    }
}
fn main() {
    let mut user1 = User{
        email:String::from("test@example.com"),
        username:String::from("test"),
        sign_in_count:1,
        active:true
    };

    println!("user email:{}",user1.email);
    user1.username = String::from("testing");
    println!("updated username:{}",user1.username);
    let user2 = build_user(String::from("user2"),String::from("test2@example.com"));
    println!("user2: {}",user2.email);

    // construct another user with help of previous user
    let user3 = User{
        username: String::from("user3"),
        email:String::from("test3@example.com"),
        ..user2
    };
    println!("user3: {}",user3.email);

    // Using Tuple Structs Without Named Fields to Create Different Types
    struct color(i32,i32,i32);
    let black = color(0, 0, 0);

    // use of struct
    let mut rectangle = Rectangle{
        height:50,
        width:30
    };

    // always pass reference so in future we can change the values
    println!("Area of rectangle: {}",area(&rectangle));
    rectangle.width =40;
    println!("{}",rectangle.width);
    // to print struct
    println!("rectangle is {rectangle:?}");
    println!("Area of rectangle: {}",rectangle.area1());

    let rect1 = Rectangle{
        height:30,
        width:20
    };

    println!("Rectangle can hold rect1: {}",rectangle.can_hold(&rect1));

}

fn build_user(username:String, email:String)->User{
    User {
        username,
        email,
        sign_in_count:1,
        active:true
    }
}

fn area(rectangle:&Rectangle)->u32{
    // if we dont want to write return keyword then dont put semicolons
    rectangle.height* rectangle.width
}
