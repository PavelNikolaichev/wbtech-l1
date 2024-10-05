use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn create_workers(n: i32, rx: mpsc::Receiver<String>) {
    // Без мьютекса не уверен, что возможно реализовать чтение (((
    let rx = Arc::new(Mutex::new(rx));

    for id in 0..n {
        let rx = std::sync::Arc::clone(&rx);

        thread::spawn(move || {
            loop {
                let data = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };

                match data {
                    Ok(data) => println!("Worker {} got: {}", id, data),
                    Err(_) => break, // Если канал закрыт, то завершаем поток. Не уверен, насколько это правильно, но все же.
                }
            }
        });
    }
}

fn main() {
    println!("Введите количество воркеров: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    println!("Введите время ожидания между отправкой данных(в миллисекундах): ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let duration: u64 = input.trim().parse().unwrap();

    let (tx, rx) = mpsc::channel();
    create_workers(n, rx);

    input.clear();

    println!("Введите кол-во произвольных данных: ");
    std::io::stdin().read_line(&mut input).unwrap();

    let n_iters: i32 = input.trim().parse().unwrap();
    input.clear();

    for i in 0..n_iters {
        println!("{i}. Введите данные: ");
        std::io::stdin().read_line(&mut input).unwrap();

        tx.send(input.trim().to_string()).unwrap();
        thread::sleep(Duration::from_millis(duration));

        input.clear();
    }
}


