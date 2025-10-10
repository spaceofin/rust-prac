use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("type: {}", type_name::<T>());
}

fn get_type_name<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn copy_types() {
    let num1 = 10;
    let num2 = num1;
    println!("num1: {} - type {}", num1, get_type_name(&num1));
    println!("num2: {} - type {}", num2, get_type_name(&num2));

    let num3: u32 = 20;
    let num4 = num3;
    println!("num4({}) copied from num3({}) - type u32",num4,num3);

    let num5: f64 = 1.5;
    let num6 = num5;
    println!("num6({}) copied from num5({}) - {}",num6,num5, get_type_name(&num6));

    let flag1 = true;
    let flag2 = flag1;
    println!("flag2({}) copied from flag1({}) - {}",num2,num1, get_type_name(&flag2));

    let char1 = 'r';
    let char2 = char1;
    println!("char2({}) copied from char1({}) - {}",char2,char1, get_type_name(&char2));

    let tuple1 = (1,2);
    let tuple2 = tuple1;
    println!("tuple2({:?}) copied from tuple1({:?}) - {}",tuple2,tuple1, get_type_name(&tuple2));

    //compile error: move, not copy - type (i32, String)
    //let tuple3 = (1,"hello".to_string());
    //let tuple4 = tuple3;
}