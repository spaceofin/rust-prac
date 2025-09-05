mod my_mod {
    // private
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // public
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
 
        // only visible within the given path. `path` must be a parent or ancestor module
        // In this example, it behaves the same as a private function
        pub(in crate::modules::visibility::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // visible within the current module, same as private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }

    pub fn print_private_nested_functions() {
      private_nested::function();
      private_nested::restricted_function();
    }
}

fn function() {
    println!("called `function()`");
}

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T: std::fmt::Debug> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }

        pub fn print_contents(&self) {
          println!("Contents of ClosedBox: {:?}", self.contents);
        }
    }
}


pub fn visibility_demo() {
  function();
  my_mod::function();

  my_mod::indirect_access();
  my_mod::nested::function();
  my_mod::call_public_function_in_my_mod();

  my_mod::public_function_in_crate();
  my_mod::print_private_nested_functions();

  println!();

  let open_box = my::OpenBox { contents: "public information" };
  println!("The open box contains: {}", open_box.contents);
  
  let closed_box = my::ClosedBox::new("classified information");
  closed_box.print_contents();
}