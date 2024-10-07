use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Counter {
    count: Mutex<i32>
}

impl Counter {
    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    fn get(&self) -> i32 {
        let count = self.count.lock().unwrap();
        *count
    }
}

fn main() {
    let counter = Arc::new(Counter { count: Mutex::new(0) });
    let mut threads = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        threads.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Результат: {}", counter.get());
}