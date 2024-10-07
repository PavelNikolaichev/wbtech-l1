use rand::random;

fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == target {
            return mid;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

fn main() {
    let mut array: Vec<i32> = (0..10).map(|_| i32::abs(random::<i32>() % 100)).collect();
    array.sort();

    let target = array[i32::abs(random::<i32>() % 10) as usize];

    println!("Сгенерированный массив: {:?}", array);
    println!("Искомый элемент из массива: {}", target);

    let result = binary_search(&array, target);
    if result != -1 {
        println!("Элемент найден под индексом: {}", result);
    } else {
        println!("Элемент не найден");
    }

    let target = 101; // out of range

    println!("Искомый элемент из массива: {}", target);
    let result = binary_search(&array, target);
    if result != -1 {
        println!("Элемент найден под индексом: {}", result);
    } else {
        println!("Элемент не найден");
    }
}
