use std::thread;
use std::time::Duration;
use std::collections::HashMap;

pub fn run() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(
    //     simulated_user_specified_value,
    //     simulated_random_number
    // );

    // let x = vec![1,2,3];
    // let equal_to_x = move |z| z == x;
    // // println!("can't use x here: {:?}", x);
    // let y = vec![1, 2, 3];
    // println!("{}", equal_to_x(y));

    // let v1 = vec![1,2,3];
    // // let v1_iter = v1.iter();
    // let sum: i32 = v1.iter().sum(); // sum 会消费一次迭代器
    // println!("{}", sum);

    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect才能使map代表的迭代器适配器产生效能
    assert_eq!(v2, vec![2,3,4]);
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_result.value(intensity));
        println!("Next do {} situps!",  expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}


// struct Cacher<T> where T: Fn(u32) -> u32 {
//     calculation: T,
//     value: Option<u32>, // 有个问题就是，再也无法变更了，因此推荐使用hashmap
// }

// impl<T> Cacher<T> where T: Fn(u32) -> u32 {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher { calculation, value: None }
//     }
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: HashMap::new() }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            return *self.value.get(&arg).unwrap();
        }
        let v = (self.calculation)(arg);
        self.value.insert(arg, v);
        v
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[test]
fn tierator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
//     shoes.into_iter()
//         .filter(|s| s.size == size).collect()
// }

#[test]
fn test_shoes_filter() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    // let shoes_filtered = shoes_in_my_size(shoes, 10);
    // into_iter 相当于新建了了slice。即把原slice的所有权提取出来。
    let shoes_filtered: Vec<_> = shoes.into_iter()
        .filter(|s| s.size == 10).collect();

    assert_eq!(
        shoes_filtered,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

// 自定义一个结构体的迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}