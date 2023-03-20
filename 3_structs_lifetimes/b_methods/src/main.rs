struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width
    }
}

fn main() {
    let mut sq = Square {
        width: 5,
        height: 5,
    };

    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());
    sq.change_width(10);
    println!("My new width is {}", sq.whats_my_width());
}
