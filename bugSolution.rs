fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();

    if let Some(value) = iter.next() {
        println!("{:?}", value);
    }
    if let Some(value) = iter.next() {
        println!("{:?}", value);
    }
    if let Some(value) = iter.next() {
        println!("{:?}", value);
    } else {
        println!("Iterator exhausted");
    }
} 