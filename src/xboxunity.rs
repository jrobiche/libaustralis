/**
 * Copyright 2025-2026 jrobiche
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

pub const API_ROOT: &str = "https://xboxunity.net";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CoverInfoItem {
    #[serde(
        rename = "CoverID",
        serialize_with = "usize_as_str",
        deserialize_with = "usize_from_str"
    )]
    pub cover_id: usize,
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
    #[serde(
        rename = "TitleID",
        serialize_with = "usize_as_str_b16",
        deserialize_with = "usize_from_str_b16"
    )]
    pub title_id: usize,
    #[serde(
        rename = "HBTitleID",
        serialize_with = "usize_as_str_b16",
        deserialize_with = "usize_from_str_b16"
    )]
    pub hb_title_id: usize,
    pub name: String,
    pub link_enabled: String, // TODO
    pub title_type: String,   // TODO
    pub covers: String,       // TODO
    pub updates: String,      // TODO
    #[serde(
        rename = "MediaIDCount",
        serialize_with = "usize_as_str",
        deserialize_with = "usize_from_str"
    )]
    pub media_id_count: usize,
    #[serde(serialize_with = "usize_as_str", deserialize_with = "usize_from_str")]
    pub user_count: usize,
    pub newest_content: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleListResult {
    pub items: Vec<TitleListItem>,
    pub count: usize,
    pub pages: usize,
    pub page: usize,
    #[serde(
        serialize_with = "search_filter_as_str",
        deserialize_with = "search_filter_from_str"
    )]
    pub filter: SearchFilter,
    #[serde(
        serialize_with = "search_category_as_str",
        deserialize_with = "search_category_from_str"
    )]
    pub category: SearchCategory,
    #[serde(
        serialize_with = "search_sort_as_str",
        deserialize_with = "search_sort_from_str"
    )]
    pub sort: SearchSort,
    #[serde(
        serialize_with = "search_direction_as_str",
        deserialize_with = "search_direction_from_str"
    )]
    pub direction: SearchDirection,
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum SearchSort {
    Name = 0,
    Covers = 1,
    Updates = 2,
    Updated = 3,
    LinkUsers = 4,
}

impl SearchSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchSort::Name => "Name",
            SearchSort::Covers => "Covers",
            SearchSort::Updates => "Updates",
            SearchSort::Updated => "Updated",
            SearchSort::LinkUsers => "LinkUsers",
        }
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_str(s: &str) -> GenericResult<Self> {
        match s {
            "Name" => Ok(SearchSort::Name),
            "Covers" => Ok(SearchSort::Covers),
            "Updates" => Ok(SearchSort::Updates),
            "Updated" => Ok(SearchSort::Updated),
            "LinkUsers" => Ok(SearchSort::LinkUsers),
            _ => Err(format!("Invalid search sort: {}", s).into()),
        }
    }

    pub fn from_usize(x: usize) -> GenericResult<Self> {
        match x {
            0 => Ok(SearchSort::Name),
            1 => Ok(SearchSort::Covers),
            2 => Ok(SearchSort::Updates),
            3 => Ok(SearchSort::Updated),
            4 => Ok(SearchSort::LinkUsers),
            _ => Err(format!("Invalid search sort: {}", x).into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum SearchDirection {
    Ascending = 0,
    Descending = 1,
}

impl SearchDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchDirection::Ascending => "Ascending",
            SearchDirection::Descending => "Descending",
        }
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_str(s: &str) -> GenericResult<Self> {
        match s {
            "Ascending" => Ok(SearchDirection::Ascending),
            "Descending" => Ok(SearchDirection::Descending),
            _ => Err(format!("Invalid search direction: {}", s).into()),
        }
    }

    pub fn from_usize(x: usize) -> GenericResult<Self> {
        match x {
            0 => Ok(SearchDirection::Ascending),
            1 => Ok(SearchDirection::Descending),
            _ => Err(format!("Invalid search direction: {}", x).into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum SearchCategory {
    All = 0,
    TitlesWithLink = 3,
}

impl SearchCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchCategory::All => "All",
            SearchCategory::TitlesWithLink => "TitlesWithLink",
        }
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_str(s: &str) -> GenericResult<Self> {
        match s {
            "All" => Ok(SearchCategory::All),
            "TitlesWithLink" => Ok(SearchCategory::TitlesWithLink),
            _ => Err(format!("Invalid search filter: {}", s).into()),
        }
    }

    pub fn from_usize(x: usize) -> GenericResult<Self> {
        match x {
            0 => Ok(SearchCategory::All),
            3 => Ok(SearchCategory::TitlesWithLink),
            _ => Err(format!("Invalid search filter: {}", x).into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum SearchFilter {
    All = 0,
    XB360 = 1,
    XBLA = 2,
    XboxClassic = 3,
    Homebrew = 4,
}

impl SearchFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchFilter::All => "All",
            SearchFilter::XB360 => "XB360",
            SearchFilter::XBLA => "XBLA",
            SearchFilter::XboxClassic => "XboxClassic",
            SearchFilter::Homebrew => "Homebrew",
        }
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_str(s: &str) -> GenericResult<Self> {
        match s {
            "All" => Ok(SearchFilter::All),
            "XB360" => Ok(SearchFilter::XB360),
            "XBLA" => Ok(SearchFilter::XBLA),
            "XboxClassic" => Ok(SearchFilter::XboxClassic),
            "Homebrew" => Ok(SearchFilter::Homebrew),
            _ => Err(format!("Invalid search filter: {}", s).into()),
        }
    }

    pub fn from_usize(x: usize) -> GenericResult<Self> {
        match x {
            0 => Ok(SearchFilter::All),
            1 => Ok(SearchFilter::XB360),
            2 => Ok(SearchFilter::XBLA),
            3 => Ok(SearchFilter::XboxClassic),
            4 => Ok(SearchFilter::Homebrew),
            _ => Err(format!("Invalid search filter: {}", x).into()),
        }
    }
}

pub async fn cover_info(title_id: usize) -> GenericResult<CoverInfoResult> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    let title_id_str = &format!("{:08x}", title_id);
    let query: Vec<(&str, &str)> = vec![("titleid", title_id_str)];
    let endpoint = "/Resources/Lib/CoverInfo.php";
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.json::<CoverInfoResult>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                let msg = "Failed to parse CoverInfoResult from response.";
                error!("{} Got the following error: {}", msg, err);
                Err(msg.into())
            }
        },
        Err(err) => {
            let msg = format!("Failed to make GET request to '{}'.", endpoint);
            error!("{} Got the following error: {}", msg, err);
            Err(msg.into())
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
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    let page_str = &format!("{}", page);
    let count_str = &format!("{}", count);
    let sort_str = &format!("{}", sort.as_usize());
    let direction_str = &format!("{}", direction.as_usize());
    let category_str = &format!("{}", category.as_usize());
    let filter_str = &format!("{}", filter.as_usize());
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
                let msg = "Failed to parse TitleListResult from response.";
                error!("{} Got the following error: {}", msg, err);
                Err(msg.into())
            }
        },
        Err(err) => {
            let msg = format!("Failed to make GET request to '{}'.", endpoint);
            error!("{} Got the following error: {}", msg, err);
            Err(msg.into())
        }
    }
}

pub async fn cover_image_bytes(cover_id: usize, cover_size: CoverSize) -> GenericResult<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    let endpoint = "/Resources/Lib/Cover.php";
    let cover_id_str = &format!("{}", cover_id);
    let query: Vec<(&str, &str)> = vec![("size", cover_size.as_str()), ("cid", cover_id_str)];
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = "Failed to get cover image bytes from response.";
                error!("{} Got the following error: {}", msg, err);
                Err(msg.into())
            }
        },
        Err(err) => {
            let msg = format!("Failed to make GET request to '{}'.", endpoint);
            error!("{} Got the following error: {}", msg, err);
            Err(msg.into())
        }
    }
}

pub async fn icon_image_bytes(title_id: usize) -> GenericResult<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    let endpoint = "/Resources/Lib/Icon.php";
    let title_id_str = &format!("{:08x}", title_id);
    let query: Vec<(&str, &str)> = vec![("tid", title_id_str), ("custom", "1")];
    let req = client
        .get(format!("{}{}", API_ROOT, endpoint))
        .query(&query);
    match req.send().await {
        Ok(resp) => match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = "Failed to get icon image bytes from response.";
                error!("{} Got the following error: {}", msg, err);
                Err(msg.into())
            }
        },
        Err(err) => {
            let msg = format!("Failed to make GET request to '{}'.", endpoint);
            error!("{} Got the following error: {}", msg, err);
            Err(msg.into())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// serialize and deserialize functions
////////////////////////////////////////////////////////////////////////////////
fn usize_as_str<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{}", value))
}

fn usize_from_str<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match usize::from_str_radix(s, 10) {
        Ok(x) => Ok(x),
        Err(err) => Err(serde::de::Error::custom(format!(
            "Failed to parse usize from string. Got the following error: {}",
            err
        ))),
    }
}

fn usize_as_str_b16<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{:08x}", value))
}

fn usize_from_str_b16<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match usize::from_str_radix(s, 16) {
        Ok(x) => Ok(x),
        Err(err) => Err(serde::de::Error::custom(format!(
            "Failed to parse usize from base 16 string. Got the following error: {}",
            err
        ))),
    }
}

fn search_category_as_str<S>(
    search_category: &SearchCategory,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match search_category {
        SearchCategory::All => serializer.serialize_str("0"),
        SearchCategory::TitlesWithLink => serializer.serialize_str("3"),
    }
}

fn search_category_from_str<'de, D>(deserializer: D) -> Result<SearchCategory, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match s {
        "0" => Ok(SearchCategory::All),
        "3" => Ok(SearchCategory::TitlesWithLink),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid SearchCategory value: {}",
            s
        ))),
    }
}

fn search_direction_as_str<S>(
    search_direction: &SearchDirection,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match search_direction {
        SearchDirection::Ascending => serializer.serialize_str("0"),
        SearchDirection::Descending => serializer.serialize_str("1"),
    }
}

fn search_direction_from_str<'de, D>(deserializer: D) -> Result<SearchDirection, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match s {
        "0" => Ok(SearchDirection::Ascending),
        "1" => Ok(SearchDirection::Descending),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid SearchDirection value: {}",
            s
        ))),
    }
}

fn search_filter_as_str<S>(search_filter: &SearchFilter, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match search_filter {
        SearchFilter::All => serializer.serialize_str("0"),
        SearchFilter::XB360 => serializer.serialize_str("1"),
        SearchFilter::XBLA => serializer.serialize_str("2"),
        SearchFilter::XboxClassic => serializer.serialize_str("3"),
        SearchFilter::Homebrew => serializer.serialize_str("4"),
    }
}

fn search_filter_from_str<'de, D>(deserializer: D) -> Result<SearchFilter, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match s {
        "0" => Ok(SearchFilter::All),
        "1" => Ok(SearchFilter::XB360),
        "2" => Ok(SearchFilter::XBLA),
        "3" => Ok(SearchFilter::XboxClassic),
        "4" => Ok(SearchFilter::Homebrew),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid SearchFilter value: {}",
            s
        ))),
    }
}

fn search_sort_as_str<S>(search_sort: &SearchSort, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match search_sort {
        SearchSort::Name => serializer.serialize_str("0"),
        SearchSort::Covers => serializer.serialize_str("1"),
        SearchSort::Updates => serializer.serialize_str("2"),
        SearchSort::Updated => serializer.serialize_str("3"),
        SearchSort::LinkUsers => serializer.serialize_str("4"),
    }
}

fn search_sort_from_str<'de, D>(deserializer: D) -> Result<SearchSort, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match s {
        "0" => Ok(SearchSort::Name),
        "1" => Ok(SearchSort::Covers),
        "2" => Ok(SearchSort::Updates),
        "3" => Ok(SearchSort::Updated),
        "4" => Ok(SearchSort::LinkUsers),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid SearchSort value: {}",
            s
        ))),
    }
}
