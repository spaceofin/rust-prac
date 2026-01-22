pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist was called.");
    }

    pub fn seat_at_table() {
        println!("seat_at_table was called.");
    }
}

mod serving {
    fn take_order() {
        println!("take_order was called.");
    }

    fn serve_order() {
        println!("serve_order was called.");
    }

    fn take_payment() {
        println!("take_payment was called.");
    }
}