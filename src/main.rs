struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn solve_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    println!("Username: {}", name);

    let user2 = build_user(String::from("test2@test.com"), String::from("test2"));
    println!("user2 {}:", user2.username);

    let rect1 = Rectangle {
        width: 50,
        height: 15,
    };

    println!(
        "The area of the Reactangle {:?} is {}",
        rect1,
        rect1.solve_area()
    );

    let rect2 = Rectangle {
        width: 25,
        height: 7,
    };

    let rect3 = Rectangle {
        width: 55,
        height: 16,
    };

    println!("rect1 can hold rect2: {} ", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {} ", rect1.can_hold(&rect3));

    let sq1 = Rectangle::square(25);
}

// fn solve_area(rect: &Rectangle) -> u32 {
//     return rect.width * rect.height;
// }

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
