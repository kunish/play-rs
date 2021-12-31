use futures;
use rust_playground::download;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let done = Arc::new(Mutex::new(0));
    let total = 10;
    let mut task_pool: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for i in 0..total {
        let done = done.clone();

        let handle = tokio::spawn(async move {
            let i = i.to_string();
            let file_name = format!("files/example-{}", i);

            download::Task::new(String::from("https://example.com"), &file_name)
                .run(|precent| {
                    println!("{}: {}", file_name, precent);
                })
                .await
                .unwrap();

            let mut done = done.lock().unwrap();
            *done += 1;
        });

        task_pool.push(handle);
    }

    futures::future::join_all(task_pool).await;

    println!("done {}", done.clone().lock().unwrap());
}
