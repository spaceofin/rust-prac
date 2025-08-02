use std::time::Duration;

pub fn run_with_tasks() {
    trpl::run(async {

        async {
          println!("Running inside an async block.")
        }.await;

        let _future = async {
          println!("This async block is created but not awaited.")
        };

        trpl::spawn_task(async {
            for i in 1..=5 {
                println!("Spawned task 1: {i}");
            }
        });

        for i in 1..=3 {
            println!("Task 2: {i}");
            trpl::sleep(Duration::from_millis(1)).await;
        }

        println!("____________________Sync print1");

        trpl::spawn_task(async {
            for i in 1..=10 {
                println!("Spawned task 2: {i}");
                trpl::sleep(Duration::from_millis(2000)).await;
            }
        });

        async {
          println!("Running inside an async block2...1");
          trpl::sleep(Duration::from_millis(5000)).await;
          println!("Running inside an async block2...2");
        }.await;

        println!("____________________Sync print2");

        for i in 1..=10 {
            println!("Task 3: {i}");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        println!("____________________Sync print3");

        async {
          println!("Running inside an async block3")
        }.await;
    });
}