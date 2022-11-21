struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User {
        email: String::from("mladenovic13@gmail.com"),
        username: String::from("mladenovic13"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new_email@gmail.com");
    let user2 = User {
        email: String::from("user2@gmail.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
