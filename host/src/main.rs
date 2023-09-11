mod api;

use api::api_start;


#[tokio::main]
async fn main() {
    api_start().await;
}
