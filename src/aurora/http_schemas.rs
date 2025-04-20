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
use serde;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub cred: u32,
    pub hidden: u32,
    pub id: u32,
    pub imageid: u32,
    pub strings: AchievementStrings,
    #[serde(rename = "type")]
    pub type_: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementStrings {
    pub caption: String,
    pub description: String,
    pub unachieved: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementPlayer {
    pub id: u32,
    pub player: [u32; 4],
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub token: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dashlaunch {
    pub options: Vec<DashlaunchOption>,
    pub version: DashlaunchVersion,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchOption {
    pub id: u32,
    pub category: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchVersion {
    pub kernel: u32,
    pub number: DashlaunchVersionNumber,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashlaunchVersionNumber {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilebrowserEntry {
    pub name: String,
    pub attributes: u32,
    pub size: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub free: u32,
    pub total: u32,
    pub used: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Multidisc {
    pub disc: MultidiscDisc,
    pub entries: [MultidiscEntry; 5],
    pub titleid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultidiscDisc {
    pub current: u32,
    pub total: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultidiscEntry {
    pub container: u32,
    pub path: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub features: PluginFeatures,
    pub path: PluginPath,
    pub version: PluginVersion,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginFeatures {
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
pub struct PluginPath {
    pub launcher: String,
    pub root: String,
    pub user: String,
    pub web: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginVersion {
    pub api: u32,
    pub number: PluginVersionNumber,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginVersionNumber {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
    #[serde(rename = "type")]
    pub type_: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub gamerscore: u32,
    pub gamertag: String,
    pub index: u32,
    pub signedin: u32,
    pub xuid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMeta {
    pub filename: String,
    pub filesize: u32,
    pub info: ScreencaptureMetaInfo,
    pub timestamp: String,
    pub titleid: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMetaInfo {
    pub format: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreencaptureMetaListCount {
    pub total: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Smc {
    pub avpack: u32,
    pub dvdmediatype: u32,
    pub smcversion: String,
    pub temperature: SmcTemperature,
    pub tiltstate: u32,
    pub traystate: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmcTemperature {
    pub celsius: bool,
    pub max: SmcTemperatureValues,
    pub target: SmcTemperatureValues,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmcTemperatureValues {
    pub cpu: f32,
    pub gpu: f32,
    pub memory: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub console: SystemConsole,
    pub consoleid: String,
    pub cpukey: String,
    pub dvdkey: String,
    pub serial: String,
    pub version: SystemVersion,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConsole {
    pub motherboard: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Systemlink {
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
pub struct SystemlinkBandwidth {
    pub bytes: SystemlinkBandwidthBytes,
    pub rate: SystemlinkBandwidthRate,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkBandwidthBytes {
    pub downstream: u32,
    pub upstream: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemlinkBandwidthRate {
    pub downstream: f32,
    pub upstream: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemVersion {
    pub build: u32,
    pub major: u32,
    pub minor: u32,
    pub qfe: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub case: f32,
    pub celsius: bool,
    pub cpu: f32,
    pub gpu: f32,
    pub memory: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
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
pub struct ThreadState {
    pub state: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub disc: TitleDisc,
    pub mediaid: String,
    pub path: String,
    pub resolution: TitleResolution,
    pub titleid: String,
    pub tuver: u32,
    pub version: TitleVersion,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleDisc {
    pub count: u32,
    pub current: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleResolution {
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleVersion {
    pub base: String,
    pub current: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotification {
    pub achievements: u32,
    pub profiles: u32,
    pub screencapture: u32,
    pub title: u32,
}
