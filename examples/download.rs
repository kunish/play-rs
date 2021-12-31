use play_rs::download::Task;

#[tokio::main]
async fn main() {
    let file_name = String::from("files/example.com");
    Task::new(String::from("https://example.com"), &file_name)
        .run(|percent| {
            println!("downloading... {}%", percent);
        })
        .await
        .unwrap();
}
