use std::pin::Pin;

fn pin_basic() {
    println!("\n----- Behavior of !Unpin types with Pin -----");

    use std::marker::PhantomPinned;

    // Compile Error: the trait `Clone` is not implemented for `NoUnpin`
    // #[derive(Debug, Copy)]
    #[derive(Debug)]
    struct NoUnpin {
        data: i32,
        _pin: PhantomPinned, // !Unpin
    }

    let item = NoUnpin {
        data: 10,
        _pin: PhantomPinned,
    };
    let pinned_item = Box::pin(item);
    
    // Compile Error
    // let item_copy = *pinned_item;

    // Compile Error: `get_mut` is unavailable for `Pin<Box<T>>` when `T: !Unpin`
    // let old = std::mem::replace(
    //     pinned_item.get_mut(),
    //     NoUnpin { data: 20, _pin: PhantomPinned }
    // );

    // Compile Error: value borrowed here after move.
    // println!("item: {item:?}");

    println!("pinned_item: {pinned_item:?}");

    println!("\n----- Behavior of Unpin types with Pin -----");
    // Types that implement Unpin can be freely moved or copied even when wrapped in a Pin.
    let mut val = 10;
    let mut pinned_val = Pin::new(&mut val);
    *pinned_val += 1;
    let val_copy = *pinned_val;
    println!("val_copy: {}", val_copy);
    let old_val = std::mem::replace(pinned_val.get_mut(), 20);
    println!("old_val: {old_val}, val: {val}");
}

pub fn run() {
    pin_basic();
}