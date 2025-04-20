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
// TODO move texture related items to new file?
use std::path::Path;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TextureEndian {
    EndianNone = 0,
    Endian8in16 = 1,
    Endian8in32 = 2,
    Endian16in32 = 3,
}

impl TextureEndian {
    pub fn display(&self) -> &str {
        match self {
            TextureEndian::EndianNone => "TextureEndian::EndianNone",
            TextureEndian::Endian8in16 => "TextureEndian::Endian8in16",
            TextureEndian::Endian8in32 => "TextureEndian::Endian8in32",
            TextureEndian::Endian16in32 => "TextureEndian::Endian16in32",
        }
    }

    pub fn from_u32(value: u32) -> GenericResult<TextureEndian> {
        let endian = match value {
            0 => Self::EndianNone,
            1 => Self::Endian8in16,
            2 => Self::Endian8in32,
            3 => Self::Endian16in32,
            _ => {
                let msg = format!("Could not convert value '{}' to TextureEndian.", value);
                return Err(msg.into());
            }
        };
        Ok(endian)
    }

    pub fn from_usize(value: usize) -> GenericResult<TextureEndian> {
        let endian = match value {
            0 => Self::EndianNone,
            1 => Self::Endian8in16,
            2 => Self::Endian8in32,
            3 => Self::Endian16in32,
            _ => {
                let msg = format!("Could not convert value '{}' to TextureEndian.", value);
                return Err(msg.into());
            }
        };
        Ok(endian)
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn to_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TextureFormat {
    RGBA8 = 6,
    BC3 = 20,
}

impl TextureFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            TextureFormat::RGBA8 => 4,
            TextureFormat::BC3 => 1,
        }
    }

    pub fn bytes_per_pixel_u32(&self) -> u32 {
        match self {
            TextureFormat::RGBA8 => 4 as u32,
            TextureFormat::BC3 => 1 as u32,
        }
    }

    pub fn display(&self) -> &str {
        match self {
            TextureFormat::RGBA8 => "TextureFormat::RGBA8",
            TextureFormat::BC3 => "TextureFormat::BC3",
        }
    }

    pub fn from_u32(value: u32) -> GenericResult<TextureFormat> {
        let endian = match value {
            6 => Self::RGBA8,
            20 => Self::BC3,
            _ => {
                let msg = format!("Could not convert value '{}' to TextureFormat.", value);
                return Err(msg.into());
            }
        };
        Ok(endian)
    }

    pub fn from_usize(value: usize) -> GenericResult<TextureFormat> {
        let endian = match value {
            6 => Self::RGBA8,
            20 => Self::BC3,
            _ => {
                let msg = format!("Could not convert value '{}' to TextureFormat.", value);
                return Err(msg.into());
            }
        };
        Ok(endian)
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn to_usize(&self) -> usize {
        *self as usize
    }
}

pub fn apply_endian(texture_endian: TextureEndian, buffer: &mut Vec<u8>) -> () {
    match texture_endian {
        TextureEndian::EndianNone => (),
        TextureEndian::Endian8in16 => {
            for i in (0..buffer.len()).step_by(2) {
                buffer.swap(i, i + 1);
            }
        }
        TextureEndian::Endian8in32 => {
            for i in (0..buffer.len()).step_by(4) {
                buffer.swap(i, i + 3);
                buffer.swap(i + 1, i + 2);
            }
        }
        TextureEndian::Endian16in32 => {
            for i in (0..buffer.len()).step_by(4) {
                buffer.swap(i, i + 2);
                buffer.swap(i + 1, i + 3);
            }
        }
    }
}

pub fn apply_swizzle(
    buffer: &mut Vec<u8>,
    swizzle_x: usize,
    swizzle_y: usize,
    swizzle_z: usize,
    swizzle_w: usize,
) -> () {
    let mut x: u8;
    let mut y: u8;
    let mut z: u8;
    let mut w: u8;
    for i in (0..buffer.len()).step_by(4) {
        x = buffer[i + swizzle_x];
        y = buffer[i + swizzle_y];
        z = buffer[i + swizzle_z];
        w = buffer[i + swizzle_w];
        buffer[i] = x;
        buffer[i + 1] = y;
        buffer[i + 2] = z;
        buffer[i + 3] = w;
    }
}

pub fn create_parent_directories(file_path: &Path) -> GenericResult<()> {
    match Path::new(file_path).parent() {
        Some(parent_path) => match std::fs::create_dir_all(parent_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        },
        None => Ok(()),
    }
}

pub fn image_from_be_bytes(bytes: Vec<u8>) -> GenericResult<image::DynamicImage> {
    Ok(image::io::Reader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?)
}
