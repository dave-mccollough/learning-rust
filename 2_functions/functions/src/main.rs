fn main() {
    let qty = 4;
    let weight = 7;
    {
        let total = calculate(qty, weight);
        println!("Total is {}", total);
    }
}

fn calculate(x: i32, y: i32) -> i32 {
    x * y
}


