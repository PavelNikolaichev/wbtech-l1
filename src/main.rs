fn remove_element_by_index<T: Clone>(vec: & mut Vec<T>, index: usize) {
    if index >= vec.len() {
        return;
    }

    for i in index..vec.len() - 1 {
        vec[i] = vec[i + 1].clone();
    }

    vec.pop();
}

fn main() {
    let mut vec = vec![10, 20, 30, 40, 50];

    println!("Вектор: {:?}", vec);

    remove_element_by_index(&mut vec, 2);
    println!("Удаляем элемент по индексу 2: {:?}", vec);

    remove_element_by_index(&mut vec, 0);
    println!("Удаляем элемент по индексу 0: {:?}", vec);

    remove_element_by_index(&mut vec, 2);
    println!("Удаляем элемент по индексу 2: {:?}", vec);
}
