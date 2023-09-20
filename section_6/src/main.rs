// struct Point<T, U> { //generics
//     x: T, //float or char
//     y: U, //i32 or char, et
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a Rust course!")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self){
        println!("Dropping: {}", self.author)
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course{
    // fn overview(&self) -> String {
    //     format!("{}, {}", self.author, self.headline)
    // }
}

// impl Overview for AnotherCourse{
//     fn overview(&self) -> String {
//         format!("{}, {}", self.author, self.headline)
//     }
// }

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
    where
    T: Add<Output = T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }
    }

fn main() {

    let coord = Point{x: 5.0, y: 5.0};
    let coord2 = Point{x: 1.0, y: 2.0};

    let sum = coord + coord2;
    println!("{:?}", sum);
    // let coord = Point{x: 5.0, y: 5.0};
    // let coord2 = Point{x: 'x', y: 5.0};

    // let course1 = Course{
    //     headline: String::from("Headline"),
    //     author: String::from("Vitor"),
    // };
    //
    // let course2 = AnotherCourse{
    //     headline: String::from("Another Headline"),
    //     author: String::from("Another Vitor"),
    // };

    //course1 went of scope -- still uses the implementation even if vvvv is commented.
    // drop(course1);

    // println!("{}", course1.overview()); //default impl
    // println!("{}", course2.overview()); //impl for anothercourse
    // call_overview(&course1);
    // call_overview(&course2);



}
// fn call_overview(item: &impl Overview){
fn call_overview<T: Overview>(item: &T){
    println!("Overview: {}", item.overview())
}

// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + Another Trait>(item1: &T, item2: &T)

trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self){
        *self = source.clone()
    }
}

// //Implementing Copy vvv
// #[derive!(Copy, Clone)]
// struct MyStruct;
//
// //or vvv
// struct MyStruct;
//
// impl Copy for MyStruct { }
//
// impl Clone for MyStruct {
//     fn clone(&self) -> MyStruct {
//         *self
//     }
// }

//From and Into
//TryFrom and TryInto