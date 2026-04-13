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
// TODO improve logging
use std::fmt;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use crate::utils::{create_file, GenericResult};
use log::error;

pub use suppaftp::types::FileType;

/// Higher level wrapper of AuroraFtpClient.
pub struct FtpClient {
    ip: String,
    port: usize,
    username: Option<String>,
    password: Option<String>,
    transfer_type: FileType,
    cwd: FtpPath,
}

impl FtpClient {
    /// Create a new instance of `FtpClient`
    pub fn new(ip: &str, port: usize) -> Self {
        Self {
            ip: String::from(ip),
            port,
            username: None,
            password: None,
            transfer_type: FileType::Binary,
            cwd: FtpPath::from_str("/"),
        }
    }

    /// Manually set the authentication credentials.
    pub fn set_credentials(&mut self, username: Option<&str>, password: Option<&str>) -> () {
        self.username = match username {
            Some(x) => Some(String::from(x)),
            None => None,
        };
        self.password = match password {
            Some(x) => Some(String::from(x)),
            None => None,
        };
    }

    /// Set current working directory to `path`.
    pub fn cwd(&mut self, path: &FtpPath) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        match ftp_client.cwd(&path.as_string()) {
            Ok(_) => {
                self.cwd = path.clone();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Get the path to the current working directory.
    pub fn pwd(&self) -> FtpPath {
        self.cwd.clone()
    }

    /// List the contents of the current working directory.
    pub fn list(&self) -> GenericResult<Vec<suppaftp::list::File>> {
        self.ftp_client()?.list()
    }

    /// List the contents of `path`.
    pub fn list_path_contents(&self, path: &FtpPath) -> GenericResult<Vec<suppaftp::list::File>> {
        // The implementation of the LIST command on the Aurora FTP
        // server does not support specifying a path. It will always
        // return the contents of the current working directory.
        //
        // So set the CWD to the parent of `path`, then list
        // the contents of the CWD
        let mut ftp_client = self.ftp_client()?;
        ftp_client.cwd(&path.as_string())?;
        ftp_client.list()
    }

    /// List the file/directory at `path`.
    pub fn list_path(&self, path: &FtpPath) -> GenericResult<Option<suppaftp::list::File>> {
        let entry_parent = path.parent().ok_or_else(|| {
            let msg = format!(
                "Failed to list entry at '{}'. Could not determine parent path.",
                path
            );
            error!("{}", msg);
            msg
        })?;
        let entry_name = path.name().ok_or_else(|| {
            let msg = format!(
                "Failed to list entry at '{}'. Could not determine entry name.",
                path
            );
            error!("{}", msg);
            msg
        })?;
        let entries = self.list_path_contents(&entry_parent)?;
        Ok(entries.into_iter().find(|x| x.name() == entry_name))
    }

    /// Create a directory at `path`.
    ///
    /// If any parent directories of `path` do not exist, they will be created.
    pub fn mkdir(&self, path: &FtpPath) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        let mut current_path = FtpPath::from_str("/");
        // starting from the root directory, create all parent directories of `path`
        // assume the directory immediately within `/` already exists (since we should
        // not be able to create directories directly within `/`)
        for entry in &path.parts {
            current_path.push(&entry);
            match self.list_path(&current_path)? {
                None => ftp_client.mkdir(&current_path.as_string())?,
                Some(x) => {
                    if x.is_file() {
                        let msg = format!(
                            "Failed to create directory at '{}'. The path '{}' already exists and is not a directory.",
                            path, current_path
                        );
                        error!("{}", msg);
                        return Err(msg.into());
                    }
                }
            };
        }
        Ok(())
    }

    /// Recursively delete the entry at `path` as well as any possible contents of `path` if it is a directory.
    pub fn rm(&self, path: &FtpPath) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        self.rm_recursive(&mut ftp_client, path)
    }

