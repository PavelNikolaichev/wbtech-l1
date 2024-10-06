#[derive(Debug)]
struct UnorderedSet<T: Eq> {
    elements: Vec<T>,
}

impl<T: Eq + Clone> UnorderedSet<T> {
    fn new() -> Self {
        UnorderedSet {
            elements: Vec::new(),
        }
    }

    fn add(&mut self, value: T) {
        if !self.elements.contains(&value) {
            self.elements.push(value);
        }
    }

    fn intersection(&self, other: &Self) -> UnorderedSet<T> {
        let mut result = UnorderedSet::new();
        for item in &self.elements {
            if other.elements.contains(item) {
                result.add(item.clone());
            }
        }
        result
    }
}

fn main() {
    let mut set1 = UnorderedSet::new();
    set1.add(1);
    set1.add(2);
    set1.add(3);
    set1.add(4);
    set1.add(5);

    let mut set2 = UnorderedSet::new();
    set2.add(3);
    set2.add(4);
    set2.add(5);
    set2.add(6);
    set2.add(7);

    let set3 = set1.intersection(&set2);

    println!("Set 1: {:?}", set1);

    println!("Set 2: {:?}", set2);

    println!("Intersection: {:?}", set3);
}
