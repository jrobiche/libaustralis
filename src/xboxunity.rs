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
// TODO use "search" enums in structs
use crate::utils::GenericResult;
use log::error;
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoverInfoItem {
    #[serde(rename = "CoverID")]
    pub cover_id: String,
    pub rating: Option<String>,
    pub official: String,
    pub username: String,
    pub file_size: String,
    pub uploaded_date: String,
    pub no_rate: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoverInfoResult {
    pub covers: Vec<CoverInfoItem>,
    pub covers_count: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleListItem {
    #[serde(rename = "TitleID")]
    pub title_id: String,
    #[serde(rename = "HBTitleID")]
    pub hb_title_id: String,
    pub name: String,
    pub link_enabled: String,
    pub title_type: String,
    pub covers: String,
    pub updates: String,
    #[serde(rename = "MediaIDCount")]
    pub media_id_count: String,
    pub user_count: String,
    pub newest_content: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleListResult {
    pub items: Vec<TitleListItem>,
    pub count: u32,
    pub pages: u32,
    pub page: u32,
    pub filter: String,
    pub category: String,
    pub sort: String,
    pub direction: String,
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum CoverSize {
    Small,
    Large,
}

impl CoverSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoverSize::Small => "small",
            CoverSize::Large => "large",
        }
    }

    pub fn from_str(s: &str) -> GenericResult<Self> {
        match s {
            "small" => Ok(CoverSize::Small),
            "large" => Ok(CoverSize::Large),
            _ => Err(format!("Invalid cover size: {}", s).into()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum SearchSort {
    Name = 0,
    Covers = 1,
    Updates = 2,
    Updated = 3,
    LinkUsers = 4,
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum SearchDirection {
    Ascending = 0,
    Descending = 1,
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum SearchCategory {
    All = 0,
    TitlesWithLink = 3,
}

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
pub enum SearchFilter {
    All = 0,
    XB360 = 1,
    XBLA = 2,
    XboxClassic = 3,
    Homebrew = 4,
}

pub const API_ROOT: &str = "https://xboxunity.net";

pub async fn cover_info(titleid: &str) -> GenericResult<CoverInfoResult> {
    let client = reqwest::Client::new();
    let query: Vec<(&str, &str)> = vec![("titleid", titleid)];
    let endpoint = "/Resources/Lib/CoverInfo.php";
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.json::<CoverInfoResult>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse CoverInfoResult from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse CoverInfoResult from response.".into())
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

pub async fn title_list(
    search: &str,
    page: usize,
    count: usize,
    sort: SearchSort,
    direction: SearchDirection,
    category: SearchCategory,
    filter: SearchFilter,
) -> GenericResult<TitleListResult> {
    let client = reqwest::Client::new();
    let page_str = &format!("{}", page);
    let count_str = &format!("{}", count);
    let sort_str = &format!("{}", sort as u32);
    let direction_str = &format!("{}", direction as u32);
    let category_str = &format!("{}", category as u32);
    let filter_str = &format!("{}", filter as u32);
    let query: Vec<(&str, &str)> = vec![
        ("page", page_str),
        ("count", count_str),
        ("search", search),
        ("sort", sort_str),
        ("direction", direction_str),
        ("category", category_str),
        ("filter", filter_str),
    ];
    let endpoint = "/Resources/Lib/TitleList.php";
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.json::<TitleListResult>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse TitleListResult from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse TitleListResult from response.".into())
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

pub async fn cover_image_bytes(cover_id: &str, cover_size: CoverSize) -> GenericResult<Vec<u8>> {
    let client = reqwest::Client::new();
    let endpoint = "/Resources/Lib/Cover.php";
    let cover_size_str = &format!("{}", cover_size.as_str().to_string());
    let query: Vec<(&str, &str)> = vec![("size", cover_size_str), ("cid", cover_id)];
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get cover image bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get cover image bytes from response.".into())
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

pub fn cover_image_url(cover_id: &str, cover_size: CoverSize) -> String {
    return format!(
        "{}/Resources/Lib/Cover.php?size={}&cid={}",
        API_ROOT,
        cover_size.as_str().to_string(),
        cover_id
    );
}

pub async fn icon_image_bytes(title_id: &str) -> GenericResult<Vec<u8>> {
    let client = reqwest::Client::new();
    let endpoint = "/Resources/Lib/Icon.php";
    let query: Vec<(&str, &str)> = vec![("tid", title_id), ("custom", "1")];
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get icon image bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get icon image bytes from response.".into())
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

pub fn icon_image_url(title_id: &str) -> String {
    return format!(
        "{}/Resources/Lib/Icon.php?tid={}&custom=1",
        API_ROOT, title_id
    );
}
