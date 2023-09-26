fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block vvvv
    // raw pointers may be null, dangling or unaligned;
    // they can violate aliasing rules and cause data races: all of these are undefined behavior
    unsafe {
        println!("r1 is {:?}", *r1);
        println!("r2 is {:?}", *r2);
    }

}