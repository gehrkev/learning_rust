#[derive(Debug)]
struct Car {
    mpg: f32,
    color: String,
    top_speed: f32,
}

#[derive(Debug)]
struct Motorcycle {
    mpg: f32,
    color: String,
    top_speed: f32,
}

pub trait Set {
    fn set_mpg(&mut self, new_mpg: f32);

    fn set_color(&mut self, new_color: String);

    fn set_top_speed(&mut self, new_top_speed: f32);
}

impl Set for Car {
    fn set_mpg(&mut self, new_mpg: f32) {
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: f32) {
        self.top_speed = new_top_speed;
    }
}

impl Set for Motorcycle {
    fn set_mpg(&mut self, new_mpg: f32) {
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: f32) {
        self.top_speed = new_top_speed;
    }
}

use std::fmt::Debug;

fn print<T: Debug>(value: T){
    println!("{:?}", value);
}

fn main() {
    let i = 42;
    let string = "Hi".to_string();
    let v = vec![0,1,2];

    print(i);
    print(string);
    print(v);

    let mut car = Car {
        mpg: 24.20,
        color: String::from("Black"),
        top_speed: 100.0,
    };
    let mut bike = Motorcycle {
        mpg: 44.00,
        color: String::from("Red"),
        top_speed: 100.0,
    };
    println!("{:?}", car); //#[derive(Debug)]
    println!("{:?}", bike); //#[derive(Debug)]
    car.set_mpg(25.20);
    car.set_color("Red".to_string());
    car.set_top_speed(120.0);
    bike.set_mpg(46.10);
    bike.set_color("Green".to_string());
    bike.set_top_speed(140.0);
    println!("{:?}", car); //#[derive(Debug)]
    println!("{:?}", bike); //#[derive(Debug)]
}
