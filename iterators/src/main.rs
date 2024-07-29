pub trait Iterator {
    // Associted types

    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn iterator_works() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    // You cannot use v1 after it has been moved by into_iter
    // Therefore, create a new vector for the next test
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);


    let v1_iter = v1.iter();
    // methods inside iterator
    let total:i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Both are the same
    // Iterate over the vector using the iterator returned by v1.iter()
    for value in v1.iter() {
        println!("{}", value); // value is already a reference (&i32)
    }

    for value in v1_iter {
        println!("{}", value);
    }

    // How iterators works
    // All iterators in rust are implemented by Iterator trait
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}