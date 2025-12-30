use trpl::join;

async fn some_async_op(label: &str) {
    println!("[{label}] async op start");
    trpl::yield_now().await;
    println!("[{label}] async op end");
}

fn single_asnyc() {
    trpl::run(async{
        println!("[f1  ] start");

        some_async_op("f1-1").await;
        println!("[f1  ] after first await");

        some_async_op("f1-2").await;
        println!("[f1  ] after second await");

        some_async_op("f1-3").await;
        println!("[f1  ] after second await");

        println!("[f1  ] end");
    })
}

fn multiple_async() {
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

fn sequential_async() {
    trpl::run(async {
        async {
            println!("[f1  ] start");

            some_async_op("f1-1").await;
            println!("[f1  ] after first await");

            some_async_op("f1-2").await;
            println!("[f1  ] after second await");

            println!("[f1  ] end");
        }.await;

        async {
            println!("[f2  ] start");

            some_async_op("f2-1").await;
            println!("[f2  ] after await");

            println!("[f2  ] end");
        }.await;

        async {
            println!("[f3  ] start");

            some_async_op("f3-1").await;
            println!("[f3  ] after first await");

            some_async_op("f3-2").await;
            println!("[f3  ] after second await");

            println!("[f3  ] end");
        }.await;
    });
}

// Compile Error: multiple blocking calls are performed
// fn double_block_async() {
//     trpl::run(async{
//         trpl::run(async {
//             println!("[f1  ] start");

//             some_async_op("f1-1").await;
//             println!("[f1  ] after first await");

//             some_async_op("f1-2").await;
//             println!("[f1  ] after second await");

//             println!("[f1  ] end");
//         });
//     });
// }

// Compile Error: `await` only allowed inside `async` functions and blocks
// fn await_in_sync_block() {
//     async {
//         println!("[f1  ] start");

//         some_async_op("f1-1").await;
//         println!("[f1  ] after first await");

//         some_async_op("f1-2").await;
//         println!("[f1  ] after second await");

//         some_async_op("f1-3").await;
//         println!("[f1  ] after second await");

//         println!("[f1  ] end");
//     }.await;
// }

fn nested_async() {
    trpl::run(async{
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
            println!("[f2  ] after first await");

            // This inner async block runs sequentially like synchronous code because it is immediately `.await`ed.
            async {
                println!("[f2i ] start");

                some_async_op("f2i-1").await;
                println!("[f2i ] after first await");

                some_async_op("f2i-2").await;
                println!("[f2i ] after second await");

                println!("[f2i ] end");
            }.await;
            
            some_async_op("f2-2").await;
            println!("[f2  ] after second await");

            println!("[f2  ] end");
        };

        join!(f1,f2);
    });
}

pub fn run() {
    // single_asnyc();
    // multiple_async();
    // sequential_async();
    nested_async();
}