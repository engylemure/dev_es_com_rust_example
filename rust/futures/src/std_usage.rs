#![allow(dead_code)]
use futures::executor::block_on; // 0.3.12

async fn say_hello() -> String {
    String::from("Hello")
}

async fn say_bye() -> String {
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

pub fn main() {
    let future = say_bye();
    // println!("{}", future);
    // Execute a future
    println!("{:?}", block_on(future));
}
