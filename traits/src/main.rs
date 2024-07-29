// A trait defines functionality a particular type has and can share with other types.
//  We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a
//  generic type can be any type that has certain behavior. for example we covered in generic type where we comparing two variable
// which can be comparable type

// traits are most likely interfaces in java
// structs are like classes

pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply:bool,
    pub retweet:bool,
}

// shared behvaiour between news articles and tweets (means same method)

pub trait Summary{
    // traits does not have implementation
  
    fn summarize(&self) -> String;

    // we can define the default implementation and the strcuts can override that if want
    // like this
    // fn summarize(&self) -> String{
    //     String::from("Read more...")
    // }
}

//By using impl Summary for the return type, we specify that the returns_summarizable function
// returns some type that implements the Summary trait without naming the concrete type. In this case, 
// returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// now implementations for our traits for different structs
impl Summary for NewsArticle{
    fn summarize(&self) ->String{
        //format! returns a String which you can store in a variable or use in further processing.
        //println! returns (), as its primary purpose is to print to the console.

        format!("{}, by {}", self.headline, self.author)
    }
}


// if we dont have default implementation than it necessary to define implementations in the implement block of structs
impl Summary for Tweet{
    fn summarize(&self) ->String{
        format!("{}: {}", self.username, self.content)
    }
}


// traits as parameters here we have function which takes item which is a refrence of something that implements Summary
pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn main() {
   
   let tweet = Tweet{
    username:String::from("John Doe"),
    content: String::from("I'm learning Rust! #rustlang"),
    reply: false,
    retweet: false,
   };

   let article = NewsArticle{
    author: String::from("Alice Johnson"),
    headline: String::from("Breaking News! U.S. Marines on Mars"),
    content: String::from("We've discovered a new planet in our solar system! #mars"),
   };

   println!("Summary for tweet: {}", tweet.summarize());

   println!("Summary for article: {}", article.summarize());

   notify(&article);

   println!("{}",returns_summarizable().summarize());
}
