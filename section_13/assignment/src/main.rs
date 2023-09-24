use std::rc::Rc;

fn main() {
    let stack = 5;
    let heap = Box::new(10);
    let result = stack * *heap;
    println!("Stack: {} * Heap: {} = Result: {}", stack, heap, result);

    let string = String::from("a string");
    {
        let rc = Rc::new(string);
        println!("{}", Rc::strong_count(&rc));
        {
            let rc2 = Rc::clone(&rc);
            println!("{}", Rc::strong_count(&rc));
            println!("{}", Rc::strong_count(&rc2));
        }
        println!("{}", Rc::strong_count(&rc));
    }
    // println!("rc_value: {}", string); //moved to rc inside the code block, dropped after the block ended
}