    /// Move the file or directory at `remote_src` to `remote_dest`.
    pub fn mv(&self, src: &FtpPath, dest: &FtpPath) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        match dest.parent() {
            None => (),
            Some(x) => self.mkdir(&x)?,
        };
        ftp_client.rename(&src.as_string(), &dest.as_string())
    }

    /// Recursively download the entry at `remote_path` to `local_path`
    /// as well as any possible contents of `remote_path` if it is a directory.
    ///
    /// If the `local_path` already exists, the remote files will be merged with existing local files.
    pub fn download(&mut self, remote_path: &FtpPath, local_path: &Path) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        self.download_recursive(&mut ftp_client, remote_path, local_path)
    }

    /// Recursively upload the entry at `local_path` to `remote_path`
    /// as well as any possible contents of `local_path` if it is a directory.
    ///
    /// If the `remote_path` already exists, the local files will be merged with existing remote files.
    pub fn upload(&mut self, local_path: &Path, remote_path: &FtpPath) -> GenericResult<()> {
        let mut ftp_client = self.ftp_client()?;
        self.upload_recursive(&mut ftp_client, local_path, remote_path)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // private methods
    ////////////////////////////////////////////////////////////////////////////////
    fn ftp_client(&self) -> GenericResult<AuroraFtpClient> {
        let mut ftp_client = AuroraFtpClient::new(&self.ip, self.port)?;
        match (&self.username, &self.password) {
            (Some(u), Some(p)) => ftp_client.login(&u, &p)?,
            _ => (),
        }
        ftp_client.transfer_type(self.transfer_type.clone())?;
        ftp_client.cwd(&self.cwd.as_string())?;
        Ok(ftp_client)
    }

    fn download_recursive(
        &self,
        ftp_client: &mut AuroraFtpClient,
        remote_path: &FtpPath,
        local_path: &Path,
    ) -> GenericResult<()> {
        let entry = match self.list_path(remote_path)? {
            None => {
                let msg =
                    format!(
                    "Failed to download from '{}' to '{}'. The remote path '{}' does not exist.",
                    remote_path, local_path.to_string_lossy(), remote_path
                );
                error!("{}", msg);
                return Err(msg.into());
            }
            Some(x) => x,
        };
        if entry.is_file() {
            // download file
            ftp_client.retr_to_file(&remote_path.as_string(), &local_path.to_string_lossy())?;
        } else if entry.is_directory() {
            // create local directory
            let entry_local_path = Path::new(local_path);
            std::fs::create_dir_all(&entry_local_path)?;
            // recursively call `download` on contents of `remote_path`
            for x in self.list_path_contents(remote_path)? {
                let mut new_remote_path = remote_path.clone();
                new_remote_path.push(x.name());
                let new_local_path = Path::new(local_path).join(x.name());
                self.download_recursive(ftp_client, &new_remote_path, &new_local_path)?;
            }
        } else {
            // error unsupported type
            let msg = format!(
                "Failed to download '{}'. Path is neither a file nor a directory.",
                remote_path
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(())
    }

    fn rm_recursive(&self, ftp_client: &mut AuroraFtpClient, path: &FtpPath) -> GenericResult<()> {
        let entry = match self.list_path(path)? {
            None => return Ok(()),
            Some(x) => x,
        };
        if entry.is_file() {
            // delete file at `path`
            ftp_client.rm(&path.as_string())?;
        } else if entry.is_directory() {
            // recursively call `rm()` on contents of `path` then delete directory at `path`
            let entry_contents = self.list_path_contents(path)?;
            for entry in entry_contents {
                let mut entry_ftp_path = path.clone();
                entry_ftp_path.push(entry.name());
                self.rm_recursive(ftp_client, &entry_ftp_path)?;
            }
            ftp_client.rmdir(&path.as_string())?;
        } else {
            // error unsupported type
            let msg = format!(
                "Failed to remove '{}'. Path is neither a file nor a directory.",
                path
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(())
    }

    fn upload_recursive(
        &self,
        ftp_client: &mut AuroraFtpClient,
        local_path: &Path,
        remote_path: &FtpPath,
    ) -> GenericResult<()> {
        if local_path.is_file() {
            match remote_path.parent() {
                None => (),
                Some(x) => self.mkdir(&x)?,
            };
            ftp_client.put_file(&local_path.to_string_lossy(), &remote_path.as_string())?;
        } else if local_path.is_dir() {
            // create remote directory then recursively call `upload()` for each entry in the local directory
            self.mkdir(remote_path)?;
            for entry in std::fs::read_dir(local_path)? {
                let entry = entry?;
                let new_local_path = entry.path();
                let mut new_remote_path = remote_path.clone();
                new_remote_path.push(&entry.file_name().to_string_lossy());
                self.upload_recursive(ftp_client, &new_local_path, &new_remote_path)?;
            }
        } else {
            // error unsupported type
            let msg = format!(
                "Failed to upload '{}'. Path is neither a file nor a directory.",
                local_path.to_string_lossy()
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(())
    }
}

// TODO use suppaftp::AsyncFtpStream?
// #[derive(Clone, Debug)]
/// Direct implementation of the Aurora FTP Client.
pub struct AuroraFtpClient {
    ftp_stream: suppaftp::FtpStream,
}

impl AuroraFtpClient {
    /// Create a new `AuroraHttpClient` instance.
    pub fn new(ip: &str, port: usize) -> GenericResult<Self> {
        let host = format!("{}:{}", ip, port);
        let socket_address = host.parse()?;
        let timeout = Duration::new(15, 0);
        let ftp_stream =
            suppaftp::FtpStream::connect_timeout(socket_address, timeout).map_err(|err| {
                let msg = format!(
                    "Failed to connect to FTP server at '{}'. Got the following error: {}",
                    &host, err,
                );
                error!("{}", msg);
                msg
            })?;
        Ok(Self { ftp_stream })
    }

    /// Authenticate with FTP server.
    pub fn login(&mut self, username: &str, password: &str) -> GenericResult<()> {
        self.ftp_stream.login(username, password).map_err(|err| {
            let msg = format!(
                "Failed to call login on FTP server. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Set the transfer type.
    pub fn transfer_type(&mut self, transfer_type: suppaftp::types::FileType) -> GenericResult<()> {
        self.ftp_stream
            .transfer_type(transfer_type.clone())
            .map_err(|err| {
                let msg = format!(
                    "Failed to set FTP transfer type to '{}'. Got the following error: {}",
                    &transfer_type, err
                );
                error!("{}", msg);
                msg.into()
            })
    }

    /// Set the current working directory.
    pub fn cwd(&mut self, path: &str) -> GenericResult<()> {
        self.ftp_stream.cwd(path).map_err(|err| {
            let msg = format!(
                "Failed to call cwd with path '{}' on FTP server. Got the following error: {}",
                path, err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Print the current working directory.
    pub fn pwd(&mut self) -> GenericResult<String> {
        self.ftp_stream.pwd().map_err(|err| {
            let msg = format!(
                "Failed to call pwd on FTP server. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// List contents of the current working directory.
    ///
    /// The implementation of the LIST command on the Aurora FTP server does not support specifying a path.
    /// It will always return the contents of the current working directory.
    pub fn list(&mut self) -> GenericResult<Vec<suppaftp::list::File>> {
        let mut entries: Vec<suppaftp::list::File> = Vec::new();
        let raw_entries = self.ftp_stream.list(None).map_err(|err| {
            let msg = format!(
                "Failed to call list on FTP server. Got the following error: {}",
                err
            );
            error!("{}", msg);
            msg
        })?;
        for raw_entry in raw_entries {
            entries.push(suppaftp::list::ListParser::parse_posix(&raw_entry)?);
        }
        Ok(entries)
    }

    /// Create a directory at `path`
    ///
    /// Note: If directory already exists or if not all parent directories exist, then an error will be thrown.
    pub fn mkdir(&mut self, path: &str) -> GenericResult<()> {
        self.ftp_stream.mkdir(path).map_err(|err| {
            let msg = format!(
                "Failed to call mkdir for path '{}' on FTP server. Got the following error: {}",
                path, err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Download file at `remote_path`.
    pub fn retr(&mut self, remote_path: &str) -> GenericResult<Vec<u8>> {
        let mut file_bytes: Vec<u8> = Vec::new();
        self.ftp_stream
            .retr(remote_path, |stream| {
                let mut buffer: Vec<u8> = vec![0; 4096];
                loop {
                    let bytes_read = stream.read(&mut buffer).map_err(|err| {
                        let msg = format!("Failed to call read on FTP stream. Got the following error: {}", err);
                        error!("{}", msg);
                        suppaftp::FtpError::ConnectionError(err)
                    })?;
                    if bytes_read == 0 {
                        break;
                    }
                    file_bytes.append(&mut buffer[..bytes_read].to_vec());
                }
                Ok(())
            })
            .map_err(|err| {
                let msg = format!(
                    "Failed to call retr for remote path '{}' on FTP server. Got the following error: {}",
                    remote_path, err
                );
                error!("{}", msg);
                msg
            })?;
        Ok(file_bytes)
    }

    /// Download file at `remote_path` and save to file at `local_path`.
    pub fn retr_to_file(&mut self, remote_path: &str, local_path: &str) -> GenericResult<()> {
        let local_path = std::path::Path::new(local_path);
        create_file(local_path)?;
        let mut file = std::fs::OpenOptions::new().append(true).open(local_path)?;
        self.ftp_stream
            .retr(remote_path, |stream| {
                let mut buffer: Vec<u8> = vec![0; 4096];
                loop {
                    let bytes_read = stream.read(&mut buffer).map_err(|err| {
                        let msg = format!("Failed to call read on FTP stream. Got the following error: {}", err);
                        error!("{}", msg);
                        suppaftp::FtpError::ConnectionError(err)
                    })?;
                    if bytes_read == 0 {
                        break;
                    }
                    file.write_all(&buffer[..bytes_read]).map_err(|err| {
                        let msg = format!("Failed to write bytes to file at '{}'. Got the following error: {}", local_path.display(), err);
                        error!("{}", msg);
                        suppaftp::FtpError::ConnectionError(err)
                    })?;
                }
                Ok(())
            })
            .map_err(|err| {
                let msg = format!(
                    "Failed to call retr for remote path '{}' on FTP server. Got the following error: {}",
                    remote_path, err
                );
                error!("{}", msg);
                msg.into()
            })
    }

    /// Delete file at `path`.
    ///
    /// If the file does not exist, an error will be thrown.
    pub fn rm(&mut self, path: &str) -> GenericResult<()> {
        self.ftp_stream.rm(path).map_err(|err| {
            let msg = format!(
                "Failed to call rm for path '{}' on FTP server. Got the following error: {}",
                path, err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Delete directory at `path`.
    ///
    /// If the directory does not exist or if the directory is not empty, then an error will be thrown.
    pub fn rmdir(&mut self, path: &str) -> GenericResult<()> {
        self.ftp_stream.rmdir(path).map_err(|err| {
            let msg = format!(
                "Failed to call rmdir for path '{}' on FTP server. Got the following error: {}",
                path, err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Rename file or directory
    ///
    /// If the parent directory of `dest_path` does not exist, then an error will be thrown.
    pub fn rename(&mut self, src_path: &str, dest_path: &str) -> GenericResult<()> {
        self.ftp_stream.rename(src_path, dest_path).map_err(|err| {
            let msg = format!(
                "Failed to call rename with source path '{}' and destination path '{}' on FTP server. Got the following error: {}",
                src_path, dest_path, err
            );
            error!("{}", msg);
            msg.into()
        })
    }

    /// Upload file
    ///
    /// If the parent directory of `remote_path` does not exist, then an error will be thrown.
    pub fn put_file(&mut self, local_path: &str, remote_path: &str) -> GenericResult<()> {
        let file_len_bytes = std::fs::metadata(local_path)?.len();
        let mut file = std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(local_path)?;
        let bytes_written = self.ftp_stream.put_file(remote_path, &mut file)?;
        if bytes_written != file_len_bytes {
            let err = format!(
                "Expected to write {} bytes to FTP server but actually wrote {} bytes.",
                file_len_bytes, bytes_written
            );
            let msg = format!(
                "Failed to put file with local path '{}' and remote path '{}' on FTP server. Got the following error: {}",
                local_path, remote_path, err
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(())
    }
}

// TODO change `parts` to something else?
/// Representation of an path on the console.
#[derive(Clone, Debug)]
pub struct FtpPath {
    pub parts: Vec<String>,
}

impl fmt::Display for FtpPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl FtpPath {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn from_str(path: &str) -> Self {
        let parts: Vec<String> = path
            .split("/")
            .filter(|x| !x.is_empty())
            .map(|x| String::from(x))
            .collect();
        Self { parts }
    }

    pub fn from_vec(path_parts: &Vec<String>) -> Self {
        Self {
            parts: path_parts.clone(),
        }
    }

    pub fn as_string(&self) -> String {
        format!("/{}", self.parts.join("/"))
    }

    pub fn name(&self) -> Option<String> {
        match self.parts.len() {
            0 => None,
            _ => Some(self.parts[self.parts.len() - 1].clone()),
        }
    }

    pub fn parent(&self) -> Option<FtpPath> {
        match self.parts.len() {
            0 => None,
            _ => {
                let mut new_parts = self.parts.clone();
                new_parts.pop();
                Some(Self { parts: new_parts })
            }
        }
    }

    pub fn pop(&mut self) -> () {
        self.parts.pop();
    }

    pub fn push(&mut self, entry_name: &str) -> () {
        self.parts.push(String::from(entry_name));
    }
}
