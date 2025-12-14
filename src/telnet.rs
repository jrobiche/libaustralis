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
// TODO validate input
use crate::utils::GenericResult;
use log::error;
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use telnet::{Event, Telnet};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TelnetResponse {
    pub status: TelnetResponseStatus,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TelnetResponseStatus {
    pub code: usize,
    pub text: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DrivelistEntry {
    pub drivename: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirlistEntry {
    pub name: String,
    pub sizehi: usize,
    pub sizelo: usize,
    pub createhi: usize,
    pub createlo: usize,
    pub changehi: usize,
    pub changelo: usize,
    pub directory: bool,
    pub hidden: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Drivefreespace {
    pub freetocallerlo: usize,
    pub freetocallerhi: usize,
    pub totalbyteslo: usize,
    pub totalbyteshi: usize,
    pub totalfreebyteslo: usize,
    pub totalfreebyteshi: usize,
}

pub struct TelnetClient {
    pub address: SocketAddr,
}

impl TelnetClient {
    pub fn new(ip: &str, port: u16) -> GenericResult<Self> {
        let address = match Ipv4Addr::from_str(ip) {
            Ok(ipv4) => SocketAddr::new(IpAddr::V4(ipv4), port),
            Err(err) => {
                let msg = format!("Invalid IPv4 address '{}'.", ip);
                error!("{} Got the following error: {}", msg, err);
                return Err(msg.into());
            }
        };
        Ok(Self { address })
    }

    pub fn connect(&self) -> GenericResult<Telnet> {
        // TODO parameterize timeout?
        let timeout = core::time::Duration::new(15, 0);
        // TODO parameterize buffer size?
        let buffer_size = 4096;
        let mut telnet = Telnet::connect_timeout(&self.address, buffer_size, timeout)?;
        let response_status = self.get_response_status(&mut telnet)?;
        let expected_response_status = TelnetResponseStatus {
            code: 201,
            text: String::from("connected"),
        };
        if response_status != expected_response_status {
            let msg = format!(
                "Unexpected connection response status. Expected {:?} but got {:?}",
                expected_response_status, response_status
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(telnet)
    }

    pub fn disconnect(&self, telnet: &mut Telnet) -> GenericResult<()> {
        // does not call `execute` because telnet connection is closed after response status
        telnet.write("bye\r\n".as_bytes())?;
        let response_status = self.get_response_status(telnet)?;
        let expected_response_status = TelnetResponseStatus {
            code: 200,
            text: String::from("bye"),
        };
        if response_status != expected_response_status {
            let msg = format!(
                "Unexpected disconnect response status. Expected {:?} but got {:?}",
                expected_response_status, response_status
            );
            error!("{}", msg);
            return Err(msg.into());
        }
        Ok(())
    }

    // generic function to execute any command
    // caller is responsible for parsing `TelnetResponse.data`
    pub fn execute(&self, telnet: &mut Telnet, cmd: &str) -> GenericResult<TelnetResponse> {
        telnet.write(format!("{}\r\n", cmd).as_bytes())?;
        let status = self.get_response_status(telnet)?;
        let data = match status.code {
            202 => self.get_response_data_202(telnet)?,
            _ => self.flush(telnet)?,
        };
        Ok(TelnetResponse { status, data })
    }

    pub fn execute_and_disconnect(&self, cmd: &str) -> GenericResult<TelnetResponse> {
        let mut telnet = self.connect()?;
        let response = self.execute(&mut telnet, cmd)?;
        self.disconnect(&mut telnet)?;
        Ok(response)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // common commands (that do not require an existing connection)
    ////////////////////////////////////////////////////////////////////////////////
    pub fn dirlist(&self, path: Vec<&str>) -> GenericResult<Vec<DirlistEntry>> {
        if path.len() == 0 {
            let msg = "Path list must contain at least 1 value.";
            return Err(msg.into());
        }
        let name = format!("{}:\\{}", path[0], &path[1..].join("\\"));
        let cmd = format!("dirlist name=\"{}\"", name);
        let resp = self.execute_and_disconnect(&cmd)?;
        parse_dirlist_data(resp.data)
    }

    pub fn drivefreespace(&self, name: &str) -> GenericResult<Drivefreespace> {
        let cmd = format!("drivefreespace name=\"{}\"", name);
        let resp = self.execute_and_disconnect(&cmd)?;
        parse_drivefreespace_data(resp.data)
    }

    pub fn drivelist(&self) -> GenericResult<Vec<DrivelistEntry>> {
        let resp = self.execute_and_disconnect("drivelist")?;
        parse_drivelist_data(resp.data)
    }

    pub fn dvdeject(&self) -> GenericResult<()> {
        let response = self.execute_and_disconnect("dvdeject")?;
        if response.status.code != 200 {
            let msg = format!(
                "Expected a status code of 200 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn magicboot(&self) -> GenericResult<()> {
        let response = self.execute_and_disconnect("magicboot")?;
        if response.status.code != 200 {
            let msg = format!(
                "Expected a status code of 200 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn magicboot_cold(&self) -> GenericResult<()> {
        let response = self.execute_and_disconnect("magicboot COLD")?;
        if response.status.code != 200 {
            let msg = format!(
                "Expected a status code of 200 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn magicboot_path(&self, path: Vec<&str>) -> GenericResult<()> {
        if path.len() < 2 {
            let msg = "Path list must contain at least 2 values.";
            return Err(msg.into());
        }
        let title = format!("{}:\\{}", path[0], &path[1..].join("\\"));
        let directory = format!("{}:\\{}", path[0], &path[1..path.len() - 1].join("\\"));
        let cmd = format!("magicboot title=\"{}\" directory=\"{}\"", title, directory);
        let response = self.execute_and_disconnect(&cmd)?;
        if response.status.code != 200 {
            let msg = format!(
                "Expected a status code of 200 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn getmem(&self, address: usize, length: usize) -> GenericResult<Vec<u8>> {
        let cmd = format!("getmem addr=\"0x{:X}\" length=\"0x{:X}\"", address, length);
        let resp = self.execute_and_disconnect(&cmd)?;
        parse_getmem_data(resp.data)
    }

    pub fn go(&self) -> GenericResult<()> {
        let response = self.execute_and_disconnect("go")?;
        if response.status.code != 200 && response.status.code != 408 {
            let msg = format!(
                "Expected a status code of 200 or 408 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn setmem(&self, address: usize, data: String) -> GenericResult<()> {
        let cmd = format!("setmem addr=\"0x{:X}\" data={}", address, data);
        let response = self.execute_and_disconnect(&cmd)?;
        if response.status.code != 200 {
            let msg = format!(
                "Expected a status code of 200 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn stop(&self) -> GenericResult<()> {
        let response = self.execute_and_disconnect("stop")?;
        if response.status.code != 200 && response.status.code != 426 {
            let msg = format!(
                "Expected a status code of 200 or 426 but got {}. Full response follows {:?}",
                response.status.code, response
            );
            return Err(msg.into());
        }
        Ok(())
    }

    pub fn shutdown(&self) -> GenericResult<()> {
        // does not use `execute_and_disconnect()` because console cannot return a response
        let mut telnet = self.connect()?;
        telnet.write("shutdown\r\n".as_bytes())?;
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    // private methods
    ////////////////////////////////////////////////////////////////////////////////
    fn flush(&self, telnet: &mut Telnet) -> GenericResult<Vec<u8>> {
        // TODO freezes if telnet is disconnected (on `bye` command)
        // TODO parameterize timeout
        let timeout_duration = std::time::Duration::new(1, 0);
        let mut output: Vec<u8> = Vec::new();
        loop {
            let event = telnet.read_timeout(timeout_duration)?;
            if event == Event::TimedOut {
                break;
            }
            if let Event::Data(buffer) = event {
                for x in buffer {
                    output.push(x);
                }
            }
        }
        return Ok(output);
    }

    fn get_response_status(&self, telnet: &mut Telnet) -> GenericResult<TelnetResponseStatus> {
        loop {
            let event = telnet.read_nonblocking()?;
            if let Event::Data(buffer) = event {
                match std::str::from_utf8(&buffer)?.trim().split_once("- ") {
                    Some((code_str, msg)) => {
                        let code = code_str.parse::<usize>()?;
                        let msg_string = String::from(msg);
                        return Ok(TelnetResponseStatus {
                            code: code,
                            text: msg_string,
                        });
                    }
                    _ => return Err("Invalid response string.".into()),
                }
            }
        }
    }

    fn get_response_data_202(&self, telnet: &mut Telnet) -> GenericResult<Vec<u8>> {
        let mut output: Vec<u8> = Vec::new();
        loop {
            let event = telnet.read_nonblocking()?;
            if let Event::Data(buffer) = event {
                for x in &buffer {
                    output.push(*x);
                }
                if output.len() >= 3 {
                    // exit loop if output ends in b'.\r\n'
                    if &output[(output.len() - 3)..] == [46, 13, 10] {
                        break;
                    }
                }
            }
        }
        Ok(output)
    }
}

fn parse_dirlist_data(data: Vec<u8>) -> GenericResult<Vec<DirlistEntry>> {
    let mut entries = Vec::new();
    let hay = str::from_utf8(&data).map_err(|err| format!("Invalid UTF-8 sequence: {}", err))?;
    let re = Regex::new(
        r#"name="(.+)" sizehi=0[xX]([0-9a-fA-F]+) sizelo=0[xX]([0-9a-fA-F]+) createhi=0[xX]([0-9a-fA-F]+) createlo=0[xX]([0-9a-fA-F]+) changehi=0[xX]([0-9a-fA-F]+) changelo=0[xX]([0-9a-fA-F]+)(.*)"#,
    )?;
    for (_, [name, sizehi, sizelo, createhi, createlo, changehi, changelo, extra]) in
        re.captures_iter(hay).map(|c| c.extract())
    {
        entries.push(DirlistEntry {
            name: String::from(name),
            sizehi: usize::from_str_radix(sizehi, 16)?,
            sizelo: usize::from_str_radix(sizelo, 16)?,
            createhi: usize::from_str_radix(createhi, 16)?,
            createlo: usize::from_str_radix(createlo, 16)?,
            changehi: usize::from_str_radix(changehi, 16)?,
            changelo: usize::from_str_radix(changelo, 16)?,
            directory: extra.contains("directory"),
            hidden: extra.contains("hidden"),
        });
    }
    Ok(entries)
}

fn parse_drivefreespace_data(data: Vec<u8>) -> GenericResult<Drivefreespace> {
    let hay = str::from_utf8(&data).map_err(|err| format!("Invalid UTF-8 sequence: {}", err))?;
    let re = Regex::new(
        r#"freetocallerlo=0[xX]([0-9a-fA-F]+) freetocallerhi=0[xX]([0-9a-fA-F]+) totalbyteslo=0[xX]([0-9a-fA-F]+) totalbyteshi=0[xX]([0-9a-fA-F]+) totalfreebyteslo=0[xX]([0-9a-fA-F]+) totalfreebyteshi=0[xX]([0-9a-fA-F]+)"#,
    )?;
    let mut drivefreespace: Option<Drivefreespace> = None;
    for (
        _,
        [freetocallerlo, freetocallerhi, totalbyteslo, totalbyteshi, totalfreebyteslo, totalfreebyteshi],
    ) in re.captures_iter(hay).map(|c| c.extract())
    {
        drivefreespace = Some(Drivefreespace {
            freetocallerlo: usize::from_str_radix(freetocallerlo, 16)?,
            freetocallerhi: usize::from_str_radix(freetocallerhi, 16)?,
            totalbyteslo: usize::from_str_radix(totalbyteslo, 16)?,
            totalbyteshi: usize::from_str_radix(totalbyteshi, 16)?,
            totalfreebyteslo: usize::from_str_radix(totalfreebyteslo, 16)?,
            totalfreebyteshi: usize::from_str_radix(totalfreebyteshi, 16)?,
        })
    }
    match drivefreespace {
        Some(x) => Ok(x),
        None => Err("Failed to parse Drivefreespace properties from response.".into()),
    }
}

fn parse_drivelist_data(data: Vec<u8>) -> GenericResult<Vec<DrivelistEntry>> {
    let mut entries = Vec::new();
    let hay = str::from_utf8(&data).map_err(|err| format!("Invalid UTF-8 sequence: {}", err))?;
    let re = Regex::new(r#"drivename="(.+)""#)?;
    for (_, [drivename]) in re.captures_iter(hay).map(|c| c.extract()) {
        entries.push(DrivelistEntry {
            drivename: String::from(drivename),
        });
    }
    Ok(entries)
}

fn parse_getmem_data(data: Vec<u8>) -> GenericResult<Vec<u8>> {
    // validate input data
    let expected_trailing_bytes = [13, 10, 46, 13, 10];
    if data.len() <= (expected_trailing_bytes.len() + 1) {
        let msg = format!(
            "Getmem data must be at least {} bytes.",
            expected_trailing_bytes.len() + 1
        );
        error!("{}", msg);
        return Err(msg.into());
    }
    if (data.len() - 1) % 2 != 0 {
        let msg = "Getmem data must have an odd length.";
        error!("{}", msg);
        return Err(msg.into());
    }
    if data[data.len() - expected_trailing_bytes.len()..] != expected_trailing_bytes {
        let msg = format!(
            "Getmem data must end with the following values: {:?}",
            expected_trailing_bytes
        );
        error!("{}", msg);
        return Err(msg.into());
    }
    // convert string data to actual byte values
    // example:
    //   if `data` represents the string `0102\r\nABCD\r\n.\r\n`,
    //   then `01` -> `0x01`, `02` -> `0x02`, `\r\n` -> skipped, `AB` -> `0xAB`, `CD` -> `0xCD`, `\r\n.\r\n` -> skipped
    //   resulting in `[0x01, 0x02, 0xAB, 0xCD]` (i.e. `[1, 2, 171, 205]`)
    let mut bytes = Vec::new();
    for i in (0..data.len() - 5).step_by(2) {
        let byte_str = str::from_utf8(&data[i..i + 2])
            .map_err(|err| format!("Invalid UTF-8 sequence: {}", err))?;
        if byte_str == "\r\n" {
            continue;
        }
        bytes.push(u8::from_str_radix(&byte_str, 16)?);
    }
    Ok(bytes)
}
