use play_rs::download::Task;

#[tokio::main]
async fn main() {
    Task::new(String::from("https://example.com"), "files/example.com")
        .run(|percent| {
            println!("downloading... {}%", percent);
        })
        .await
        .unwrap();
}
