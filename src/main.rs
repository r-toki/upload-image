use tempfile::Builder;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("storage/50kb.jpg").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let encoded = base64::encode(&buffer);

    let decoded = base64::decode(encoded)?;
    let tmpfile = Builder::new().tempfile()?;
    let mut file = File::create(&tmpfile.path()).await?;
    file.write_all(&decoded).await?;

    Ok(())
}
