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

// TODO define tests

/// Data Transfer Object definitions
pub mod dto;
use crate::utils::GenericResult;
use dto::*;
use log::error;

/// Higher level wrapper of AuroraHttpClient
#[derive(Clone, Debug)]
pub struct HttpClient {
    http_client: AuroraHttpClient,
}

impl HttpClient {
    /// Create a new `HttpClient` instance.
    pub fn new(ip: &str, port: usize) -> Self {
        Self {
            http_client: AuroraHttpClient::new(ip, port),
        }
    }

    /// Manually set the authentication token.
    pub fn set_authentication_token(&mut self, token: Option<&str>) -> () {
        self.http_client.set_token(token)
    }

    /// Set authentication token by requesting a token from the console using given credentials.
    pub async fn set_authentication_token_from_credentials(
        &mut self,
        username: &str,
        password: &str,
    ) -> GenericResult<AuthenticationDTO> {
        let response = self
            .http_client
            .post_authenticate(username, password)
            .await?;
        self.http_client.set_token(Some(&response.token));
        Ok(response)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // achievement methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get the list of achievements for the running title.
    ///
    /// If the game does not have any achievements, then an empty list is returned.
    pub async fn achievements(&self) -> GenericResult<Vec<AchievementDTO>> {
        self.http_client.get_achievement().await
    }

    /// Get the icon image for an achievement in the currently running title.
    ///
    /// The value of `achievement_imageid` should be the Achievement's `imageid` value.
    ///
    /// The bytes returned make up a PNG image.
    pub async fn achievement_image_bytes(
        &self,
        achievement_imageid: u32,
    ) -> GenericResult<Vec<u8>> {
        let uuid = format!("{}", achievement_imageid);
        self.http_client.get_image_achievement(&uuid).await
    }

    /// Get information about which signed in players have unlocked which achievements for the running title.
    pub async fn achievement_unlock_status_by_player(
        &self,
    ) -> GenericResult<Vec<AchievementPlayerDTO>> {
        self.http_client.get_achievement_player().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // authenticate methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Authenticate with console to request a new token.
    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> GenericResult<AuthenticationDTO> {
        self.http_client.post_authenticate(username, password).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // dashlaunch methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about DashLaunch.
    pub async fn dashlaunch(&self) -> GenericResult<DashlaunchDTO> {
        self.http_client.get_dashlaunch().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // filebrowser methods
    ////////////////////////////////////////////////////////////////////////////////
    // TODO document that it only works with paths under a certain number of characters
    /// List files.
    pub async fn files(
        &self,
        path: Option<&str>,
        filter: Option<&str>,
    ) -> GenericResult<Vec<FilebrowserEntryDTO>> {
        // TODO fix value of `size` property
        self.http_client.get_filebrowser(path, filter).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // memory methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get current RAM usage.
    pub async fn memory(&self) -> GenericResult<MemoryDTO> {
        self.http_client.get_memory().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // multidisc methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the different discs that make up the running title.
    ///
    /// Returns `None` if the title is not a multidisc title.
    pub async fn multidisc(&self) -> GenericResult<Option<MultidiscDTO>> {
        self.http_client.get_multidisc().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // plugin methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the NOVA plugin.
    pub async fn plugin(&self) -> GenericResult<PluginDTO> {
        self.http_client.get_plugin().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // profile methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get list of logged in profiles.
    pub async fn profiles(&self) -> GenericResult<Vec<ProfileDTO>> {
        self.http_client.get_profile().await
    }

    /// Get the avatar for a signed in profile.
    ///
    /// The bytes returned make up a BMP (PC bitmap, Windows 95/NT4 and newer format) image.
    pub async fn profile_image_bytes(&self, player_index: PlayerIndex) -> GenericResult<Vec<u8>> {
        let uuid = format!("{}", player_index.as_u32());
        self.http_client.get_image_profile(&uuid).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // screencapture methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Take a screencapture.
    pub async fn create_screencapture(&self) -> GenericResult<ScreencaptureMetaDTO> {
        self.http_client.get_screencapture_meta().await
    }

    /// Delete a screencapture.
    ///
    /// The value of `screencapture_filename` should be the screencapture's `filename` value as a `&str`.
    pub async fn delete_screencapture(&self, screencapture_filename: &str) -> GenericResult<()> {
        self.http_client
            .delete_screencapture(screencapture_filename)
            .await
    }

    /// Get list of screencaptures taken for the running title.
    pub async fn screencaptures(&self) -> GenericResult<Vec<ScreencaptureMetaDTO>> {
        self.http_client.get_screencapture_meta_list().await
    }

    /// Get the number of screencaptures taken for the running title.
    pub async fn screencapture_count(&self) -> GenericResult<u32> {
        let screencapture_meta_list_count =
            self.http_client.get_screencapture_meta_list_count().await?;
        Ok(screencapture_meta_list_count.total)
    }

    /// Get a screenshot that was taken in the currently running title.
    ///
    /// The value of `screencapture_filename` should be the screencapture's `filename` value.
    ///
    /// The bytes returned make up a BMP (PC bitmap, Windows 95/NT4 and newer format) image.
    pub async fn screencapture_image_bytes(
        &self,
        screencapture_filename: &str,
    ) -> GenericResult<Vec<u8>> {
        self.http_client
            .get_image_screencapture(screencapture_filename)
            .await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // smc methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the System Management Controller.
    pub async fn smc(&self) -> GenericResult<SmcDTO> {
        self.http_client.get_smc().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // system methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the console.
    pub async fn system(&self) -> GenericResult<SystemDTO> {
        self.http_client.get_system().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // systemlink methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about LiNK configuration.
    pub async fn systemlink(&self) -> GenericResult<SystemlinkDTO> {
        self.http_client.get_systemlink().await
    }

    /// Get information about LiNK network usage.
    pub async fn systemlink_bandwidth(&self) -> GenericResult<SystemlinkBandwidthDTO> {
        self.http_client.get_systemlink_bandwidth().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // temperature methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get console component temperatures.
    pub async fn temperature(&self) -> GenericResult<TemperatureDTO> {
        self.http_client.get_temperature().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // thread methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about threads.
    pub async fn threads(&self) -> GenericResult<Vec<ThreadDTO>> {
        self.http_client.get_thread().await
    }

    /// Determine if all threads are running.
    pub async fn threads_are_active(&self) -> GenericResult<bool> {
        let thread_state = self.http_client.get_thread_state().await?;
        Ok(thread_state.state == 0)
    }

    /// Determine if all threads are suspended.
    pub async fn threads_are_suspended(&self) -> GenericResult<bool> {
        let thread_state = self.http_client.get_thread_state().await?;
        Ok(thread_state.state == 2)
    }

    /// Set all thread states to active.
    pub async fn set_threads_active(&self) -> GenericResult<()> {
        self.http_client.post_thread_state("0").await
    }

    /// Set all thread states to suspended.
    pub async fn set_threads_suspended(&self) -> GenericResult<()> {
        self.http_client.post_thread_state("1").await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // title methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the running title.
    pub async fn title(&self) -> GenericResult<TitleDTO> {
        self.http_client.get_title().await
    }

    /// Download a file from the `Game:` drive.
    ///
    /// The value of `path` should be the path within the `Game:` drive.
    /// For example, to download the file at `Game:\fake\path\abc.txt` use the value `fake\path\abc.txt`
    ///
    /// Note: Does not appear to work with all files.
    pub async fn title_file_bytes(&self, path: &str) -> GenericResult<Vec<u8>> {
        self.http_client.get_title_file(path).await
    }

    /// Launch an executable.
    ///
    /// The value of `directory` should be the directory containing the executable file.
    ///
    /// The value of `name` should be the name of the executable file.
    pub async fn launch_title(
        &self,
        directory: RemotePath,
        name: &str,
        executable_type: ExecutableType,
    ) -> GenericResult<()> {
        let device_path = directory.as_device_string();
        let executable_type_string = format!("{}", executable_type.as_u32());
        self.http_client
            .post_title_launch(&device_path, name, &executable_type_string)
            .await
    }

    /// Get xbox live catalog information that has been cached for the running title.
    pub async fn title_xbox_catalog_cache(&self) -> GenericResult<String> {
        self.http_client.get_title_live_cache().await
    }

    /// Set xbox live catalog information for running title.
    pub async fn set_title_xbox_catalog_cache(&self, liveinfo: &str) -> GenericResult<()> {
        self.http_client.post_title_live_cache(liveinfo).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    // update methods
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about changes in achievements, profiles, screenshots, and titles launched.
    pub async fn update_notification(&self) -> GenericResult<UpdateNotificationDTO> {
        self.http_client.get_update_notification().await
    }
}

/// Direct implementation of the Aurora HTTP REST API
#[derive(Clone, Debug)]
pub struct AuroraHttpClient {
    http_client: reqwest::Client,
    host: String,
    token: Option<String>,
}

impl AuroraHttpClient {
    /// Create a new `AuroraHttpClient` instance.
    pub fn new(ip: &str, port: usize) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            host: format!("http://{}:{}", ip, port),
            token: None,
        }
    }

    /// Manually set the authentication token.
    pub fn set_token(&mut self, token: Option<&str>) -> () {
        match token {
            Some(x) => self.token = Some(String::from(x)),
            None => self.token = None,
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // achievement endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get the list of achievements for the running title.
    ///
    /// If the game does not have any achievements, then an empty list is returned.
    pub async fn get_achievement(&self) -> GenericResult<Vec<AchievementDTO>> {
        let resp = self.get("/achievement", None).await?;
        Self::error_for_status(&resp)?;
        // return an empty list if the game does not have any achievements
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(Vec::new());
        }
        resp.json::<Vec<AchievementDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse AchievementDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Get information about which signed in players have unlocked which achievements for the running title.
    pub async fn get_achievement_player(&self) -> GenericResult<Vec<AchievementPlayerDTO>> {
        let resp = self.get("/achievement/player", None).await?;
        Self::error_for_status(&resp)?;
        // return an empty list if the game does not have any achievements
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(Vec::new());
        }
        resp.json::<Vec<AchievementPlayerDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse AchievementPlayerDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // authenticate endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Authenticate with console to request a new token.
    pub async fn post_authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> GenericResult<AuthenticationDTO> {
        let params = vec![("username", username), ("password", password)];
        let resp = self.post("/authenticate", params).await?;
        Self::error_for_status(&resp)?;
        resp.json::<AuthenticationDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse AuthenticationDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // dashlaunch endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about DashLaunch.
    pub async fn get_dashlaunch(&self) -> GenericResult<DashlaunchDTO> {
        let resp = self.get("/dashlaunch", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<DashlaunchDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse DashlaunchDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // filebrowser endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// List files within the `Game:` directory.
    pub async fn get_filebrowser(
        &self,
        path: Option<&str>,
        filter: Option<&str>,
    ) -> GenericResult<Vec<FilebrowserEntryDTO>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        match &path {
            Some(x) => params.push(("path", &x)),
            _ => (),
        }
        match &filter {
            Some(x) => params.push(("filter", &x)),
            _ => (),
        }
        let resp = self.get("/filebrowser", Some(&params)).await?;
        Self::error_for_status(&resp)?;
        resp.json::<Vec<FilebrowserEntryDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse FilebrowserEntryDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // image endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get the icon for an achievement in the currently running title.
    ///
    /// The value of `uuid` should be the Achievement's `imageid` value as a `&str`.
    ///
    /// The bytes returned make up a PNG image.
    pub async fn get_image_achievement(&self, uuid: &str) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self.get("/image/achievement", Some(&params)).await?;
        Self::error_for_status(&resp)?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = format!(
                    "Failed to get bytes for achievement image. Got the following error: {}",
                    err
                );
                error!("{}", err);
                Err(msg.into())
            }
        }
    }

    /// Get the avatar for a signed in profile.
    ///
    /// The value of `uuid` should be the player index:
    /// `"0"` for Player 1, `"1"` for Player 2, `"2"` for Player 3, and `"3"` for Player 4.
    ///
    /// The bytes returned make up a BMP (PC bitmap, Windows 95/NT4 and newer format) image.
    pub async fn get_image_profile(&self, uuid: &str) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self.get("/image/profile", Some(&params)).await?;
        Self::error_for_status(&resp)?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = format!(
                    "Failed to get bytes for profile image. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    /// Get a screenshot that was taken in the currently running title.
    ///
    /// The value of `uuid` should be the screencapture's `filename` value as a `&str`.
    ///
    /// The bytes returned make up a BMP (PC bitmap, Windows 95/NT4 and newer format) image.
    pub async fn get_image_screencapture(&self, uuid: &str) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self.get("/image/screencapture", Some(&params)).await?;
        Self::error_for_status(&resp)?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = format!(
                    "Failed to get bytes for screencapture image. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // memory endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get current RAM usage.
    pub async fn get_memory(&self) -> GenericResult<MemoryDTO> {
        let resp = self.get("/memory", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<MemoryDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse MemoryDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // multidisc endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the different discs that make up the running title.
    pub async fn get_multidisc(&self) -> GenericResult<Option<MultidiscDTO>> {
        let resp = self.get("/multidisc", None).await?;
        Self::error_for_status(&resp)?;
        // return `None` if the game does not have multiple discs
        if resp.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }
        match resp.json::<MultidiscDTO>().await {
            Ok(x) => Ok(Some(x)),
            Err(err) => {
                let msg = format!(
                    "Failed to parse MultidiscDTO from response. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // plugin endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the NOVA plugin.
    pub async fn get_plugin(&self) -> GenericResult<PluginDTO> {
        let resp = self.get("/plugin", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<PluginDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse PluginDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // profile endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get list of logged in profiles.
    pub async fn get_profile(&self) -> GenericResult<Vec<ProfileDTO>> {
        let resp = self.get("/profile", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<Vec<ProfileDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse ProfileDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // screencapture endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Delete a screencapture.
    ///
    /// The value of `uuid` should be the screencapture's `filename` value as a `&str`.
    pub async fn delete_screencapture(&self, uuid: &str) -> GenericResult<()> {
        let params = vec![("uuid", uuid)];
        match self.delete("/screencapture", Some(&params)).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!("Failed to make DELETE request to /screencapture endpoint. Got the following error: {}", err);
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    /// Take a screencapture.
    pub async fn get_screencapture_meta(&self) -> GenericResult<ScreencaptureMetaDTO> {
        let resp = self.get("/screencapture/meta", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<ScreencaptureMetaDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse ScreencaptureMetaDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Get list of screencaptures taken for the running title.
    pub async fn get_screencapture_meta_list(&self) -> GenericResult<Vec<ScreencaptureMetaDTO>> {
        let resp = self.get("/screencapture/meta/list", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<Vec<ScreencaptureMetaDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse ScreencaptureMetaDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Get the number of screencaptures taken for the running title.
    pub async fn get_screencapture_meta_list_count(
        &self,
    ) -> GenericResult<ScreencaptureMetaListCountDTO> {
        let resp = self.get("/screencapture/meta/list/count", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<ScreencaptureMetaListCountDTO>()
            .await
            .map_err(|err| {
                let msg = format!(
                "Failed to parse ScreencaptureMetaListCountDTO from response. Got the following error: {}",
                err
            );
                error!("{}", msg);
                msg.into()
            })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // smc endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the System Management Controller.
    pub async fn get_smc(&self) -> GenericResult<SmcDTO> {
        let resp = self.get("/smc", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<SmcDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse SmcDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // system endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the console.
    pub async fn get_system(&self) -> GenericResult<SystemDTO> {
        let resp = self.get("/system", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<SystemDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse SystemDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // systemlink endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about LiNK configuration.
    pub async fn get_systemlink(&self) -> GenericResult<SystemlinkDTO> {
        let resp = self.get("/systemlink", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<SystemlinkDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse SystemlinkDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Get information about LiNK network usage.
    pub async fn get_systemlink_bandwidth(&self) -> GenericResult<SystemlinkBandwidthDTO> {
        let resp = self.get("/systemlink/bandwidth", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<SystemlinkBandwidthDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse SystemlinkBandwidthDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // temperature endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get console component temperatures.
    pub async fn get_temperature(&self) -> GenericResult<TemperatureDTO> {
        let resp = self.get("/temperature", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<TemperatureDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse TemperatureDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // thread endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about threads.
    pub async fn get_thread(&self) -> GenericResult<Vec<ThreadDTO>> {
        let resp = self.get("/thread", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<Vec<ThreadDTO>>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse ThreadDTO list from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Determine if all threads are active or suspended.
    pub async fn get_thread_state(&self) -> GenericResult<ThreadStateDTO> {
        let resp = self.get("/thread/state", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<ThreadStateDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse ThreadStateDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Set state of threads.
    ///
    /// A suspend value of `"0"` will resume all threads and `"1"` will suspend all threads.
    pub async fn post_thread_state(&self, suspend: &str) -> GenericResult<()> {
        let params = vec![("suspend", suspend)];
        match self.post("/thread/state", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!("Failed to make POST request to /thread/state endpoint. Got the following error: {}", err);
                error!("{}", msg);
                return Err(msg.into());
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // title endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about the running title.
    pub async fn get_title(&self) -> GenericResult<TitleDTO> {
        let resp = self.get("/title", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<TitleDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse TitleDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Download a file from the `Game:` directory.
    pub async fn get_title_file(&self, path: &str) -> GenericResult<Vec<u8>> {
        let params = vec![("path", path)];
        let resp = self.get("/title/file", Some(&params)).await?;
        Self::error_for_status(&resp)?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                let msg = format!(
                    "Failed to get Title File bytes from response. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    /// Launch an executable.
    ///
    /// The value of `path` should be the device path to directory containing the executable file.
    ///
    /// The value of `exec` should be the name of the executable file.
    ///
    /// The value of `exec_type` should be indicate the type of executable:
    /// `"0"` for Xbox 360 Executables (xex)
    /// `"1"` for Classic Xbox Executables (xbe)
    /// `"2"` for Xbox 360 Container
    /// `"3"` for Classic Xbox Continer
    /// `"4"` for XNA Container.
    pub async fn post_title_launch(
        &self,
        path: &str,
        exec: &str,
        exec_type: &str,
    ) -> GenericResult<()> {
        let params = vec![("path", path), ("exec", exec), ("type", exec_type)];
        match self.post("/title/launch", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!(
                    "Failed to make POST request to /title/launch. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                return Err(msg.into());
            }
        }
    }

    /// Get xbox live catalog information that has been cached for the running title.
    pub async fn get_title_live_cache(&self) -> GenericResult<String> {
        let resp = self.get("/title/live/cache", None).await?;
        // return an empty string if there is no cached xbox live catalog information
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(String::new());
        }
        Self::error_for_status(&resp)?;
        match resp.text().await {
            Ok(x) => Ok(x),
            Err(err) => {
                let msg = format!(
                    "Failed to parse String from response. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }

    /// Set xbox live catalog information for running title.
    pub async fn post_title_live_cache(&self, liveinfo: &str) -> GenericResult<()> {
        let params = vec![("liveinfo", liveinfo)];
        match self.post("/title/live/cache", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!(
                    "Failed to make POST request to /title/live/cache. Got the following error: {}",
                    err
                );
                error!("{}", msg);
                return Err(msg.into());
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // update endpoints
    ////////////////////////////////////////////////////////////////////////////////
    /// Get information about changes in achievements, profiles, screenshots, and titles launched.
    pub async fn get_update_notification(&self) -> GenericResult<UpdateNotificationDTO> {
        let resp = self.get("/update/notification", None).await?;
        Self::error_for_status(&resp)?;
        resp.json::<UpdateNotificationDTO>().await.map_err(|err| {
            let msg = format!(
                "Failed to parse UpdateNotificationDTO from response. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    // private methods
    ////////////////////////////////////////////////////////////////////////////////
    async fn delete(
        &self,
        endpoint: &str,
        query: Option<&Vec<(&str, &str)>>,
    ) -> GenericResult<reqwest::Response> {
        let req = self
            .http_client
            .delete(format!("{}{}", self.host, endpoint));
        let req = match &self.token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        let req = match &query {
            Some(q) => req.query(q),
            _ => req,
        };
        match req.send().await {
            Ok(x) => Ok(x),
            Err(err) => Err(err.into()),
        }
    }

    async fn get(
        &self,
        endpoint: &str,
        query: Option<&Vec<(&str, &str)>>,
    ) -> GenericResult<reqwest::Response> {
        let req = self.http_client.get(format!("{}{}", self.host, endpoint));
        let req = match &self.token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        let req = match &query {
            Some(q) => req.query(q),
            _ => req,
        };
        match req.send().await {
            Ok(x) => Ok(x),
            Err(err) => Err(err.into()),
        }
    }

    async fn post(
        &self,
        endpoint: &str,
        params: Vec<(&str, &str)>,
    ) -> GenericResult<reqwest::Response> {
        let req = self.http_client.post(format!("{}{}", self.host, endpoint));
        let req = match &self.token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        match req.form(&params).send().await {
            Ok(x) => Ok(x),
            Err(err) => Err(err.into()),
        }
    }

    fn error_for_status(response: &reqwest::Response) -> GenericResult<()> {
        match response.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(err) => {
                let msg = format!("{}", err);
                error!("{}", msg);
                Err(msg.into())
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum PlayerIndex {
    Player1 = 0,
    Player2 = 1,
    Player3 = 2,
    Player4 = 3,
}

impl PlayerIndex {
    pub fn from_u32(value: u32) -> GenericResult<Self> {
        let result = match value {
            0 => Self::Player1,
            1 => Self::Player2,
            2 => Self::Player3,
            3 => Self::Player4,
            _ => {
                let msg = format!("Could not convert value '{}' to PlayerIndex.", value);
                return Err(msg.into());
            }
        };
        Ok(result)
    }

    pub fn from_usize(value: usize) -> GenericResult<Self> {
        let result = match value {
            0 => Self::Player1,
            1 => Self::Player2,
            2 => Self::Player3,
            3 => Self::Player4,
            _ => {
                let msg = format!("Could not convert value '{}' to PlayerIndex.", value);
                return Err(msg.into());
            }
        };
        Ok(result)
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum ExecutableType {
    Xbox360 = 0,
    Xbox = 1,
    Xbox360Container = 2,
    XboxContainer = 3,
    XNA = 4,
}

impl ExecutableType {
    pub fn from_u32(value: u32) -> GenericResult<Self> {
        let result = match value {
            0 => Self::Xbox360,
            1 => Self::Xbox,
            2 => Self::Xbox360Container,
            3 => Self::XboxContainer,
            4 => Self::XNA,
            _ => {
                let msg = format!("Could not convert value '{}' to ExecutableType.", value);
                return Err(msg.into());
            }
        };
        Ok(result)
    }

    pub fn from_usize(value: usize) -> GenericResult<Self> {
        let result = match value {
            0 => Self::Xbox360,
            1 => Self::Xbox,
            2 => Self::Xbox360Container,
            3 => Self::XboxContainer,
            4 => Self::XNA,
            _ => {
                let msg = format!("Could not convert value '{}' to ExecutableType.", value);
                return Err(msg.into());
            }
        };
        Ok(result)
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

/// Representation of a path on the console.
///
/// Can be used to convert between device paths and drive paths, e.g. `\Device\Harddisk0\Partition1\fake\path` to `Hdd1:\fake\path`.
#[derive(Clone, Debug)]
pub struct RemotePath {
    root: RemotePathRoot,
    path: Option<String>,
}

impl RemotePath {
    pub fn new(path: &str) -> GenericResult<Self> {
        let msg = format!("Could not create RemotePath from path '{}'.", path);
        let error = Err(msg.into());
        Self::from_device_str(path)
            .or(Self::from_drive_str(path))
            .or(error)
    }

    pub fn from_device_str(value: &str) -> GenericResult<Self> {
        let root = RemotePathRoot::from_device_str(value)?;
        let root_string = root.as_device_string();
        let mut path_str = "";
        if value.len() > root_string.len() {
            path_str = &value[root_string.len()..value.len()];
            if path_str.starts_with("\\") {
                path_str = &path_str[1..path_str.len()];
            }
        }
        let path = match path_str {
            "" => None,
            _ => Some(String::from(path_str)),
        };
        Ok(Self { root, path })
    }

    pub fn from_drive_str(value: &str) -> GenericResult<Self> {
        let root = RemotePathRoot::from_drive_str(value)?;
        let root_string = root.as_drive_string();
        let mut path_str = "";
        if value.len() > root_string.len() {
            path_str = &value[root_string.len()..value.len()];
            if path_str.starts_with("\\") {
                path_str = &path_str[1..path_str.len()];
            }
        }
        let path = match path_str {
            "" => None,
            _ => Some(String::from(path_str)),
        };
        Ok(Self { root, path })
    }

    pub fn as_device_string(&self) -> String {
        let mut path = self.root.as_device_string();
        match &self.path {
            Some(x) => path.push_str(&format!("\\{}", &x)),
            None => path.push_str("\\"),
        }
        path
    }

    pub fn as_drive_string(&self) -> String {
        let mut path = self.root.as_drive_string();
        match &self.path {
            Some(x) => path.push_str(&format!("\\{}", &x)),
            None => path.push_str("\\"),
        }
        path
    }
}

/// Representation of a path root on the console.
///
/// Can be used to convert between device roots and drive roots, e.g. `\Device\Harddisk0\Partition1` to `Hdd1:`.
///
/// Note: This does not support the `OnBoardMU:` drive because there is no one to one mapping between that drive root and a device root.
/// The `OnBoardMU:` drive root could be any of the following device roots:
/// `\Device\BuiltInMuMmc\Storage`, `\Device\BuiltInMuSfc`, `\Device\BuiltInMuUsb\Storage`.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum RemotePathRoot {
    Dvd,
    Flash,
    HdDvdPlayer,
    HdDvdStorage,
    Hdd0,
    Hdd1,
    Hddx,
    Memunit0,
    Memunit1,
    SysExt,
    Transfercable,
    TransfercableXbox1,
    USBMU0,
    USBMU1,
    USBMU2,
    USBMUCache0,
    USBMUCache1,
    USBMUCache2,
    Usb0,
    Usb1,
    Usb2,
}

impl RemotePathRoot {
    pub fn as_device_string(&self) -> String {
        let value = match *self {
            Self::Dvd => "\\Device\\Cdrom0",
            Self::Flash => "\\SystemRoot",
            Self::HdDvdPlayer => "\\Device\\HdDvdPlayer",
            Self::HdDvdStorage => "\\Device\\HdDvdStorage",
            Self::Hdd0 => "\\Device\\Harddisk0\\Partition0",
            Self::Hdd1 => "\\Device\\Harddisk0\\Partition1",
            Self::Hddx => "\\Device\\Harddisk0\\SystemPartition",
            Self::Memunit0 => "\\Device\\Mu0",
            Self::Memunit1 => "\\Device\\Mu1",
            Self::SysExt => "\\Sep",
            Self::Transfercable => "\\Device\\Transfercable",
            Self::TransfercableXbox1 => "\\Device\\Transfercable\\Compatibility\\Xbox1",
            Self::USBMU0 => "\\Device\\Mass0PartitionFile\\Storage",
            Self::USBMU1 => "\\Device\\Mass1PartitionFile\\Storage",
            Self::USBMU2 => "\\Device\\Mass2PartitionFile\\Storage",
            Self::USBMUCache0 => "\\Device\\Mass0PartitionFile\\StorageSystem",
            Self::USBMUCache1 => "\\Device\\Mass1PartitionFile\\StorageSystem",
            Self::USBMUCache2 => "\\Device\\Mass2PartitionFile\\StorageSystem",
            Self::Usb0 => "\\Device\\Mass0",
            Self::Usb1 => "\\Device\\Mass1",
            Self::Usb2 => "\\Device\\Mass2",
        };
        String::from(value)
    }

    pub fn as_drive_string(&self) -> String {
        let value = match *self {
            Self::Dvd => "Dvd:",
            Self::Flash => "Flash:",
            Self::HdDvdPlayer => "HdDvdPlayer:",
            Self::HdDvdStorage => "HdDvdStorage:",
            Self::Hdd0 => "Hdd0:",
            Self::Hdd1 => "Hdd1:",
            Self::Hddx => "Hddx:",
            Self::Memunit0 => "Memunit0:",
            Self::Memunit1 => "Memunit1:",
            Self::SysExt => "SysExt:",
            Self::Transfercable => "Transfercable:",
            Self::TransfercableXbox1 => "TransfercableXbox1:",
            Self::USBMU0 => "USBMU0:",
            Self::USBMU1 => "USBMU1:",
            Self::USBMU2 => "USBMU2:",
            Self::USBMUCache0 => "USBMUCache0:",
            Self::USBMUCache1 => "USBMUCache1:",
            Self::USBMUCache2 => "USBMUCache2:",
            Self::Usb0 => "USB0:",
            Self::Usb1 => "USB1:",
            Self::Usb2 => "USB2:",
        };
        String::from(value)
    }

    pub fn from_device_str(path: &str) -> GenericResult<Self> {
        let map = vec![
            ("\\device\\cdrom0\\", Self::Dvd),
            ("\\systemroot\\", Self::Flash),
            ("\\device\\hddvdplayer\\", Self::HdDvdPlayer),
            ("\\device\\hddvdstorage\\", Self::HdDvdStorage),
            ("\\device\\harddisk0\\partition0\\", Self::Hdd0),
            ("\\device\\harddisk0\\partition1\\", Self::Hdd1),
            ("\\device\\harddisk0\\systempartition\\", Self::Hddx),
            ("\\device\\mu0\\", Self::Memunit0),
            ("\\device\\mu1\\", Self::Memunit1),
            ("\\sep\\", Self::SysExt),
            ("\\device\\transfercable\\", Self::Transfercable),
            (
                "\\device\\transfercable\\compatibility\\xbox1\\",
                Self::TransfercableXbox1,
            ),
            ("\\device\\mass0partitionfile\\storage\\", Self::USBMU0),
            ("\\device\\mass1partitionfile\\storage\\", Self::USBMU1),
            ("\\device\\mass2partitionfile\\storage\\", Self::USBMU2),
            (
                "\\device\\mass0partitionfile\\storagesystem\\",
                Self::USBMUCache0,
            ),
            (
                "\\device\\mass1partitionfile\\storagesystem\\",
                Self::USBMUCache1,
            ),
            (
                "\\device\\mass2partitionfile\\storagesystem\\",
                Self::USBMUCache2,
            ),
            ("\\device\\mass0\\", Self::Usb0),
            ("\\device\\mass1\\", Self::Usb1),
            ("\\device\\mass2\\", Self::Usb2),
        ];
        let compare_path = format!("{}\\", path).to_lowercase();
        for (device_root_str, enum_value) in map {
            if compare_path.starts_with(device_root_str) {
                return Ok(enum_value);
            }
        }
        let msg = format!(
            "Could not determine RemotePathRoot from device string '{}'.",
            path
        );
        return Err(msg.into());
    }

    pub fn from_drive_str(path: &str) -> GenericResult<Self> {
        let map = vec![
            ("dvd:\\", Self::Dvd),
            ("flash:\\", Self::Flash),
            ("hddvdplayer:\\", Self::HdDvdPlayer),
            ("hddvdstorage:\\", Self::HdDvdStorage),
            ("hdd0:\\", Self::Hdd0),
            ("hdd1:\\", Self::Hdd1),
            ("hddx:\\", Self::Hddx),
            ("memunit0:\\", Self::Memunit0),
            ("memunit1:\\", Self::Memunit1),
            ("sysext:\\", Self::SysExt),
            ("transfercable:\\", Self::Transfercable),
            ("transfercablexbox1:\\", Self::TransfercableXbox1),
            ("usbmu0:\\", Self::USBMU0),
            ("usbmu1:\\", Self::USBMU1),
            ("usbmu2:\\", Self::USBMU2),
            ("usbmucache0:\\", Self::USBMUCache0),
            ("usbmucache1:\\", Self::USBMUCache1),
            ("usbmucache2:\\", Self::USBMUCache2),
            ("usb0:\\", Self::Usb0),
            ("usb1:\\", Self::Usb1),
            ("usb2:\\", Self::Usb2),
        ];
        let compare_path = format!("{}\\", path).to_lowercase();
        for (device_root_str, enum_value) in map {
            if compare_path.starts_with(device_root_str) {
                return Ok(enum_value);
            }
        }
        let msg = format!(
            "Could not determine RemotePathRoot from drive string '{}'.",
            path
        );
        return Err(msg.into());
    }

    pub fn from_str(path: &str) -> GenericResult<Self> {
        let msg = format!("Could not determine RemotePathRoot from string '{}'.", path);
        let error = Err(msg.into());
        Self::from_device_str(path)
            .or(Self::from_drive_str(path))
            .or(error)
    }
}
