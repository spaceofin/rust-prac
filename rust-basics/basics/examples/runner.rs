#![allow(dead_code)]
#![allow(unused_imports)]

mod custom_types;
mod modules;
mod ownership;
mod traits;
mod macro_rules;
mod generics;
mod async_and_future;
mod concurrency;
mod error_handling;
mod smart_pointers;
mod primitives;
mod closures;
mod variables_and_types;
mod conversion_and_expressions;
mod flow_of_control;

use custom_types::{enums, std_library_types, structs, constants};

fn main() {
    // enums::runner::run();
    // std_library_types::runner::run();
    modules::runner::run();
    // ownership::runner::run();
    // structs::runner::run();
    // traits::runner::run();
    // macro_rules::runner::run();
    // generics::runner::run();
    // async_and_future::runner::run();
    // concurrency::runner::run();
    // error_handling::runner::run();
    // smart_pointers::runner::run();
    // primitives::runner::run();
    // constants::constants_demo();
    // closures::runner::run();
    // variables_and_types::runner::run();
    // conversion_and_expressions::runner::run();
    // flow_of_control::runner::run();
}