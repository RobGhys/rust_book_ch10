
// params: list -> a reference to a slice of i32 values
fn largest_i32(list: &[i32]) -> &i32 {
    // Get a pointer to the 0-th value of 'list'
    let mut largest = &list[0];

    // item is a reference to an i32
    for item in list {
        // we have two pointers -> Rust automatically dereferences them and checks the values ...
        // ... pointed by those variables
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//
//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only define methods for Point<f32, f32>
impl Point<f32, f32> {
    fn dist_from_o(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }    
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news. {}", item.summarize());
}

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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let nb_list = vec![32, 2, 3, 69];
    let result = largest_i32(&nb_list);
    println!("Largest number is: {result}");

    let nb_list = vec![99999, 1, 7, 912, 8888, 69];
    let result = largest_i32(&nb_list);
    println!("Largest number is: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 15};
    let float = Point {x: 1.2, y: 4.9};
    let int_and_float = Point {x: 2.0, y: 4};

    println!("integer.x = {}", integer.x());
    println!("dist from origin = {}", float.dist_from_o());

    let char_point = Point { x: "Coucou", y: "Byebye" };
    let p_mixed = int_and_float.mixup(char_point);
    println!("This is magic >>> {}, {}", p_mixed.x, p_mixed.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    returns_summarizable();

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
