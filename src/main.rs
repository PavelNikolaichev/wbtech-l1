use std::thread;
use std::sync::mpsc;

fn parallel_sum_of_squares(n: i32) {
    let nums: Vec<i32> = (1..=n).collect();
    let mut threads = vec![];

    let (tx, rx) = mpsc::channel();

    for num in nums {
        let tx = tx.clone();
        let thread = thread::spawn(move || {
            // Поскольку нам не важен порядок вывода, мы можем вызывать вывод прямо отсюда.
            let square = num * num;

            tx.send(square).unwrap();
        });
        threads.push(thread);
    }

    // Ждем завершения всех потоков.
    for thread in threads {
        thread.join().unwrap();
    }

    let res: i32 = rx.iter().take(n as usize).sum();
    println!("{}",  res);
}

fn main() {
    parallel_sum_of_squares(10);
    parallel_sum_of_squares(10);

    parallel_sum_of_squares(20);
    parallel_sum_of_squares(20);

    parallel_sum_of_squares(30);
    parallel_sum_of_squares(30);
}


