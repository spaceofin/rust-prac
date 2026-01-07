use std::rc::Rc;
use std::cell::RefCell;

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

    println!("-----Reference Counted-----");
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

fn compare_rc_refcell_and_mut_ref() {
    println!("-----Mutable Reference-----");
    let mut val = 10;
    {
        let ref1 = &mut val;
        // Compile Error: cannot borrow `val` as mutable more than once at a time.
        // let ref2 = &mut val;

        *ref1 += 10;

        // Compile Error: cannot use `val` because it was mutably borrowed use of borrowed `val`
        // val += 10;

        // Compile Error: cannot borrow `val` as immutable because it is also borrowed as mutable.
        // println!("val: {val}");
        println!("ref1: {ref1}");
    } // ref1 dropped and the borrow is released, and 'val' is unlocked.
    val += 10;
    println!("val: {val}");

    println!("-----Reference Counted Interior Mutability-----");
    let owners: Vec<Rc<RefCell<i32>>>;
    {
        let shared_val = Rc::new(RefCell::new(10));
        let rc1 = Rc::clone(&shared_val);
        let rc2 = Rc::clone(&shared_val);

        println!("shared_val: {shared_val:?}, rc1: {rc1:?}, rc2: {rc2:?}");
        owners = vec![rc1, rc2];
        println!("owners in block: {owners:?}");

        let mut ref_mut1 = shared_val.borrow_mut();
        // Runtime Error: RefCell already borrowed.
        // let mut ref_mut2 = shared_val.borrow_mut();

        *ref_mut1 += 10;
        // *ref_mut2 += 10;
        println!("ref_mut1: {ref_mut1}");

        println!("Reference Count in block: {}", Rc::strong_count(&shared_val));
    } // `shared_val` is dropped here.
    println!("Reference Count after block: {}", Rc::strong_count(&owners[0]));
    
    *owners[0].borrow_mut() += 10;
    // No Runtime Error: The temporary RefMut from borrow_mut() is dropped at the semicolon.
    *owners[1].borrow_mut() += 10;
    println!("owners after block: {owners:?}");
}

pub fn run() {
    // compare_rc_and_ref();
    compare_rc_refcell_and_mut_ref();
}