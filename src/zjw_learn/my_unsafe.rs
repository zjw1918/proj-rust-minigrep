extern "C" {
  fn abs(input: i32) -> i32;
}
/// 通过其他语言调用rust
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Hi i am from rust.");
}

/// 静态变量与常量的区别
/// 静态变量内存地址唯一，且可以式可变的。
/// 读写静态变量需要用unsafe

pub fn run() {
  // 解引用裸指针
  let mut num = 5;
  let p1 = &num as *const i32;
  let p2 = &mut num as *mut i32;

  unsafe {
    println!("p1 is: {}", *p1);
    println!("p2 is: {}", *p2);
  }

  unsafe {
    danger();
  }

  let mut v = vec![1,2,3,4,5,6];
  let r = &mut v[..];
  let (a, b) = r.split_at_mut(3);
  assert_eq!(a, &mut [1,2,3]);
  assert_eq!(b, &mut [4,5,6]);

  // test extern
  unsafe {
    println!("{}", abs(-3));
  }
}

unsafe fn danger() {

}

// 下面的代码，逻辑上式对的，但是借用检查器通不过
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//   let len = slice.len();
//   assert!(mid <= len);
//   (&mut slice[..mid], &mut slice[mid..])
// }

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();
  assert!(mid <= len);
  unsafe {
    (slice::from_raw_parts_mut(ptr, mid), 
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
  }
}

/// 高级生命周期; 生命周期子类型
struct Context<'s>(&'s str);
struct Parser<'a, 's: 'a> {
  context: &'a Context<'s>
}
impl<'a, 's> Parser<'a, 's> {
  fn parse(&self) -> Result<(), &'s str> {
    Err(&self.context.0[1..])
  }
}

fn test_parse(context: Context) -> Result<(), &str> {
  Parser{ context: &context }.parse()
}