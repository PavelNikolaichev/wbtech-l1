use rand::random;

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);

    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut index = 0;

    for i in 0..arr.len() - 1 {
        if arr[i] < pivot {
            arr.swap(index, i);
            index += 1;
        }
    }
    arr.swap(index, arr.len() - 1);

    index
}

fn main() {
    let mut array: Vec<i32> = (0..10).map(|_| i32::abs(random::<i32>() % 100)).collect();
    let mut sorted_array = array.clone();

    println!("Сгенерированный массив: {:?}", array);

    quicksort(&mut sorted_array);
    array.sort();

    println!("Массив, отсортированный quick sort: {:?}", sorted_array);
    println!("Массив, отсортированный  `.sort()`: {:?}", array);
}
