fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    match iter.next() {
        Some(element) => println!("Next element: {:?}", element),
        None => println!("Iterator is exhausted."),
    }
}