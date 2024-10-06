use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Введите кол-во чисел: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        for i in 1..=n {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let thread2 = thread::spawn(move || {
        loop {
            let received = rx.recv();
            match received {
                Ok(v) => {
                    tx2.send(v * v).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    let thread3 = thread::spawn(move || {
        loop {
            let received = rx2.recv();

            match received {
                Ok(v) => {
                    println!("Получено: {}", v);
                }
                Err(_) => {
                    println!("Поток чтения завершен");
                    break;
                }
            }
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}
