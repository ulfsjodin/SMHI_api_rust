mod api;
use api::smhi::{self, get_raw_json};

#[tokio::main]
async fn main() {
    match get_raw_json().await {
        Ok(json) => println!("Svar från SMHI: {}", json),
        Err(e) => eprintln!("Inget svar fån SMHI {}", e),
    }
}
