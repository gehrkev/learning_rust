#[derive(Debug)]
struct Car {
    mpg: f32,
    color: String,
    top_speed: f32,
}

impl Car {
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

fn main() {
    let mut car = Car {
        mpg: 24.20,
        color: String::from("Black"),
        top_speed: 100.0,
    };
    println!("{}, {}, {}", car.mpg, car.color, car.top_speed);
    car.set_mpg(25.20);
    car.set_color("Red".to_string());
    car.set_top_speed(120.0);
    println!("{}, {}, {}", car.mpg, car.color, car.top_speed);
    println!("{:?}", car) //#[derive(Debug)]
}
