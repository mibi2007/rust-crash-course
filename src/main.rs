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
    async {
        sleep(Duration::from_secs(1)).await;
        "Two".to_owned()
    }
}

#[tokio::main]
async fn main() {
    let name = call_api_one().await;
    println!("Hello {}", name);
    let name2 = call_api_two().await;
    println!("Hello {}", name2);
}
