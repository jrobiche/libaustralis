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
use crate::aurora::http_schemas;
use crate::utils::GenericResult;
use log::error;

#[derive(Clone, Debug)]
pub struct HttpClient {
    ip: String,
    port: usize,
    username: Option<String>,
    password: Option<String>,
}

impl HttpClient {
    pub fn new(
        ip: String,
        port: usize,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self {
            ip,
            port,
            username,
            password,
        }
    }

    async fn delete(
        &self,
        token: Option<&str>,
        endpoint: &str,
        query: Option<&Vec<(&str, &str)>>,
    ) -> GenericResult<reqwest::Response> {
        let client = reqwest::Client::new();
        let req = client.delete(format!("http://{}:{}{}", self.ip, self.port, endpoint));
        let req = match &token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        let req = match &query {
            Some(q) => req.query(q),
            _ => req,
        };
        match req.send().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to make DELETE request to '{}'. Got the following error: {}",
                    endpoint, err
                );
                Err(format!("Failed to make DELETE request to '{}'.", endpoint).into())
            }
        }
    }

    async fn get(
        &self,
        token: Option<&str>,
        endpoint: &str,
        query: Option<&Vec<(&str, &str)>>,
    ) -> GenericResult<reqwest::Response> {
        let client = reqwest::Client::new();
        let req = client.get(format!("http://{}:{}{}", self.ip, self.port, endpoint));
        let req = match &token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        let req = match &query {
            Some(q) => req.query(q),
            _ => req,
        };
        match req.send().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to make GET request to '{}'. Got the following error: {}",
                    endpoint, err
                );
                Err(format!("Failed to make GET request to '{}'.", endpoint).into())
            }
        }
    }

    async fn post(
        &self,
        token: Option<&str>,
        endpoint: &str,
        params: Vec<(&str, &str)>,
    ) -> GenericResult<reqwest::Response> {
        let client = reqwest::Client::new();
        let req = client.post(format!("http://{}:{}{}", self.ip, self.port, endpoint));
        let req = match &token {
            Some(t) => req.bearer_auth(t),
            _ => req,
        };
        match req.form(&params).send().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to make POST request to '{}'. Got the following error: {}",
                    endpoint, err
                );
                Err(format!("Failed to make POST request to '{}'.", endpoint).into())
            }
        }
    }

    pub async fn new_token(&self) -> GenericResult<Option<String>> {
        match (&self.username, &self.password) {
            (Some(user), Some(pass)) => {
                let resp = self.post_authenticate(&user, &pass).await?;
                Ok(Some(resp.token))
            }
            _ => Ok(None),
        }
    }

    // achievement endpoints
    pub async fn get_achievement(
        &self,
        token: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::Achievement>> {
        let resp = self.get(token, "/achievement", None).await?;
        match resp.json::<Vec<http_schemas::Achievement>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Achievement from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Achievement from response.".into())
            }
        }
    }

    pub async fn get_achievement_player(
        &self,
        token: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::AchievementPlayer>> {
        let resp = self.get(token, "/achievement/player", None).await?;
        match resp.json::<Vec<http_schemas::AchievementPlayer>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse AchievementPlayer from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse AchievementPlayer from response.".into())
            }
        }
    }

    // authenticate endpoints
    pub async fn post_authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> GenericResult<http_schemas::Authentication> {
        let params = vec![("username", username), ("password", password)];
        let resp = self.post(None, "/authenticate", params).await?;
        match resp.json::<http_schemas::Authentication>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Authentication from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Authentication from response.".into())
            }
        }
    }

    // dashlaunch endpoints
    pub async fn get_dashlaunch(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::Dashlaunch> {
        let resp = self.get(token, "/dashlaunch", None).await?;
        match resp.json::<http_schemas::Dashlaunch>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Dashlaunch from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Dashlaunch from response.".into())
            }
        }
    }

    // filebrowser endpoints
    pub async fn get_filebrowser(
        &self,
        token: Option<&str>,
        path: Option<&str>,
        filter: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::FilebrowserEntry>> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        match &path {
            Some(x) => params.push(("path", &x)),
            _ => (),
        }
        match &filter {
            Some(x) => params.push(("filter", &x)),
            _ => (),
        }
        let resp = self.get(token, "/filebrowser", Some(&params)).await?;
        match resp.json::<Vec<http_schemas::FilebrowserEntry>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse FilebrowserEntry's from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse FilebrowserEntry's from response.".into())
            }
        }
    }

    // image endpoints
    pub async fn get_image_achievement(
        &self,
        token: Option<&str>,
        uuid: &str,
    ) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self.get(token, "/image/achievement", Some(&params)).await?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get bytes from response.".into())
            }
        }
    }

    pub async fn get_image_profile(
        &self,
        token: Option<&str>,
        uuid: &str,
    ) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self.get(token, "/image/profile", Some(&params)).await?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get bytes from response.".into())
            }
        }
    }

    pub async fn get_image_screencapture(
        &self,
        token: Option<&str>,
        uuid: &str,
    ) -> GenericResult<Vec<u8>> {
        let params: Vec<(&str, &str)> = vec![("uuid", uuid)];
        let resp = self
            .get(token, "/image/screencapture", Some(&params))
            .await?;
        match resp.bytes().await {
            Ok(x) => Ok(x.to_vec()),
            Err(err) => {
                error!(
                    "Failed to get bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get bytes from response.".into())
            }
        }
    }

    // memory endpoints
    pub async fn get_memory(&self, token: Option<&str>) -> GenericResult<http_schemas::Memory> {
        let resp = self.get(token, "/memory", None).await?;
        match resp.json::<http_schemas::Memory>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Memory from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Memory from response.".into())
            }
        }
    }

    // multidisc endpoints
    pub async fn get_multidisc(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::Multidisc> {
        let resp = self.get(token, "/multidisc", None).await?;
        match resp.json::<http_schemas::Multidisc>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Multidisc from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Multidisc from response.".into())
            }
        }
    }

    // plugin endpoints
    pub async fn get_plugin(&self, token: Option<&str>) -> GenericResult<http_schemas::Plugin> {
        let resp = self.get(token, "/plugin", None).await?;
        match resp.json::<http_schemas::Plugin>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Plugin from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Plugin from response.".into())
            }
        }
    }

    // profile endpoints
    pub async fn get_profile(
        &self,
        token: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::Profile>> {
        let resp = self.get(token, "/profile", None).await?;
        match resp.json::<Vec<http_schemas::Profile>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Profile from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Profile from response.".into())
            }
        }
    }

    // screencapture endpoints
    pub async fn delete_screencapture(&self, token: Option<&str>, uuid: &str) -> GenericResult<()> {
        let params = vec![("uuid", uuid)];
        match self.delete(token, "/screencapture", Some(&params)).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(
                    "Failed to delete screencapture. Got the following error: {}",
                    err
                );
                return Err("Failed to delete screencapture.".into());
            }
        }
    }

    pub async fn get_screencapture_meta(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::ScreencaptureMeta> {
        let resp = self.get(token, "/screencapture/meta", None).await?;
        match resp.json::<http_schemas::ScreencaptureMeta>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse ScreencaptureMeta from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse ScreencaptureMeta from response.".into())
            }
        }
    }

    pub async fn get_screencapture_meta_list(
        &self,
        token: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::ScreencaptureMeta>> {
        let resp = self.get(token, "/screencapture/meta/list", None).await?;
        match resp.json::<Vec<http_schemas::ScreencaptureMeta>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse ScreencaptureMetas from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse ScreencaptureMetas from response.".into())
            }
        }
    }

    pub async fn get_screencapture_meta_list_count(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::ScreencaptureMetaListCount> {
        let resp = self
            .get(token, "/screencapture/meta/list/count", None)
            .await?;
        match resp
            .json::<http_schemas::ScreencaptureMetaListCount>()
            .await
        {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse ScreencaptureMetaListCount from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse ScreencaptureMetaListCount from response.".into())
            }
        }
    }

    // smc endpoints
    pub async fn get_smc(&self, token: Option<&str>) -> GenericResult<http_schemas::Smc> {
        let resp = self.get(token, "/smc", None).await?;
        match resp.json::<http_schemas::Smc>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Smc from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Smc from response.".into())
            }
        }
    }

    // system endpoints
    pub async fn get_system(&self, token: Option<&str>) -> GenericResult<http_schemas::System> {
        let resp = self.get(token, "/system", None).await?;
        match resp.json::<http_schemas::System>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse System from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse System from response.".into())
            }
        }
    }

    // systemlink endpoints
    pub async fn get_systemlink(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::Systemlink> {
        let resp = self.get(token, "/systemlink", None).await?;
        match resp.json::<http_schemas::Systemlink>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Systemlink from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Systemlink from response.".into())
            }
        }
    }

    pub async fn get_systemlink_bandwidth(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::SystemlinkBandwidth> {
        let resp = self.get(token, "/systemlink/bandwidth", None).await?;
        match resp.json::<http_schemas::SystemlinkBandwidth>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse SystemlinkBandwidth from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse SystemlinkBandwidth from response.".into())
            }
        }
    }

    // temperature endpoints
    pub async fn get_temperature(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::Temperature> {
        let resp = self.get(token, "/temperature", None).await?;
        match resp.json::<http_schemas::Temperature>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Temperature from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Temperature from response.".into())
            }
        }
    }

    // thread endpoints
    pub async fn get_thread(
        &self,
        token: Option<&str>,
    ) -> GenericResult<Vec<http_schemas::Thread>> {
        let resp = self.get(token, "/thread", None).await?;
        match resp.json::<Vec<http_schemas::Thread>>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Threads from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Threads from response.".into())
            }
        }
    }

    pub async fn get_thread_state(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::ThreadState> {
        let resp = self.get(token, "/thread/state", None).await?;
        match resp.json::<http_schemas::ThreadState>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse ThreadState from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse ThreadState from response.".into())
            }
        }
    }

    pub async fn post_thread_state(&self, token: Option<&str>, suspend: bool) -> GenericResult<()> {
        let suspend_value = match suspend {
            false => "0",
            true => "1",
        };
        let params = vec![("suspend", suspend_value)];
        match self.post(token, "/thread/state", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(
                    "Failed to set thread state. Got the following error: {}",
                    err
                );
                return Err("Failed to set thread state.".into());
            }
        }
    }

    // title endpoints
    pub async fn get_title(&self, token: Option<&str>) -> GenericResult<http_schemas::Title> {
        let resp = self.get(token, "/title", None).await?;
        match resp.json::<http_schemas::Title>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Title from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Title from response.".into())
            }
        }
    }

    pub async fn get_title_file(&self, token: Option<&str>, path: &str) -> GenericResult<Vec<u8>> {
        let params = vec![("path", path)];
        let resp = self.get(token, "/title/file", Some(&params)).await?;
        match resp.bytes().await {
            Ok(x) => {
                let mut data: Vec<u8> = Vec::new();
                for e in x {
                    data.push(e);
                }
                Ok(data)
            }
            Err(err) => {
                error!(
                    "Failed to get Title File bytes from response. Got the following error: {}",
                    err
                );
                Err("Failed to get Title File bytes from response.".into())
            }
        }
    }

    pub async fn post_title_launch(
        &self,
        token: Option<&str>,
        path: &str,
        exec: &str,
        exec_type: u32,
    ) -> GenericResult<()> {
        let exec_type_string = format!("{}", exec_type);
        let params = vec![("path", path), ("exec", exec), ("type", &exec_type_string)];
        match self.post(token, "/title/launch", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Failed to launch title. Got the following error: {}", err);
                return Err("Failed to launch title.".into());
            }
        }
    }

    pub async fn get_title_live_cache(&self, token: Option<&str>) -> GenericResult<String> {
        let resp = self.get(token, "/title/live/cache", None).await?;
        match resp.text().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse String from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse String from response.".into())
            }
        }
    }

    pub async fn post_title_live_cache(
        &self,
        token: Option<&str>,
        liveinfo: &str,
    ) -> GenericResult<()> {
        let params = vec![("liveinfo", liveinfo)];
        match self.post(token, "/title/live/cache", params).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(
                    "Failed to set title live cache. Got the following error: {}",
                    err
                );
                return Err("Failed to set title live cache.".into());
            }
        }
    }

    // update endpoints
    pub async fn get_update_notification(
        &self,
        token: Option<&str>,
    ) -> GenericResult<http_schemas::UpdateNotification> {
        let resp = self.get(token, "/update/notification", None).await?;
        match resp.json::<http_schemas::UpdateNotification>().await {
            Ok(x) => Ok(x),
            Err(err) => {
                error!(
                    "Failed to parse Update Notification from response. Got the following error: {}",
                    err
                );
                Err("Failed to parse Update Notification from response.".into())
            }
        }
    }
}
