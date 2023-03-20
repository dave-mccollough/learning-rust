// Name field struct
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

//Tuple struct
struct Coordinates(i32, i32, i32);


fn main() {
    let user_1 = User {
        active: true,
        username: String::from("Dave"),
        sign_in_count: 0
    };

    println!("{}", user_1.username);

    let user_2 = build_user(String::from("Jennifer"));

    println!("{}", user_2.username)

    let cords = Coordinates(1, 2, 3);
}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }

}