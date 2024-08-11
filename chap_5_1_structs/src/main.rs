struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user_bertha = build_user(
        String::from("berthalicious@birthingberthas.com"),
        String::from("berthablaster425"),
    );
    let user_berthalt = User {
        email: String::from("birthingberthagonnabust@berthmail.com"),
        ..user_bertha
    };
    println!(
        "Hello, world! Bertha Name: {}, Bertha Username: {}, Berthalt Username {}",
        user_bertha.sign_in_count, user_bertha.email, user_berthalt.email
    );
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
