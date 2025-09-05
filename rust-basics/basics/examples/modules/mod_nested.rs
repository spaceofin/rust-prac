pub mod nested_mod1 {
    pub fn print_msg(){
        println!("nested_mod1");
    }
}

pub mod nested_mod2{
    pub fn print_msg(){
        super::nested_mod1::print_msg();
        println!("nested_mod2");
    }
}