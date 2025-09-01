/**
 * Copyright 2025 jrobiche
 *
 * This file is part of libaustralis.
 *
 * libaustrais is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * libaustralis is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * libaustralis. If not, see <https://www.gnu.org/licenses/>.
 */
// TODO define tests
// TODO document functions
// TODO improve logging
// TODO improve error handling/bubbling
// TODO verify response codes
use crate::utils::GenericResult;
use log::error;
use serde;

// TODO enumerate or usize values
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveImage {
    media_type: String,
    relationship_type: Option<String>,
    format: String,
    size: String,
    file_url: String,
}

fn parse_live_images(xml: &str) -> GenericResult<Vec<LiveImage>> {
    let mut live_images: Vec<LiveImage> = Vec::new();
    let reader = xml::reader::EventReader::from_str(xml);
    let mut element_name: String = String::from("");
    let mut media_type: String = String::from("");
    let mut relationship_type: Option<String> = None;
    let mut format: String = String::from("");
    let mut size: String = String::from("");
    let mut file_url: String = String::from("");
    for reader_element in reader {
        match reader_element {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                element_name = name.local_name;
                if element_name == "image" {
                    media_type = "".to_string();
                    relationship_type = None;
                    format = "".to_string();
                    size = "".to_string();
                    file_url = "".to_string();
                }
            }
            Ok(xml::reader::XmlEvent::EndElement { name }) => {
                if name.local_name == "image" {
                    let new_live_image = LiveImage {
                        media_type: media_type.clone(),
                        relationship_type: relationship_type.clone(),
                        format: format.clone(),
                        size: size.clone(),
                        file_url: file_url.clone(),
                    };
                    live_images.push(new_live_image);
                } else if name.local_name == "entry" {
                    break;
                }
            }
            Ok(xml::reader::XmlEvent::Characters(s)) => match element_name.as_str() {
                "imageMediaType" => media_type = s,
                "relationshipType" => relationship_type = Some(s),
                "format" => format = s,
                "size" => size = s,
                "fileUrl" => file_url = s,
                _ => (),
            },
            Err(err) => {
                error!("Failed to parse xml. Got the following error: {}", err);
                return Err("Failed to parse xml.".into());
            }
            _ => {}
        }
    }
    Ok(live_images)
}

pub const API_ROOT: &str = "https://catalog.xboxlive.com";

pub async fn image_bytes(live_image: LiveImage) -> GenericResult<Vec<u8>> {
    let client = reqwest::Client::new();
    let req = client.get(&live_image.file_url);
    match req.send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get image bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get image bytes from response.".into())
            }
        },
        Err(err) => {
            error!(
                "Failed to make GET request to '{}'. Got the following error: {}",
                &live_image.file_url, err
            );
            Err(format!("Failed to make GET request to '{}'.", live_image.file_url).into())
        }
    }
}

pub async fn live_images_for_title_id(
    title_id: &str,
    locale: &str,
) -> GenericResult<Vec<LiveImage>> {
    let client = reqwest::Client::new();
    let media_ids_value_str = &format!("66acd000-77fe-1000-9115-d802{}", title_id);
    let query: Vec<(&str, &str)> = vec![
        ("methodName", "FindGames"),
        ("Names", "MediaIds"),
        ("Values", media_ids_value_str),
        ("Names", "DetailView"),
        ("Values", "5"),
        ("Names", "LegalLocale"),
        ("Values", locale),
        ("Names", "Locale"),
        ("Values", locale),
        ("Names", "MediaTypes"),
        ("Values", "1"),
        ("Names", "MediaTypes"),
        ("Values", "5"),
        ("Names", "MediaTypes"),
        ("Values", "18"),
        ("Names", "MediaTypes"),
        ("Values", "19"),
        ("Names", "MediaTypes"),
        ("Values", "20"),
        ("Names", "MediaTypes"),
        ("Values", "21"),
        ("Names", "MediaTypes"),
        ("Values", "22"),
        ("Names", "MediaTypes"),
        ("Values", "23"),
        ("Names", "MediaTypes"),
        ("Values", "24"),
        ("Names", "MediaTypes"),
        ("Values", "30"),
        ("Names", "MediaTypes"),
        ("Values", "34"),
        ("Names", "MediaTypes"),
        ("Values", "37"),
        ("Names", "MediaTypes"),
        ("Values", "45"),
        ("Names", "MediaTypes"),
        ("Values", "46"),
        ("Names", "MediaTypes"),
        ("Values", "47"),
        ("Names", "MediaTypes"),
        ("Values", "57"),
        ("Names", "MediaTypes"),
        ("Values", "59"),
        ("Names", "MediaTypes"),
        ("Values", "60"),
        ("Names", "MediaTypes"),
        ("Values", "61"),
        ("Names", "MediaTypes"),
        ("Values", "62"),
        ("Names", "MediaTypes"),
        ("Values", "63"),
        ("Names", "MediaTypes"),
        ("Values", "64"),
        ("Names", "MediaTypes"),
        ("Values", "66"),
        ("Names", "MediaTypes"),
        ("Values", "67"),
        ("Names", "OfferFilterLevel"),
        ("Values", "1"),
        ("Names", "OrderDirection"),
        ("Values", "1"),
        ("Names", "PageNum"),
        ("Values", "1"),
        ("Names", "PageSize"),
        ("Values", "100"),
        ("Names", "Store"),
        ("Values", "1"),
        ("Names", "UserTypes"),
        ("Values", "2"),
    ];
    let endpoint = "/Catalog/Catalog.asmx/Query";
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.text().await {
            Ok(xml) => parse_live_images(&xml),
            Err(err) => {
                error!(
                    "Failed to get text from response. Got the following error: {}",
                    err
                );
                Err("Failed to get text from response.".into())
            }
        },
        Err(err) => {
            error!(
                "Failed to make GET request to '{}'. Got the following error: {}",
                endpoint, err
            );
            Err(format!("Failed to make GET request to '{}'.", endpoint).into())
        }
    }
}
