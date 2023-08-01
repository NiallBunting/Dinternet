mod directory;
mod network;

#[async_std::main]
async fn main() {
    println!("Dinterent node");

    directory::check_filesystem_or_create();
    let _ = network::start().await;
}
