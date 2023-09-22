// #[derive(Debug)]
// struct City {
//     city: String,
//     population: u64,
// }

#[derive(Debug)]
struct Item {
    name: String,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

// fn sort_pop(city: &mut Vec<City>){
//     city.sort_by_key(pop_helper)
// }
//
// fn pop_helper(pop: &City) -> u64{
//     pop.population
// }

// fn sort_pop_closure(pop: &mut Vec<City>) {
//     pop.sort_by_key(|p| p.population)
// }

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|i| i.name == product).collect()
}

fn main() {
    // let a = City {
    //     city: "a".to_string(),
    //     population: 100,
    // };
    // let b = City {
    //     city: "b".to_string(),
    //     population: 57,
    // };
    // let c = City {
    //     city: "c".to_string(),
    //     population: 140,
    // };
    // let d = City {
    //     city: "d".to_string(),
    //     population: 15,
    // };
    // let e = City {
    //     city: "e".to_string(),
    //     population: 70,
    // };
    //
    // let mut vec: Vec<City> = Vec::new();
    //
    // vec.push(a);
    // vec.push(b);
    // vec.push(c);
    // vec.push(d);
    // vec.push(e);
    //
    // // sort_pop(&mut vec);
    // sort_pop_closure(&mut vec);
    // println!("{:?}", vec);
    //
    // let add = |x: i32| -> i32 { x + 1 };
    // let add_v2 = |x| x + 1;
    // add_v2(1);
    //
    // let example = |x| x;
    // let string = example("string".to_string());
    // let num = example(2); // ^^^^ try switching these around

    //Fn, FnMut and FnOnce
    //Fn -- calls multiple times, borrows immutably -- includes all fn functions
    //FnMut -- calls multiple times, borrows mutably
    //FnOnce --

    // |args| v.contains(arg) -- Fn
    // |args| v.push(arg) -- FnMut
    // || drop(v) -- FnOnce

    // let y = 5;
    // let add_y = |x: i32| x + y; //RustRover thought x was an implicit fn() -- had to declare it as i32 to stop erroring out -- still, it ran even with the linting error
    // let copy = add_y; //this is closure being copied;
    // println!("{}", add_y(copy(10)));

    // let mut y = 5;
    // let mut add_y = |x| {y += x; y};
    // let mut copy = add_y; //this is closure being copied;
    // println!("{}", add_y(copy(10)));

    // let vec = vec![1,2,3];
    //
    // for val in vec.iter(){
    //     println!("{}", val);
    // }

    // let vec2 = vec![1,2,3];
    // let mut iter = (&vec2).into_iter();
    //
    // while let Some(v) = iter.next() {
    //     println!("{}", v);
    // }

    let mut vec: Vec<Item> = Vec::new();
    vec.push(Item {
        name: String::from("coat"),
    });
    vec.push(Item {
        name: String::from("shirt"),
    });
    vec.push(Item {
        name: String::from("shorts"),
    });
    vec.push(Item {
        name: String::from("shoes"),
    });

    let checked = check_inventory(vec, "shirt".to_string());
    println!("{:?}", checked);

    let mut range = Range { start: 0, end: 10 };
    // for r in range {
    //     println!("{}", r);
    // }

    let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("{:?}", vec);
}

// pub trait Iterator{
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     //many default methods
// }
