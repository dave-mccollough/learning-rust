// fn main() {
//     let phrase_type = "Function";
//     print_phrase(phrase_type);
// }

// fn print_phrase (phrase: &str) {
//     println!("Hello from our {}!", phrase);
// }


fn main (){
  println!("{}", greatest_common_denominator(20, 5));
  println!("{}", multiple_return_types(false));
}

fn greatest_common_denominator(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
} 

fn multiple_return_types(flag: bool) -> bool {
    if flag == true {
        true
    } else {
        false
    }
}