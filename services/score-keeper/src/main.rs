#[tokio::main]
async fn main() {
    let server = webserver::new();

    println!("Starting score keeper...");

    server.listen().await
}
