fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Trying to access the next element without checking if it exists");
    println!("Next element: {:?}", iter.next().unwrap());
}