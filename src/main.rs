#![deny(clippy::all)]

use futures::Future;
use tokio::time::{sleep, Duration};

fn call_api_one() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "One".to_owned()
    }
}

fn call_api_two() -> impl Future<Output = String> {
    let name = "John".to_owned();
    async move {
        sleep(Duration::from_secs(1)).await;
        format!("{} Doe", name)
    }
}

#[tokio::main]
async fn main() {
    let name = call_api_one().await;
    println!("Hello {}", name);
    let name2 = call_api_two().await;
    println!("Hello {}", name2);
}
