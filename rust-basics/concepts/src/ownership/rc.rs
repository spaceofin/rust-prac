use std::rc::Rc;

fn compare_rc_and_ref() {
    println!("-----Immutable Reference-----");
    let refs: Vec<&i32>;
    {
        let val = 10;
        let ref1 = &val;
        let ref2 = &val;

        println!("val: {val}, ref1: {ref1}, ref2: {ref2}");
        refs = vec![ref1, ref2];
    } // `val` is dropped here.
    // Compile Error: `refs` contains references to a value that has been dropped.
    // println!("refs: {refs:?}");

    println!("-----Reference Counting-----");
    let owners: Vec<Rc<i32>>;
    {
        let val = Rc::new(10);
        let rc1 = Rc::clone(&val);
        let rc2 = Rc::clone(&val);

        println!("val: {val}, rc1: {rc1}, rc2: {rc2}");
        // `rc1` and `rc2` are moved into `owners` (no cloning, ref count unchanged).
        println!("Reference Count Initial: {}", Rc::strong_count(&rc1));
        owners = vec![rc1, rc2];
        println!("Reference Count After Move: {}", Rc::strong_count(&val));
    } // `val` is dropped here.
    println!("owners: {owners:?}");
    println!("Reference Count After `val` Drop: {}",Rc::strong_count(&owners[0]));
}

pub fn run() {
    compare_rc_and_ref();
}