fn main() {
    // Method one of creating an instance of a stryct
    let mut user1 = User {
        email: String::from("amogus"),
        username: String::from("guy"),
        active: true,
        sign_in_count: 1,
    };

    // entire instance has to be mutable to edit something
    user1.email = String::from("epic guy");

    // struct update syntax

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("platasdadaas"),
        sign_in_count: user1.sign_in_count,
    };

}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}