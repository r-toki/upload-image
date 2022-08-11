use dotenv::dotenv;
use reqwest::{multipart, Client};
use sha1::{Digest, Sha1};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cloudinary_cloud_name = env::var("CLOUDINARY_CLOUD_NAME")?;
    let cloudinary_api_key = env::var("CLOUDINARY_API_KEY")?;
    let cloudinary_api_secret = env::var("CLOUDINARY_API_SECRET")?;

    let folder = "upload-image";
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let mut signature = Sha1::new();
    signature.update(format!(
        "folder={}&timestamp={}{}",
        folder, timestamp, cloudinary_api_secret
    ));
    let signature = hex::encode(signature.finalize());

    let mut file = File::open("storage/50kb.jpg").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let encoded = base64::encode(&buffer);

    let decoded = base64::decode(encoded)?;
    let file = multipart::Part::stream(decoded)
        .file_name("50kb.jpg")
        .mime_str("image/jpeg")?;

    let form = multipart::Form::new()
        .part("file", file)
        .text("api_key", cloudinary_api_key)
        .text("folder", folder)
        .text("timestamp", timestamp.to_string())
        .text("signature", signature);

    let client = Client::new();

    let res = client
        .post(format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            cloudinary_cloud_name
        ))
        .multipart(form)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}
