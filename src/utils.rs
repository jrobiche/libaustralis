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
use std::ffi::OsStr;
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
    pub fn u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TextureFormat {
    RGBA8 = 6,
    BC3 = 20,
}

impl TextureFormat {
    pub fn u32(&self) -> u32 {
        *self as u32
    }
}

pub fn apply_endian(buffer: &mut Vec<u8>, endian: TextureEndian) -> () {
    match endian {
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
    let mut r: u8;
    let mut g: u8;
    let mut b: u8;
    let mut a: u8;
    for i in (0..buffer.len()).step_by(4) {
        r = buffer[i + swizzle_x];
        g = buffer[i + swizzle_y];
        b = buffer[i + swizzle_z];
        a = buffer[i + swizzle_w];
        buffer[i] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
        buffer[i + 3] = a;
    }
}

// TODO rename to create_parent_directories
pub fn create_parent_dirs<P: AsRef<OsStr> + AsRef<Path>>(file_path: &P) -> GenericResult<()> {
    match Path::new(file_path).parent() {
        Some(parent_path) => match std::fs::create_dir_all(parent_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        },
        None => Ok(()),
    }
}
