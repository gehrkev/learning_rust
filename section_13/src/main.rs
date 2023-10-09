use std::cell::RefCell;
use std::rc::Rc;

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}

fn main() {
    let t = (12, "eggs"); //tuples are created on the stack
    let b = Box::new(t); //box pointer, created on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); //deallocation

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); //deallocation

    println!("{:?}", y);

    /*
    Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
    The type Rc<T> provides shared ownership of a value of type T, allocated in the heap.
    Invoking clone on Rc produces a new pointer to the same allocation in the heap.
    When the last Rc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped.
    */
    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    println!("{}, {}, {}", s1.contains("Point"), s2, s3.contains("ter"));

    let flag = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    //borrow returns Ref<T>
    //borrow_mut returns RefMut<T>

    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", reference);

    let mut mut_ref = reference.borrow_mut();
    *mut_ref = false; //dereference first to access inside;
    println!("{}", mut_ref);
}
