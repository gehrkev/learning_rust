fn main() {
    // let var = 1; //fixed size - on the stack
    // let mut s = "Hello".to_string(); //strings can change in size - on the heap
    // s.push_str((",world"));

    // let x = vec!["vitor".to_string()];
    // let y = x;
    // let z = y;
    // // println!("{:?}", x); //error[E0382]: borrow of moved value: `x`
    // // println!("{:?}", y); //error[E0382]: borrow of moved value: `y`
    // println!("{:?}", z);

    // let x = vec!["vitor".to_string()];
    // let y = x.clone(); //clone is expensive!
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // let x = 1;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // let s = String::from("takes"); //create a var with a string "takes"
    // takes_ownership(s); //give ownership to the fn
    // // // println!("{}",s); //ownership error if tried;
    // let val = 1;
    // make_copy(val);
    // println!("{}", val); //i32 implements copy trait, no error
    //
    // let str1: String = give_ownership();
    // println!("{}",str1);
    //
    // let str3: String = take_and_give(str1);
    //
    // // println!("{}",str1); //ownership given to str3
    // println!("{}",str3);
    //
    // if(true) {
    //     // let str4 = str3;
    // } else {
    //     // let _str5 = str3;
    // }
    // println!("{}",str3);

    // let mut str1 =  String::from("Vitor");
    // let mut str2: String;
    //
    // loop {
    //     str2 = str1; //error[E0382]: - str1 loses ownership since it's moved to str2 after the 1st iteration
    // }

    // let s = String::from("hello");
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s)
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn takes_ownership(s: String) {
//     let string = s;
//     println!("{}", string);
// }
//
// fn make_copy(one: i32) {
//     let val1 = one;
//     println!("{}", val1);
// }
//
// fn give_ownership() -> String {
//     "given".to_string()
// }
//
// fn take_and_give(str2: String) -> String {
//     str2
// }

//out of main-
//var is dropped, s is dropped
