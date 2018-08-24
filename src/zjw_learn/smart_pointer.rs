use self::List::{ Cons, Nil };
use self::ListRc::{ ConsRc, NilRc };
use self::ListRf::{ ConsRf, NilRf };
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

pub fn run() {
    // let list = Cons(1, 
    //     Box::new(Cons(2, 
    //         Box::new(Cons(3, 
    //             Box::new(Nil))))));
    // println!("{:?}", list);

    // let x = 5;
    // // let y = &x;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // deref, 解引用强制多态
    // let hello = |x: &str| println!("{}", x);
    // let m = MyBox::new(String::from("Hello Rust"));
    // hello(&m[1..4]);

    // drop trait
    // let c = CustomSmartPointer { data: String::from("my stuff") };
    // let d = CustomSmartPointer { data: String::from("other stuff") };
    // drop(c); // 不能调用c.drop(), 会导致 double free错误
    // println!("CustomSmartPointers created.");

    //  rc 引用智能指针
    // 如下不能被编译，因为所有权moved
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    // let b = ConsRc(3, Rc::clone(&a)); // Rc::clone 只会增加引用计数
    // let c = ConsRc(4, Rc::clone(&a));
    // println!("{:?} \n{:?} \n{:?}", a, b, c);

    // let a = RefCell::new([1,2,3,4]);
    // a.borrow_mut()[0] += 10;
    // println!("{:?}", a);

    let value = Rc::new(RefCell::new(5)); // 不可变，且可被公用
    let a = Rc::new(ConsRf(Rc::clone(&value), Rc::new(NilRf)));
    let b = ConsRf(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ConsRf(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Box 用来实现递归
// 不用Box的话，编译器无法判断需要多少空间来存储，Box的大小是固定的，会在堆上分配内存
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 可以共享所有权版本的 cons list。有引用计数的智能指针
#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

// refcell
#[derive(Debug)]
enum ListRf {
    ConsRf(Rc<RefCell<i32>>, Rc<ListRf>),
    NilRf,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// RefCell 与 内部可变性模式
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger, value: 0, max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    
    struct MockMessenger {
        messages: RefCell<Vec<String>>, // 内部可变性模式
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limitTracker = LimitTracker::new(&mock_messenger, 100);
        limitTracker.set_value(80);

        assert_eq!(mock_messenger.messages.borrow().len(), 1);
    }
}