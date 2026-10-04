use std::{ rc::Rc, cell::RefCell };

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    let mut current = &list;
    loop {
        match current {
            List::Cons(value, next) => {
                println!("list value: {}", value);
                current = next;
            }
            List::Nil => {
                break;
            }
        }
    }

    let name = Rc::new(String::from("configmaster"));
    let name2 = Rc::clone(&name);

    println!("count: {}", Rc::strong_count(&name));
    println!("name2: {}", name2);

    let count = RefCell::new(0);
    *count.borrow_mut() += 10;
    println!("count value: {:?}", count.borrow());

    let shared = Rc::new(RefCell::new(Vec::<String>::new()));
    shared.borrow_mut().push(String::from("configmaster"));
    println!("shared value: {:?}", shared.borrow());
}
