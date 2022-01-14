fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

    println!("{} v{}", NAME, VERSION);
    println!("Copyright (c) 2021 {}", AUTHORS);

}