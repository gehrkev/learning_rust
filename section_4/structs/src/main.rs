struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32,i32,i32);

struct UnitStruct;

fn main() {
    let user1 = User{active: true, username: String::from("Vitor"), sign_in_count: 0};
    println!("{}",user1.username);

    let user2 = build_user("Vitor2".to_string());
    println!("{}",user2.username);

    let cords = Coordinates(1,2,3);

    // 1..5, .. -> Range {start: 1, end: 5}

}

fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}
