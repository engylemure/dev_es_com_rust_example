#![allow(dead_code)]
use tokio; // 1.0.2
use tokio::time::{sleep, Duration};

async fn say_hello() -> String {
    String::from("Hello")
}

async fn say_bye() -> String {
    sleep(Duration::from_secs(2)).await;
    String::from("Bye")
}

async fn hello_then_bye() -> String {
    let hello = say_hello().await;
    let bye = say_bye().await;
    format!("{} {}!", hello, bye)
}

async fn join_hello_and_bye() -> (String, String) {
    let hello = say_hello();
    let bye = say_bye();
    futures::join!(hello, bye)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::runtime::Runtime;

    let rt = Runtime::new()?;
    let result = rt.block_on(async {
        let future = join_hello_and_bye();
        println!("{:?}", future.await);
    });
    Ok(result)
}
