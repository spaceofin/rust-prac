
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct UserRef<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct UserStatic {
    active: bool,
    username: &'static str,
    email: &'static str,
    sign_in_count: u64,
}

fn print_user<T: std::fmt::Debug>(user: &T) {
    println!("{:#?}", user);
}

pub fn print_users() {

    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.username = String::from("anotheremail@example.com");

        print_user(&user1);

        let user1_username_addr_inside = user1.username.as_ptr();
        println!("user1.username pointer inside scope: {:?}", user1_username_addr_inside);
    
        let same_value_as_user1_username = String::from("someusername123");
        let same_value_as_user1_username_addr_inside = same_value_as_user1_username.as_ptr();
        println!("pointer of string with same value as user1.username: {:?}", same_value_as_user1_username_addr_inside);
    }
    println!();

    let user4_username_outside = String::from("outsideusername123");
    let user4_email_outside = String::from("outsideeamil@example.com");

    let user2;
    let user3;
    let user4;
    {
        let temp_username = String::from("tempusername123");
        let temp_email = String::from("tempemail@example.com");

        user2 = UserRef {
            active: true,
            username: &temp_username,
            email: &temp_email,
            sign_in_count: 1,
        };

        user3 = UserRef {
            active: true,
            username: "tempusername123",
            email: "tempemail@example.com",
            sign_in_count: 1,
        };

        user4 = UserRef {
            active: true,
            username: &user4_username_outside,
            email: &user4_email_outside,
            sign_in_count: 1,
        };

        print_user(&user2);
        print_user(&user3);
        print_user(&user4);
        
        let user2_username_addr_inside = user2.username.as_ptr();
        println!("user2.username pointer inside scope: {:?}", user2_username_addr_inside);
        let user3_username_addr_inside = user3.username.as_ptr();
        println!("user3.username pointer inside scope: {:?}", user3_username_addr_inside);
        let user4_username_addr_inside = user4.username.as_ptr();
        println!("user4.username pointer inside scope: {:?}", user4_username_addr_inside);
    };
    println!();

    // compile error: borrowed value does not live long enough
    // println!("user2.username: {}",user2.username);
    println!("user3.username: {}",user3.username);
    println!("user4.username: {}",user4.username);
    println!();

    let user3_username_literal_outside: &'static str = "tempusername123";
    let user3_username_literal_addr_outside = user3_username_literal_outside.as_ptr();
    println!("pointer of literal string with same value as user3.username outside scope: {:?}", user3_username_literal_addr_outside);

    let user4_username_outside_addr = user4_username_outside.as_ptr();
    println!("user4_username pointer outside scope: {:?}", user4_username_outside_addr);

    println!();

    {
        let user5 = UserStatic {
            active: true,
            username: "someusername123",
            email: "someone@example.com",
            sign_in_count: 1,
        };

        print_user(&user5);
    
        let user5_username_addr_inside = user5.username.as_ptr();
        println!("user5.username pointer inside scope: {:?}", user5_username_addr_inside);
    };

    let user5_username_literal_outside: &'static str = "someusername123";
    let user5_username_literal_addr_outside = user5_username_literal_outside.as_ptr();

    println!("pointer of literal string with same value as user5.username outside scope: {:?}", user5_username_literal_addr_outside);
    println!();

}
