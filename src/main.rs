use anyhow::Result;
use rust_chimp::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
