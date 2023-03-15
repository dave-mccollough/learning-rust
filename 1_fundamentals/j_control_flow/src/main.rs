fn main() {
    println!("{}", if_statements(6));
    if_two(2);
    println!("{:?}", if_five(1));
    // infinite_loop();
    loopy();

}

// If 
fn if_statements(number: u64) -> bool {
    if number > 5 {
        true
    } else {
        false
    }
}

// Else
fn if_two (number: u64) {
    if number == 2 {
        println!("The number {} is correct!", number);
    } else {
        println!("The number is incorrect");
    }
}

// Else if
fn if_five (number: u64)  {
    if number > 5{
        println!("Number is too high!");
    } else if number == 5 {
        println! {"Number is correct"};
    } else {
    println!("Number is too low");
    }
}

// Infinite Loop
fn infinite_loop () {
    loop {
        println!("Infinite Loop!");
    }
}

// Loop
fn loopy () {
    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;

        loop {
            println!("Decrease: {}", decrease);
            if decrease ==  4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }
}
