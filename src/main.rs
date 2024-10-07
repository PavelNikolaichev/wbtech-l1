use std::any::Any;

fn get_type<T: Any>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    get_type(42);
    get_type("Hello");
    get_type(3.14);
    get_type(true);
    get_type('A');
    get_type([1, 2, 3]);
    get_type((1, 2, 3));
    get_type(Some(42));
    get_type(Box::new(42));
    get_type(String::from("Hello"));
    get_type(vec![1, 2, 3]);
    get_type(std::collections::HashMap::<i32, i32>::new());
    get_type(std::collections::HashSet::<i32>::new());
}