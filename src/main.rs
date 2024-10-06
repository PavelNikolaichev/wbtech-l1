use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use dashmap::DashMap;

fn mutex_with_hashmap() {
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        threads.push(thread::spawn(move || {
            let mut map = map.lock().unwrap();
            map.insert(i, i * 2);
            println!("Inserted key: {}, value: {}", i, i * 2);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Final map: {:?}", *map.lock().unwrap());

    println!("Concurrent read from HashMap:");
    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        threads.push(thread::spawn(move || {
            let map = map.lock().unwrap();
            let value = map.get(&i);
            match value {
                Some(v) => println!("Key: {}, Value: {}", i, v),
                None => println!("Key: {} not found", i),
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Concurrent read is done");

    println!("Concurrent read-write from HashMap:");
    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        if i % 2 == 0 {
            threads.push(thread::spawn(move || {
                let map = map.lock().unwrap();
                let value = map.get(&i);
                match value {
                    Some(v) => println!("Key: {}, Value: {}", i, v),
                    None => println!("Key: {} not found", i),
                }
            }));
        } else {
            threads.push(thread::spawn(move || {
                let mut map = map.lock().unwrap();
                map.insert(i, i * 3);
                println!("Inserted key: {}, value: {}", i, i * 3);
            }));
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Concurrent read-write is done");
}

fn dashmap_concurrent_write() {
    let map = Arc::new(DashMap::new());
    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        threads.push(thread::spawn(move || {
            map.insert(i, i * 2);
            println!("Inserted key: {}, value: {}", i, i * 2);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Final map: {:?}", map);

    println!("Concurrent read from DashMap:");
    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        threads.push(thread::spawn(move || {
            let value = map.get(&i);
            match value {
                Some(v) => println!("Key: {}, Value: {}", i, v.value()),
                None => println!("Key: {} not found", i),
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Concurrent read is done");

    println!("Concurrent read-write from DashMap:");

    let mut threads = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        if i % 2 == 0 {
            threads.push(thread::spawn(move || {
                let value = map.get(&i);
                match value {
                    Some(v) => println!("Key: {}, Value: {}", i, v.value()),
                    None => println!("Key: {} not found", i),
                }
            }));
        } else {
            threads.push(thread::spawn(move || {
                map.insert(i, i * 3);
                println!("Inserted key: {}, value: {}", i, i * 3);
            }));
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Concurrent read-write is done");
}

fn main() {
    println!("=== Тестирование Mutex с HashMap ===");
    mutex_with_hashmap();

    println!("\n=== Тестирование DashMap ===");
    dashmap_concurrent_write();
}
