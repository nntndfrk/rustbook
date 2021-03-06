#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    /*
    !Note that we needed to make v1_iter mutable:
        calling the next method on an iterator changes
        internal state that the iterator uses to keep
        track of where it is in the sequence.
    */
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// consuming adaptors
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

// iterator adaptors
#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| i32::pow(*x, 2)).collect();

    assert_eq!(v2, vec![1, 4, 9])
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 13, style: "sandal".to_string() },
        Shoe { size: 10, style: "boot".to_string() },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: "sneaker".to_string() },
            Shoe { size: 10, style: "boot".to_string() },
        ]
    );
}

// Creating Our Own Iterators with the Iterator Trait

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}


#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}




fn main() {}
