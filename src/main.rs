use dotenv::dotenv;
use reqwest::{multipart, Client};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ImageUploadResponse {
    // asset_id: String,
    public_id: String,
    // version: u64,
    // version_id: String,
    // signature: String,
    // width: u64,
    // height: u64,
    format: String,
    resource_type: String,
    // created_at: String,
    // tags: Vec<String>,
    // bytes: u64,
    // r#type: String,
    // etag: String,
    // placeholder: bool,
    // url: String,
    secure_url: String,
    // folder: String,
    // original_filename: String,
    // original_extension: String,
    // api_key: String,
}

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
    let filename = Uuid::new_v4().to_string() + ".jpeg";
    let file = multipart::Part::stream(decoded)
        .file_name(filename)
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
    let text = res.text().await?;
    let extracted: ImageUploadResponse = serde_json::from_str(&text)?;
    println!("{:?}", extracted);

    let secure_url = extracted.secure_url;
    let res = reqwest::get(secure_url).await?;
    println!("{:?}", res);
    let encoded = base64::encode(res.bytes().await?);
    let mut file = File::create("storage/encoded.txt").await?;
    file.write_all(encoded.as_bytes()).await?;

    Ok(())
}
