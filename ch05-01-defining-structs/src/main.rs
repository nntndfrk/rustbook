// Defining Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Defining Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//  with any expression, we can construct a new instance of the struct
fn build_user(email: String, username: String) -> User {
    // Using the Field Init Shorthand when Variables and Fields Have the Same Name
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // Instantiating Structs
    let mut user1 = User {
        email: String::from("some@exp.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("alex@123.io"),
        String::from("alex123"),
    );

    // Creating Instances From Other Instances
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2
    };

    // Tuple Structs without Named Fields
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
