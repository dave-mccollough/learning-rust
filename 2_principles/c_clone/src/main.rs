fn main() {
    let x = vec!["Dave".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
