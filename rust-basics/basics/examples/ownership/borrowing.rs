fn print_msg(msg: String) {
    println!("{}",msg);
}

fn print_and_return_msg(msg: String) -> String {
    println!("{}",msg);
    return msg;
}

fn print_msg_borrowed(msg: &String) {
    println!("{}",msg);
}

fn gen_greeting_msg(name: &str) -> String {
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    return greeting;
}

fn append_exclamation(s: &mut String) {
    s.push_str("!");
}

fn add_one(num: &mut i32) {
    *num += 1;
}


pub fn borrowing() {
    let s1 = String::from("s1: Just print message");
    print_msg(s1);
    // compile error: s1 has been moved to print_msg
    // println!("{}",s1);

    let s2 = String::from("s2: Print message and return its ownership");
    let s2_returned = print_and_return_msg(s2);
    println!("s2_returned - {}",s2_returned);
    
    let s3 = String::from("s3: Print message by bowrrowing");
    println!("{}",s3);

    // mutable reference
    let mut greeting = gen_greeting_msg("rust");
    append_exclamation(&mut greeting);
    println!("{}",greeting);

    // dereference
    let mut num = 10;
    add_one(&mut num);
    println!("{}",num);
}