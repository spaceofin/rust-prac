fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);
}

fn check_memory_leak() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);
        
        // `_box3` is destroyed here, and memory get freed
    }

    // Creating lots of boxed just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box()
    }

    // `_box2` is destroyed here, and memory gets freed
}

fn main() {
    check_memory_leak();
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn drop_example() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}

pub fn raii_demo() {
    drop_example();
}