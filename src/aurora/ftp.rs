/**
 * Copyright 2024 jrobiche
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
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use log::{debug, error, warn};

use crate::utils::{create_parent_dirs, GenericResult};

#[derive(Clone, Debug)]
pub struct FtpClient {
    ip: String,
    port: usize,
    username: Option<String>,
    password: Option<String>,
}

impl FtpClient {
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

    pub fn new_ftp_stream(&self) -> GenericResult<suppaftp::FtpStream> {
        let address = format!("{}:{}", self.ip, self.port);
        debug!("Create FTP Stream: calling connect('{}')", address);
        match suppaftp::FtpStream::connect(address) {
            Ok(mut ftp_stream) => {
                match (&self.username, &self.password) {
                    (Some(username), Some(password)) => ftp_stream.login(username, password)?,
                    _ => (),
                };
                ftp_stream.transfer_type(suppaftp::types::FileType::Binary)?;
                Ok(ftp_stream)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn create_directory(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_dir: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = create_directory(&mut stream, remote_dir);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn delete_directory(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_dir: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = delete_directory(&mut stream, remote_dir);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn delete_file(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_file: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = delete_file(&mut stream, remote_file);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn download_directory<P: AsRef<OsStr> + AsRef<Path>>(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_dir: &str,
        local_dir: P,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = download_directory(&mut stream, remote_dir, local_dir);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn download_file<P: AsRef<OsStr> + AsRef<Path>>(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_file: &str,
        local_file: P,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = download_file(&mut stream, remote_file, local_file);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn list_directory_contents(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_dir: &str,
    ) -> GenericResult<Vec<suppaftp::list::File>> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = list_directory_contents(&mut stream, remote_dir);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn list_entry(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_path: &str,
    ) -> GenericResult<Option<suppaftp::list::File>> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = list_entry(&mut stream, remote_path);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn rename(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        remote_path: &str,
        new_name: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = rename(&mut stream, remote_path, new_name);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn upload_directory<P: AsRef<OsStr> + AsRef<Path>>(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        local_dir: P,
        remote_dir: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = upload_directory(&mut stream, local_dir, remote_dir);
        if quit_stream {
            stream.quit()?;
        }
        result
    }

    pub fn upload_file<P: AsRef<OsStr> + AsRef<Path>>(
        &self,
        ftp_stream: Option<suppaftp::FtpStream>,
        local_file: P,
        remote_file: &str,
    ) -> GenericResult<()> {
        let quit_stream = ftp_stream.is_none();
        let mut stream = match ftp_stream {
            Some(x) => x,
            None => self.new_ftp_stream()?,
        };
        let result = upload_file(&mut stream, local_file, remote_file);
        if quit_stream {
            stream.quit()?;
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct FTPPath {
    pub parts: Vec<String>,
}

impl FTPPath {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn from_str(path: &str) -> Self {
        let mut parts: Vec<String> = Vec::new();
        for x in path.split("/").into_iter() {
            if x != "" {
                parts.push(String::from(x));
            }
        }
        Self { parts }
    }

    pub fn from_vec(path_parts: &Vec<String>) -> Self {
        Self {
            parts: path_parts.clone(),
        }
    }

    pub fn as_string(&self) -> String {
        return format!("/{}", self.parts.join("/"));
    }

    pub fn file_name(&self) -> Option<String> {
        match self.parts.len() {
            0 => None,
            _ => Some(self.parts[self.parts.len() - 1].clone()),
        }
    }

    pub fn parent_string(&self) -> Option<String> {
        match self.parts.len() {
            0 => None,
            1 => Some(String::from("/")),
            _ => Some(format!(
                "/{}",
                self.parts[..=(self.parts.len() - 2)].join("/")
            )),
        }
    }

    pub fn parts(&self) -> Vec<String> {
        self.parts.clone()
    }

    pub fn pop(&mut self) -> () {
        self.parts.pop();
    }

    pub fn push(&mut self, path: &str) -> () {
        self.parts.push(String::from(path));
    }
}

fn create_directory(ftp_stream: &mut suppaftp::FtpStream, remote_dir: &str) -> GenericResult<()> {
    debug!("Create Directory: '{}'", remote_dir);
    let remote_path = FTPPath::from_str(remote_dir);
    let mut new_remote_path = FTPPath::new();
    for directory in remote_path.parts() {
        new_remote_path.push(&directory);
        match list_entry(ftp_stream, &new_remote_path.as_string())? {
            Some(x) => {
                // entry already exists, error if it is not a directory
                if !x.is_directory() {
                    warn!(
                        "Create Directory: Cannot create directory '{}' because the path '{}' exists and is not a directory.",
                        remote_path.as_string(), new_remote_path.as_string()
                    );
                    return Err(format!(
                        "Cannot create directory '{}' because the path '{}' exists and is not a directory.",
                        remote_path.as_string(), new_remote_path.as_string()
                    ).into());
                }
            }
            None => {
                debug!(
                    "Create Directory: Calling mkdir('{}')",
                    new_remote_path.as_string()
                );
                ftp_stream.mkdir(new_remote_path.as_string())?;
            }
        }
    }
    Ok(())
}

fn delete_directory(ftp_stream: &mut suppaftp::FtpStream, remote_dir: &str) -> GenericResult<()> {
    debug!("Delete Directory: '{}'", remote_dir);
    let mut remote_path = FTPPath::from_str(remote_dir);
    for entry in list_directory_contents(ftp_stream, &remote_path.as_string())? {
        remote_path.push(entry.name());
        let new_remote_path = remote_path.as_string();
        remote_path.pop();
        if entry.is_file() {
            delete_file(ftp_stream, &new_remote_path)?;
        }
        if entry.is_directory() {
            delete_directory(ftp_stream, &new_remote_path)?;
        }
    }
    debug!(
        "Delete Directory: Calling rmdir('{}')",
        remote_path.as_string()
    );
    match ftp_stream.rmdir(remote_path.as_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn delete_file(ftp_stream: &mut suppaftp::FtpStream, remote_file: &str) -> GenericResult<()> {
    debug!("Delete File: '{}'", remote_file);
    debug!(
        "Delete File: Calling rm('{}')",
        FTPPath::from_str(remote_file).as_string()
    );
    match ftp_stream.rm(FTPPath::from_str(remote_file).as_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn download_directory<P: AsRef<OsStr> + AsRef<Path>>(
    ftp_stream: &mut suppaftp::FtpStream,
    remote_dir: &str,
    local_path: P,
) -> GenericResult<()> {
    // mirror contents of `remote_path` into `local_path`
    // if a directory already exists at `local_path`, it will be
    // deleted before remote files are downloaded into it.
    debug!("Download Directory: '{}'", remote_dir);
    std::fs::create_dir_all(&local_path)?;
    if std::path::Path::new(&local_path).is_file() {
        return Err("Destination already exists and is a file.".into());
    }
    if std::path::Path::new(&local_path).is_dir() {
        std::fs::remove_dir_all(&local_path)?;
    }
    let mut remote_path = FTPPath::from_str(remote_dir);
    for entry in list_directory_contents(ftp_stream, &remote_path.as_string())? {
        let new_local_path = Path::new(&local_path).join(entry.name());
        remote_path.push(entry.name());
        let new_remote_path = remote_path.as_string();
        remote_path.pop();
        if entry.is_directory() {
            download_directory(ftp_stream, &new_remote_path, &new_local_path)?;
        }
        if entry.is_file() {
            download_file(ftp_stream, &new_remote_path, &new_local_path)?;
        }
    }
    Ok(())
}

fn download_file<P: AsRef<OsStr> + AsRef<Path>>(
    ftp_stream: &mut suppaftp::FtpStream,
    remote_path: &str,
    local_path: P,
) -> GenericResult<()> {
    debug!("Download File: '{}'", remote_path);
    // prepare local directory that file will be downloaded into
    create_parent_dirs(&local_path)?;
    if std::path::Path::new(&local_path).is_file() {
        std::fs::remove_file(&local_path)?;
    }
    if std::path::Path::new(&local_path).is_dir() {
        return Err("Destination already exists and is a directory.".into());
    }
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(&local_path)?;
    debug!(
        "Download File: calling retr('{}')",
        FTPPath::from_str(remote_path).as_string()
    );
    ftp_stream.retr(&FTPPath::from_str(remote_path).as_string(), |stream| {
        let mut buf: Vec<u8> = vec![0; 4096];
        loop {
            let bytes_read = match stream.read(&mut buf) {
                Ok(x) => x,
                Err(e) => return Err(suppaftp::FtpError::ConnectionError(e)),
            };
            debug!("Download File: Read {} bytes", bytes_read);
            if bytes_read == 0 {
                break;
            }
            match file.write_all(&buf[0..bytes_read]) {
                Ok(_) => (),
                Err(e) => return Err(suppaftp::FtpError::ConnectionError(e)),
            };
        }
        Ok(())
    })?;
    Ok(())
}

fn list_directory_contents(
    ftp_stream: &mut suppaftp::FtpStream,
    remote_path: &str,
) -> GenericResult<Vec<suppaftp::list::File>> {
    debug!("List Directory Contents: '{}'", remote_path);
    // `list(<path>)` returns the contents of the CWD instead of
    // the contents of `<path>`. so set the CWD to the parent of
    // `<path>`, then list the contents of the CWD, then restore
    // the original CWD
    let original_cwd = ftp_stream.pwd()?;
    debug!(
        "List Directory Contents: calling cwd('{}')",
        FTPPath::from_str(remote_path).as_string()
    );
    ftp_stream.cwd(FTPPath::from_str(remote_path).as_string())?;
    let mut contents: Vec<suppaftp::list::File> = Vec::new();
    debug!("List Directory Contents: calling list(None)");
    let entries = match ftp_stream.list(None) {
        Ok(x) => x,
        Err(e) => {
            error!(
                "List Directory Contents: Failed to list cwd. Got the error: '{}'.",
                e
            );
            // restore orginal cwd
            debug!("List Directory Contents: calling cwd('{}')", original_cwd);
            ftp_stream.cwd(original_cwd)?;
            return Err(e.into());
        }
    };
    for entry in entries {
        contents.push(suppaftp::list::File::from_posix_line(&entry)?);
    }
    // restore original cwd
    debug!("List Directory Contents: calling cwd('{}')", original_cwd);
    ftp_stream.cwd(original_cwd)?;
    Ok(contents)
}

fn list_entry(
    ftp_stream: &mut suppaftp::FtpStream,
    remote_path: &str,
) -> GenericResult<Option<suppaftp::list::File>> {
    debug!("List Entry: '{}'", remote_path);
    let ftp_path = FTPPath::from_str(remote_path);
    let parent_path = match ftp_path.parent_string() {
        Some(x) => x,
        None => {
            error!(
                "List Entry: Cannot list entry at path '{}' because parent is 'None'.",
                ftp_path.as_string()
            );
            return Err(format!(
                "Cannot list entry at path '{}' because parent is 'None'.",
                ftp_path.as_string()
            )
            .into());
        }
    };
    let file_name = match ftp_path.file_name() {
        Some(x) => x,
        None => {
            error!(
                "List Entry: Cannot list entry at path '{}' because file name is 'None'.",
                ftp_path.as_string()
            );
            return Err(format!(
                "Cannot list entry at path '{}' because file name is 'None'.",
                ftp_path.as_string()
            )
            .into());
        }
    };
    for entry in list_directory_contents(ftp_stream, &parent_path)? {
        if entry.name() == file_name {
            debug!(
                "List Entry: Found entry at '{}/{}'",
                &parent_path,
                entry.name()
            );
            return Ok(Some(entry));
        }
    }
    debug!("List Entry: Did not find entry for path '{}'", remote_path);
    Ok(None)
}

fn rename(
    ftp_stream: &mut suppaftp::FtpStream,
    remote_path: &str,
    new_name: &str,
) -> GenericResult<()> {
    debug!("Rename: '{}' to '{}'", remote_path, new_name);
    let src_path = FTPPath::from_str(remote_path);
    let mut dest_path = FTPPath::from_str(remote_path);
    dest_path.pop();
    dest_path.push(new_name);
    debug!(
        "Rename: calling rename('{}', '{}')",
        src_path.as_string(),
        dest_path.as_string()
    );
    match ftp_stream.rename(src_path.as_string(), dest_path.as_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn upload_directory<P: AsRef<OsStr> + AsRef<Path>>(
    ftp_stream: &mut suppaftp::FtpStream,
    local_dir: P,
    remote_dir: &str,
) -> GenericResult<()> {
    debug!("Upload Directory: '{}'", remote_dir);
    create_directory(ftp_stream, remote_dir)?;
    let mut new_remote_path = FTPPath::from_str(remote_dir);
    for entry in std::fs::read_dir(std::path::Path::new(&local_dir))? {
        let path = entry?.path();
        let file_name = match path.file_name() {
            None => {
                error!(
                    "Upload Directory: Failed to get file name for path '{}'",
                    path.display()
                );
                return Err(
                    format!("Failed to get file name for path '{}'.", path.display()).into(),
                );
            }
            Some(x) => match x.to_str() {
                Some(y) => y,
                None => {
                    error!(
                        "Upload Directory: Failed to convert file name of path '{}' to str",
                        path.display()
                    );
                    return Err(format!(
                        "Failed to convert file name of path '{}' to str.",
                        path.display()
                    )
                    .into());
                }
            },
        };
        new_remote_path.push(file_name);
        if path.is_dir() {
            upload_directory(ftp_stream, path, &new_remote_path.as_string())?;
        } else {
            upload_file(ftp_stream, path, &new_remote_path.as_string())?;
        }
        new_remote_path.pop();
    }
    Ok(())
}

fn upload_file<P: AsRef<OsStr> + AsRef<Path>>(
    ftp_stream: &mut suppaftp::FtpStream,
    local_path: P,
    remote_file: &str,
) -> GenericResult<()> {
    debug!("Upload File: '{}'", remote_file);
    let remote_path = FTPPath::from_str(remote_file);
    let file_len_bytes = std::fs::metadata(&local_path)?.len();
    let mut file = std::fs::OpenOptions::new()
        .create(false)
        .read(true)
        .open(&local_path)?;
    match remote_path.parent_string() {
        Some(x) => create_directory(ftp_stream, &x)?,
        None => (),
    }
    debug!(
        "Upload File: calling put_file('{}')",
        remote_path.as_string()
    );
    let bytes_written = ftp_stream.put_file(remote_path.as_string(), &mut file)?;
    if bytes_written != file_len_bytes {
        error!(
            "Upload File: Expected to write {} bytes to remote but wrote {}.",
            file_len_bytes, bytes_written
        );
        return Err(format!(
            "Expected to write {} bytes to remote but wrote {}.",
            file_len_bytes, bytes_written
        )
        .into());
    }
    Ok(())
}
