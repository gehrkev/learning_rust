mod list {

    pub struct Tasks {
        pub item: String,
    }

    //cargo install cargo-modules <----- this one proposed by the instructor doesn't work
    //^^^ error[E0063]: missing field `diagnostic_provider` in initializer of `ServerCapabilities`
    //cargo install --locked cargo-modules <------- this one worked
    //cargo modules generate tree
    //cargo modules generate tree --with-types <--- doesn't work
    //cargo modules generate tree --fns --types <--- use this instead
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;
use things_todo::items_completed::test::test_fn;

fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("Tasks"),
    };
    add_activity(); //relative path
    items_completed::remove_task();
    test_fn();
}
