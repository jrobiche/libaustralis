// Copyright 2025-2026 jrobiche
//
// This file is part of libaustralis.
//
// libaustrais is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// libaustralis is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// libaustralis. If not, see <https://www.gnu.org/licenses/>.

use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementDTO {
    pub cred: u32,
    pub hidden: u32,
    pub id: u32,
    pub imageid: u32,
    pub strings: AchievementStringsDTO,
    #[serde(rename = "type")]
    pub type_: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementStringsDTO {
    pub caption: String,
    pub description: String,
    pub unachieved: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementPlayerDTO {
    pub id: u32,
    pub player: [u32; 4],
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationDTO {
    pub token: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchDTO {
    pub options: Vec<DashlaunchOptionDTO>,
    pub version: DashlaunchVersionDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchOptionDTO {
    pub id: u32,
    pub category: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchVersionDTO {
    pub kernel: u32,
    pub number: DashlaunchVersionNumberDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchVersionNumberDTO {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
}

/// For large files, the `size` in the response may be a negative number.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilebrowserEntryDTO {
    pub name: String,
    pub attributes: u32,
    pub size: i32,
}

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FilebrowserEntryDTO {
//     pub name: String,
//     pub attributes: u32,
//     pub size: u32,
// }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryDTO {
    pub free: u32,
    pub total: u32,
    pub used: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultidiscDTO {
    pub disc: MultidiscDiscDTO,
    pub entries: [MultidiscEntryDTO; 5],
    pub titleid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultidiscDiscDTO {
    pub current: u32,
    pub total: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultidiscEntryDTO {
    pub container: u32,
    pub path: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginDTO {
    pub features: PluginFeaturesDTO,
    pub path: PluginPathDTO,
    pub version: PluginVersionDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginFeaturesDTO {
    pub achievements: u32,
    pub debugger: u32,
    pub gamepad: u32,
    pub httpdaemon: u32,
    pub multidisc: u32,
    pub network: u32,
    pub systemlink: u32,
    pub threads: u32,
    pub trainers: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginPathDTO {
    pub launcher: String,
    pub root: String,
    pub user: String,
    pub web: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginVersionDTO {
    pub api: u32,
    pub number: PluginVersionNumberDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginVersionNumberDTO {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
    #[serde(rename = "type")]
    pub type_: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDTO {
    pub gamerscore: u32,
    pub gamertag: String,
    pub index: u32,
    pub signedin: u32,
    pub xuid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMetaDTO {
    pub filename: String,
    pub filesize: u32,
    pub info: ScreencaptureMetaInfoDTO,
    pub timestamp: String,
    pub titleid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMetaInfoDTO {
    pub format: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMetaListCountDTO {
    pub total: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmcDTO {
    pub avpack: u32,
    pub dvdmediatype: u32,
    pub smcversion: String,
    pub temperature: SmcTemperatureDTO,
    pub tiltstate: u32,
    pub traystate: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmcTemperatureDTO {
    pub celsius: bool,
    pub max: SmcTemperatureValuesDTO,
    pub target: SmcTemperatureValuesDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmcTemperatureValuesDTO {
    pub cpu: f32,
    pub gpu: f32,
    pub memory: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDTO {
    pub console: SystemConsoleDTO,
    pub consoleid: String,
    pub cpukey: String,
    pub dvdkey: String,
    pub serial: String,
    pub version: SystemVersionDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConsoleDTO {
    pub motherboard: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkDTO {
    pub apikey: String,
    pub broadcastport: u32,
    pub dataport: u32,
    pub enabled: u32,
    pub gatewayip: String,
    pub gatewaymac: String,
    pub username: String,
    pub xboxip: String,
    pub xboxmac: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkBandwidthDTO {
    pub bytes: SystemlinkBandwidthBytesDTO,
    pub rate: SystemlinkBandwidthRateDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkBandwidthBytesDTO {
    pub downstream: u32,
    pub upstream: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkBandwidthRateDTO {
    pub downstream: f32,
    pub upstream: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemVersionDTO {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
    pub qfe: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureDTO {
    pub case: f32,
    pub celsius: bool,
    pub cpu: f32,
    pub gpu: f32,
    pub memory: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadDTO {
    pub address: String,
    pub flags: String,
    pub id: String,
    pub priority: u32,
    pub state: u32,
    #[serde(rename = "type")]
    pub type_: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadStateDTO {
    pub state: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleDTO {
    pub disc: TitleDiscDTO,
    pub mediaid: String,
    pub path: String,
    pub resolution: TitleResolutionDTO,
    pub titleid: String,
    pub tuver: u32,
    pub version: TitleVersionDTO,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleDiscDTO {
    pub count: u32,
    pub current: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleResolutionDTO {
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleVersionDTO {
    pub base: String,
    pub current: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationDTO {
    pub achievements: u32,
    pub profiles: u32,
    pub screencapture: u32,
    pub title: u32,
}
