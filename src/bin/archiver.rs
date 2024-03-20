use lib::archiver;

#[tokio::main]
async fn main() {
    archiver::launch().await;
}
