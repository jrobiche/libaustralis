use libaustralis::xboxunity;
use std::io::prelude::*;

// using tokio to allow main to be async
#[tokio::main]
async fn main() {
    // get titles matching `search` query
    let search: &str = "Black Ops"; // search can be a TitleID, MediaID, or Name
    let page: usize = 0;
    let count: usize = 10;
    let sort: xboxunity::SearchSort = xboxunity::SearchSort::Name;
    let direction: xboxunity::SearchDirection = xboxunity::SearchDirection::Ascending;
    let category: xboxunity::SearchCategory = xboxunity::SearchCategory::All;
    let filter: xboxunity::SearchFilter = xboxunity::SearchFilter::All;
    let title_list_result: xboxunity::TitleListResult =
        xboxunity::title_list(search, page, count, sort, direction, category, filter)
            .await
            .unwrap();
    println!(
        "Title List Result for query '{}':\n{:?}\n",
        search, title_list_result
    );

    // save icon for first title
    let icon_title_id: usize = title_list_result.items[0].title_id;
    let icon_bytes: Vec<u8> = xboxunity::icon_image_bytes(icon_title_id).await.unwrap();
    let icon_file_name: &str = &format!("{:08X}_icon.png", icon_title_id);
    let mut file: std::fs::File = std::fs::File::create(icon_file_name).unwrap();
    file.write_all(&icon_bytes).unwrap();
    println!(
        "Saved icon for title id {} (0x{:08X}) to {}.\n",
        icon_title_id, icon_title_id, icon_file_name
    );

    // get cover information for first title
    let cover_title_id: usize = title_list_result.items[0].title_id;
    let cover_info_result: xboxunity::CoverInfoResult =
        xboxunity::cover_info(cover_title_id).await.unwrap();
    println!(
        "Cover Info Result for title id {} (0x{:08X}):\n{:?}\n",
        cover_title_id, cover_title_id, cover_info_result
    );

    // save small cover for first title
    let cover_id: usize = cover_info_result.covers[0].cover_id;
    let cover_bytes: Vec<u8> = xboxunity::cover_image_bytes(cover_id, xboxunity::CoverSize::Small)
        .await
        .unwrap();
    let cover_file_name: &str = &format!("{}_cover_small.png", cover_id);
    let mut file: std::fs::File = std::fs::File::create(cover_file_name).unwrap();
    file.write_all(&cover_bytes).unwrap();
    println!(
        "Saved small cover with cover id {} to {}.\n",
        cover_id, cover_file_name
    );
}
