use std::time::Duration;
use tokio::task;
use tokio::sync::mpsc;

async fn send_data(tx: mpsc::Sender<i32>, duration: Duration) {
    let mut i = 0;

    loop {
        if let Err(_) = tx.send(i).await {
            println!("Receiver have been dropped. Exiting...");
            break;
        }

        println!("Worker sent data: {}", i);
        i += 1;

        tokio::time::sleep(duration).await;
    }
}

async fn receive_data(mut rx: mpsc::Receiver<i32>) {
    while let Some(data) = rx.recv().await {
        println!("Worker received data: {}", data);
    }

    println!("Sender have been dropped. Exiting...");
}

#[tokio::main]
async fn main() {
    let mut input = String::new();

    println!("Введите время ожидания между отправкой данных(в миллисекундах): ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let duration: u64 = input.trim().parse().unwrap();

    println!("Введите время работы программы(в миллисекундах): ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let program_duration: Duration = Duration::from_millis(input.trim().parse().unwrap());

    let (tx, rx) = mpsc::channel::<i32>(100);

    let send_task = task::spawn(send_data(tx, Duration::from_millis(duration)));
    let receive_task = task::spawn(receive_data(rx));

    tokio::time::sleep(program_duration).await;

    println!("Время работы программы истекло, завершение работы...");

    send_task.abort();
    receive_task.await.unwrap();
}


