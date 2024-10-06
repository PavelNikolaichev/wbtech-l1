use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::time::{sleep, Duration as TokioDuration};
use tokio_util::sync::CancellationToken;

fn thread_with_channel_termination() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => println!("Thread received: {}", msg),
                Err(_) => {
                    println!("Channel closed, thread stopping.");
                    break;
                }
            }
        }
    });

    for i in 0..5 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    drop(tx);

    handle.join().unwrap();
}

async fn tokio_task_with_channel_termination() {
    let (tx, mut rx) = tokio_mpsc::channel(100);

    let task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Task received: {}", msg);
        }
        println!("Channel closed, task stopping.");
    });

    for i in 0..5 {
        tx.send(i).await.unwrap();
        sleep(TokioDuration::from_millis(500)).await;
    }

    drop(tx);

    task.await.unwrap();
}

async fn tokio_task_with_cancellation_token() {
    let cancel_token = CancellationToken::new();

    let task_token = cancel_token.clone();

    let task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = task_token.cancelled() => {
                    println!("Task cancelled, stopping.");
                    break;
                },
                _ = sleep(TokioDuration::from_millis(500)) => {
                    println!("Task is running...");
                }
            }
        }
    });

    sleep(TokioDuration::from_secs(2)).await;

    println!("Cancelling task...");
    cancel_token.cancel();

    task.await.unwrap();
}

#[tokio::main]
async fn main() {
    println!("=== Тестирование остановки потока через закрытие канала ===");
    thread_with_channel_termination();

    println!("\n=== Тестирование остановки tokio задачи через закрытие канала ===");
    tokio_task_with_channel_termination().await;

    println!("\n=== Тестирование остановки tokio задачи через CancellationToken ===");
    tokio_task_with_cancellation_token().await;
}
