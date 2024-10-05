use std::time::Duration;
use tokio::{task, signal};

async fn create_workers(n: i32, rx: flume::Receiver<String>) {
    let mut threads = Vec::new();

    for id in 0..n {
        let rx = rx.clone();

        threads.push(task::spawn(async move {
            loop {
                match rx.recv_async().await {
                    Ok(data) => println!("Worker {} got: {}", id, data),
                    Err(_) => {
                        println!("Worker {} is closing...", id);
                        break;
                    }
                }
            }
        }));
    }

    for thread in threads {
        thread.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Введите количество воркеров: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    println!("Введите время ожидания между отправкой данных(в миллисекундах): ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let duration: u64 = input.trim().parse().unwrap();

    println!("Введите кол-во произвольных данных: ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let n_iters: i32 = input.trim().parse().unwrap();

    let (tx, rx) = flume::unbounded();

    let worker = tokio::spawn(create_workers(n, rx));

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 0..n_iters {
            println!("{i}. Введите данные: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            tx_clone.send_async(input.trim().to_string()).await.unwrap();
            tokio::time::sleep(Duration::from_millis(duration)).await;
        }
    });

    signal::ctrl_c().await.unwrap();
    println!("Получен Ctrl+C. Завершение работы...");

    drop(tx);

    worker.await.unwrap();
    println!("Все воркеры завершили работу.");
}


