
const INACTIVE_USERS: i32 = 12;
fn main() {
    let mut user_count = 25;
    let admin = 2;
    print!("There are {} users and {} admins currently in the system.\n", user_count, admin);
    user_count = user_count - INACTIVE_USERS;
    print!("There are {} active users currently in the system.\n", user_count);
}
