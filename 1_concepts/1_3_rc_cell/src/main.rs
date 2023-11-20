use std::cell::RefCell;
use std::rc::Rc;

use std::fmt;
use std::fmt::Debug;

struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T: Debug> Debug for GlobalStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalStack {:?}", self.stack.borrow())
    }
}

impl<T> GlobalStack<T> {
    fn new() -> Self {
        GlobalStack {
            stack: Rc::new(RefCell::new(Vec::new())),
        }
    }
    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value)
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        GlobalStack {
            stack: self.stack.clone(),
        }
    }
}

fn main() {
    let stack_1: GlobalStack<u8> = GlobalStack::new();
    let stack_2 = &stack_1;
    let stack_3 = stack_1.clone();

    println!("Initial");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );

    stack_1.push(1);
    println!("Push to stack1");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );
    stack_2.push(2);
    println!("Push to stack2");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );
    stack_3.push(3);
    println!("Push to stack3");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );

    stack_1.pop();
    println!("Pop from stack1");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );
    stack_2.pop();
    println!("Pop from stack2");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );
    stack_3.pop();
    println!("Pop from stack3");
    println!(
        "stack_1: {:?}\nstack_2: {:?}\nstack_3: {:?}\n",
        stack_1, stack_2, stack_3
    );
}
