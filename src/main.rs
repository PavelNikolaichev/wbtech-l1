use std::thread;

fn parallel_squares(n: i32) {
    let nums: Vec<i32> = (1..n+1).collect();

    let mut threads = vec![];

    println!("Квадраты чисел:");

    for num in nums {
        let thread = thread::spawn(move || {
            // Поскольку нам не важен порядок вывода, мы можем вызывать вывод прямо отсюда.
            print!("{} ", num * num);
        });

        threads.push(thread);
    }

    // Ждем завершения всех потоков.
    for thread in threads {
        thread.join().unwrap();
    }

    println!();
}

fn main() {
    parallel_squares(10);

    parallel_squares(20);

    parallel_squares(30);
}


