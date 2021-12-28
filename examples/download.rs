use rust_playground::download::Task;

#[tokio::main]
async fn main() {
    let res = Task::new
    (
        String::from("https://mirrors.tuna.tsinghua.edu.cn/archlinux/iso/latest/archlinux-2021.12.01-x86_64.iso"),
        String::from("files/archlinux.iso")
    )
        .run(|percent| {
            println!("downloading... {}%", percent);
        })
        .await
        .unwrap();

    println!("{}", res.file_name);
}
