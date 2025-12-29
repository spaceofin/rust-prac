use trpl::join;

async fn some_async_op(label: &str) {
    println!("[{label}] async op start");
    trpl::yield_now().await;
    println!("[{label}] async op end");
}

fn async_join_demo() {
    trpl::run(async {
        let f1 = async {
            println!("[f1  ] start");

            some_async_op("f1-1").await;
            println!("[f1  ] after first await");

            some_async_op("f1-2").await;
            println!("[f1  ] after second await");

            println!("[f1  ] end");
        };

        let f2 = async {
            println!("[f2  ] start");

            some_async_op("f2-1").await;
            println!("[f2  ] after await");

            println!("[f2  ] end");
        };

        let f3 = async {
            println!("[f3  ] start");

            some_async_op("f3-1").await;
            println!("[f3  ] after first await");

            some_async_op("f3-2").await;
            println!("[f3  ] after second await");

            println!("[f3  ] end");
        };

        join!(f1, f2, f3);
    });
}

pub fn run() {
    async_join_demo();
}