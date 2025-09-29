fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Mutable references are possible with lifetimes as well.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// In this case, having the same lifetime for both is fine.
fn print_multi<'a>(x: &'a i32, y: &'a i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// different lifetimes
fn longer_string<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() >= s2.len() { s1 } 
    else { s2 }
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

fn functions_lifetime() {
    let x = 7;
    let y = 9;
    
    print_one(&x);
    print_multi(&x, &y);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    let string1 = String::from("short");
    
    let result;
    let result_cmp;
    {
        let string2 = String::from("longer_string");
        result = longer_string(&string1, &string2);
        println!("The longer string is: {}",result);
        result_cmp = "result_cmp"
        
    } // string2 drop

    // Compile Error:
    // println!("longer: {}",result);
    println!("result_cmp: {}", result_cmp);
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn add_num<'a>(&'a mut self, num: i32) -> &'a i32 {
        self.0 += num;
        &self.0
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn methods_lifetime() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
    owner.add_num(10);
    owner.print();
}

pub fn functions_and_methods_demo() {
    // functions_lifetime();
    methods_lifetime();
}