use reqwest;
use std::fs::File;
use std::io::Write;

pub struct Task<'a> {
    url: String,
    file_name: &'a str,
    client: reqwest::Client,
}

pub struct TaskResult {
    pub file_name: String,
}

impl<'a> Task<'a> {
    pub fn new(url: String, file_name: &'a str) -> Self {
        let client = reqwest::Client::new();

        Self {
            url,
            file_name,
            client,
        }
    }

    pub async fn run<T>(&self, on_progress: T) -> Result<(), ()>
    where
        T: Fn(f64),
    {
        let mut file = File::create(&self.file_name).unwrap();

        let mut res = self
            .client
            .get(&self.url)
            .header("User-Agent", "Mozilla")
            .send()
            .await
            .unwrap();

        let total = res.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;

        while let Some(chunk) = res.chunk().await.unwrap() {
            file.write_all(&chunk).unwrap();
            downloaded += chunk.len() as u64;
            let percent = ((downloaded as f64 / total as f64) * 100.0).round();
            on_progress(percent);
        }

        Ok(())
    }
}
