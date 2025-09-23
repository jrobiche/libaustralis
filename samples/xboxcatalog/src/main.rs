use libaustralis::xboxcatalog;
use std::io::prelude::*;

// using tokio to allow main to be async
#[tokio::main]
async fn main() {
    // get live images for `locale` and `title_id`
    let locale: xboxcatalog::Locale = xboxcatalog::Locale::from_code_str("en-US").unwrap();
    let title_id: usize = 0x415608C3; // title id for Black Ops II
    let live_images: Vec<xboxcatalog::LiveImage> =
        xboxcatalog::live_images_for_title_id(title_id, locale)
            .await
            .unwrap();
    println!(
        "Found {} LiveImages for title id {} (0x{:08X}) with locale {}:",
        live_images.len(),
        title_id,
        title_id,
        locale.as_code_str()
    );
    for live_image in &live_images {
        println!("{:?}", live_image);
    }
    println!("");

    // save first live image
    let live_image_bytes: Vec<u8> = live_images[0].file_bytes().await.unwrap();
    let live_image_file_name: &str = &format!("{:08X}_0.png", title_id);
    let mut file: std::fs::File = std::fs::File::create(live_image_file_name).unwrap();
    file.write_all(&live_image_bytes).unwrap();
    println!(
        "Saved live image with file url '{}' to {}.\n",
        &live_images[0].file_url, live_image_file_name
    );
}
