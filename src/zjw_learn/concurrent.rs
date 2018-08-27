use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;

pub fn run() {
  // let (tx, rx) = mpsc::channel(); // 单个生产者
  // let tx1 = mpsc::Sender::clone(&tx);

  // thread::spawn(move || {
  //   let vals = vec!["nihao", "i", "am", "thread"];
  //   for val in vals {
  //     tx.send(val).unwrap();
  //     thread::sleep(Duration::from_secs(1));
  //   }
  // });

  // thread::spawn(move || {
  //   let vals = vec!["nihao2", "i2", "am2", "thread2"];
  //   for val in vals {
  //     tx1.send(val).unwrap();
  //     thread::sleep(Duration::from_secs(2));
  //   }
  // });

  // for rec in rx {
  //   println!("rec: {}", rec);
  // }

  // 在线程间共享 Mutex<T>
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 0..10 {
    let counter = Arc::clone(&counter);  // 原子克隆
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("counter: {}", *counter.lock().unwrap());
}