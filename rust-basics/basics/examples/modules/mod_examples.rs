use crate::modules::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

use super::deeply::nested::my_third_function as third_function;

fn my_third_function() {
    println!("my_third_function in the same file was called.");
}

fn function() {
    println!("called `function()` in the same file.");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope.
        // They call the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::modules::cool::function as root_function;
            root_function();
        }
    }
}

pub fn mod_demo() {
    my_first_function();
    my_second_function();
    my_third_function();

    println!("\n<Entering block>");
    {
        // This is equivalent to `use super::deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::modules::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();
        println!("Leaving block");
    }
    function();

    println!();
    my::indirect_call();
}