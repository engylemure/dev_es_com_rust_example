mod std_usage;
mod tokio_usage;

fn main() {
    std_usage::main();
    // tokio_usage::main().unwrap();
    println!("Fim do programa");
}