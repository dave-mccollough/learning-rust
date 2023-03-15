fn main() {
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();

    vec.push("String");
    vec.push("Test");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(4);
    println!("{}", vect.capacity());

    let mut v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
}
