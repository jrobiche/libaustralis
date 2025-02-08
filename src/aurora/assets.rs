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
// TODO remove print methods
use std::path::Path;

use self::AssetType::*;
use crate::utils::{
    apply_endian, apply_swizzle, create_parent_directories, GenericResult, TextureEndian,
    TextureFormat,
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(usize)]
pub enum AssetType {
    Icon = 0,
    Banner = 1,
    Boxart = 2,
    Slot = 3,
    Background = 4,
    Screenshot1 = 5,
    Screenshot2 = 6,
    Screenshot3 = 7,
    Screenshot4 = 8,
    Screenshot5 = 9,
    Screenshot6 = 10,
    Screenshot7 = 11,
    Screenshot8 = 12,
    Screenshot9 = 13,
    Screenshot10 = 14,
    Screenshot11 = 15,
    Screenshot12 = 16,
    Screenshot13 = 17,
    Screenshot14 = 18,
    Screenshot15 = 19,
    Screenshot16 = 20,
    Screenshot17 = 21,
    Screenshot18 = 22,
    Screenshot19 = 23,
    Screenshot20 = 24,
}

impl AssetType {
    pub fn display(&self) -> &str {
        match self {
            AssetType::Icon => "AssetType::Icon",
            AssetType::Banner => "AssetType::Banner",
            AssetType::Boxart => "AssetType::Boxart",
            AssetType::Slot => "AssetType::Slot",
            AssetType::Background => "AssetType::Background",
            AssetType::Screenshot1 => "AssetType::Screenshot1",
            AssetType::Screenshot2 => "AssetType::Screenshot2",
            AssetType::Screenshot3 => "AssetType::Screenshot3",
            AssetType::Screenshot4 => "AssetType::Screenshot4",
            AssetType::Screenshot5 => "AssetType::Screenshot5",
            AssetType::Screenshot6 => "AssetType::Screenshot6",
            AssetType::Screenshot7 => "AssetType::Screenshot7",
            AssetType::Screenshot8 => "AssetType::Screenshot8",
            AssetType::Screenshot9 => "AssetType::Screenshot9",
            AssetType::Screenshot10 => "AssetType::Screenshot10",
            AssetType::Screenshot11 => "AssetType::Screenshot11",
            AssetType::Screenshot12 => "AssetType::Screenshot12",
            AssetType::Screenshot13 => "AssetType::Screenshot13",
            AssetType::Screenshot14 => "AssetType::Screenshot14",
            AssetType::Screenshot15 => "AssetType::Screenshot15",
            AssetType::Screenshot16 => "AssetType::Screenshot16",
            AssetType::Screenshot17 => "AssetType::Screenshot17",
            AssetType::Screenshot18 => "AssetType::Screenshot18",
            AssetType::Screenshot19 => "AssetType::Screenshot19",
            AssetType::Screenshot20 => "AssetType::Screenshot20",
        }
    }

    pub fn from_u32(value: u32) -> GenericResult<Self> {
        let asset_type = match value {
            0 => AssetType::Icon,
            1 => AssetType::Banner,
            2 => AssetType::Boxart,
            3 => AssetType::Slot,
            4 => AssetType::Background,
            5 => AssetType::Screenshot1,
            6 => AssetType::Screenshot2,
            7 => AssetType::Screenshot3,
            8 => AssetType::Screenshot4,
            9 => AssetType::Screenshot5,
            10 => AssetType::Screenshot6,
            11 => AssetType::Screenshot7,
            12 => AssetType::Screenshot8,
            13 => AssetType::Screenshot9,
            14 => AssetType::Screenshot10,
            15 => AssetType::Screenshot11,
            16 => AssetType::Screenshot12,
            17 => AssetType::Screenshot13,
            18 => AssetType::Screenshot14,
            19 => AssetType::Screenshot15,
            20 => AssetType::Screenshot16,
            21 => AssetType::Screenshot17,
            22 => AssetType::Screenshot18,
            23 => AssetType::Screenshot19,
            24 => AssetType::Screenshot20,
            _ => {
                let msg = format!("Could not convert value '{}' to AssetType.", value);
                return Err(msg.into());
            }
        };
        Ok(asset_type)
    }

    pub fn from_usize(value: usize) -> GenericResult<Self> {
        let asset_type = match value {
            0 => AssetType::Icon,
            1 => AssetType::Banner,
            2 => AssetType::Boxart,
            3 => AssetType::Slot,
            4 => AssetType::Background,
            5 => AssetType::Screenshot1,
            6 => AssetType::Screenshot2,
            7 => AssetType::Screenshot3,
            8 => AssetType::Screenshot4,
            9 => AssetType::Screenshot5,
            10 => AssetType::Screenshot6,
            11 => AssetType::Screenshot7,
            12 => AssetType::Screenshot8,
            13 => AssetType::Screenshot9,
            14 => AssetType::Screenshot10,
            15 => AssetType::Screenshot11,
            16 => AssetType::Screenshot12,
            17 => AssetType::Screenshot13,
            18 => AssetType::Screenshot14,
            19 => AssetType::Screenshot15,
            20 => AssetType::Screenshot16,
            21 => AssetType::Screenshot17,
            22 => AssetType::Screenshot18,
            23 => AssetType::Screenshot19,
            24 => AssetType::Screenshot20,
            _ => {
                let msg = format!("Could not convert value '{}' to AssetType.", value);
                return Err(msg.into());
            }
        };
        Ok(asset_type)
    }

    pub fn is_screenshot(&self) -> bool {
        *self >= Self::Screenshot1 && *self <= Self::Screenshot20
    }

    pub fn into_iter() -> impl Iterator<Item = AssetType> {
        [
            Icon,
            Banner,
            Boxart,
            Slot,
            Background,
            Screenshot1,
            Screenshot2,
            Screenshot3,
            Screenshot4,
            Screenshot5,
            Screenshot6,
            Screenshot7,
            Screenshot8,
            Screenshot9,
            Screenshot10,
            Screenshot11,
            Screenshot12,
            Screenshot13,
            Screenshot14,
            Screenshot15,
            Screenshot16,
            Screenshot17,
            Screenshot18,
            Screenshot19,
            Screenshot20,
        ]
        .iter()
        .copied()
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn to_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub header: Header,
    pub image_data: Vec<u8>,
}

impl Asset {
    pub fn new() -> Self {
        let header = Header::new();
        let image_data: Vec<u8> = Vec::new();
        Self { header, image_data }
    }

    pub fn load(file_path: &Path) -> GenericResult<Self> {
        Self::read_from_file(file_path)
    }

    pub fn save(&self, file_path: &Path) -> GenericResult<()> {
        self.write_to_file(file_path)
    }

    pub fn export_image(
        &self,
        asset_type: AssetType,
        file_path: &Path,
    ) -> GenericResult<Option<()>> {
        self.write_image_to_file(asset_type, file_path)
    }

    pub fn export_padded_image(
        &self,
        asset_type: AssetType,
        file_path: &Path,
    ) -> GenericResult<Option<()>> {
        self.write_padded_image_to_file(asset_type, file_path)
    }

    pub fn import_image(
        &mut self,
        file_path: &Path,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        self.set_image_from_file(file_path, asset_type, texture_format)
    }

    pub fn delete_image(&mut self, asset_type: AssetType) -> GenericResult<()> {
        let asset_type_usize = asset_type.to_usize();
        let entry_image_data_index_u32 = self.header.asset_packs[asset_type_usize].image_data_index;
        let entry_image_data_index_usize = usize::try_from(entry_image_data_index_u32)?;
        let entry_image_data_length_u32 =
            self.header.asset_packs[asset_type_usize].image_data_length;
        let entry_image_data_length_usize = usize::try_from(entry_image_data_length_u32)?;
        let new_image_data_length = u32::try_from(
            self.image_data
                .len()
                .saturating_sub(entry_image_data_length_usize),
        )?;
        // update header information
        self.header.asset_types_flag &= !(1 << asset_type_usize);
        self.header.image_data_length = new_image_data_length;
        if entry_image_data_length_usize > 0 && asset_type.is_screenshot() {
            self.header.screenshot_count = self.header.screenshot_count.saturating_sub(1);
        }
        self.header.asset_packs[asset_type_usize] = AssetPackEntry::new();
        // remove bytes from `image_data` that belong to entry being removed
        self.image_data.drain(
            entry_image_data_index_usize
                ..(entry_image_data_index_usize.saturating_add(entry_image_data_length_usize)),
        );
        // adjust `image_data_index` for all entries that have an `image_data_index` greater
        // than the `image_data_index` of the asset pack entry being removed
        for x in AssetType::into_iter() {
            let x = x.to_usize();
            if self.header.asset_packs[x].image_data_index > entry_image_data_index_u32 {
                self.header.asset_packs[x].image_data_index = self.header.asset_packs[x]
                    .image_data_index
                    .saturating_sub(entry_image_data_length_u32);
            }
        }
        Ok(())
    }

    pub fn from_be_bytes(buffer: &Vec<u8>) -> GenericResult<Self> {
        let header = Header::from_be_bytes(buffer)?;
        let mut image_data: Vec<u8> = Vec::new();
        if buffer.len() > 0x800 {
            image_data.extend_from_slice(&buffer[0x800..]);
        }
        Ok(Self { header, image_data })
    }

    pub fn read_from_file(file_path: &Path) -> GenericResult<Self> {
        let asset_bytes: Vec<u8> = std::fs::read(file_path)?;
        Self::from_be_bytes(&asset_bytes)
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.to_be_bytes();
        buffer.extend(&self.image_data);
        buffer
    }

    pub fn write_to_file(&self, file_path: &Path) -> GenericResult<()> {
        create_parent_directories(&file_path)?;
        std::fs::write(&file_path, self.to_be_bytes())?;
        Ok(())
    }

    // methods to read asset images (without padding)

    pub fn image(&self, asset_type: AssetType) -> GenericResult<Option<image::DynamicImage>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        let (image_width, image_height) = match self.image_dimensions(asset_type) {
            (Some(w), Some(h), None) => (w, h),
            _ => {
                let msg = format!(
                    "Could not determine image width and height for asset type '{}'.",
                    asset_type.display()
                );
                return Err(msg.into());
            }
        };
        let image = match self.padded_image(asset_type)? {
            Some(mut padded_image) => Some(padded_image.crop(0, 0, image_width, image_height)),
            None => None,
        };
        Ok(image)
    }

    pub fn image_rgba8(&self, asset_type: AssetType) -> GenericResult<Option<Vec<u8>>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        let rgba8 = match self.image(asset_type)? {
            Some(image) => match image.as_rgba8() {
                Some(rgba8_image) => Some(rgba8_image.clone().into_vec()),
                None => {
                    let msg = format!(
                        "Failed to create RGBA8 image for asset type '{}'.",
                        asset_type.display()
                    );
                    return Err(msg.into());
                }
            },
            None => None,
        };
        Ok(rgba8)
    }

    pub fn write_image_to_file(
        &self,
        asset_type: AssetType,
        file_path: &Path,
    ) -> GenericResult<Option<()>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        match self.image(asset_type)? {
            Some(image) => {
                create_parent_directories(&file_path)?;
                image.save(file_path)?;
                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    // methods to read asset images (with padding)

    pub fn padded_image(
        &self,
        asset_type: AssetType,
    ) -> GenericResult<Option<image::DynamicImage>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        let asset_pack_entry = &self.header.asset_packs[asset_type.to_usize()];
        let gpu_fetch = &asset_pack_entry.texture_header.gpu_texture_fetch;
        let image_data_length = usize::try_from(asset_pack_entry.image_data_length)?;
        let image_data_index = usize::try_from(asset_pack_entry.image_data_index)?;
        let (image_width, image_height) = match self.padded_image_dimensions(asset_type)? {
            (Some(w), Some(h), None) => (w, h),
            _ => {
                let msg = format!(
                    "Could not determine padded image dimensions for asset type '{}'.",
                    asset_type.display()
                );
                return Err(msg.into());
            }
        };
        // determine which bytes in `image_data` correspond to texture for `asset_type`
        let image_data_index_end = image_data_index.saturating_add(image_data_length);
        if self.image_data.len() < image_data_index_end {
            let msg = format!(
                "Asset type '{}' requested image data range [{}, {}], but the length of all image data is only {}.",
                asset_type.display(),
                image_data_index,
                image_data_index_end,
                &self.image_data.len(),
            );
            return Err(msg.into());
        }
        let mut entry_image_data: Vec<u8> = vec![0; image_data_length];
        entry_image_data.copy_from_slice(
            &self.image_data[image_data_index..image_data_index.saturating_add(image_data_length)],
        );
        // apply endian, decompression, and swizzle to image's rgba8 bytes
        apply_endian(
            asset_pack_entry.texture_header.gpu_texture_fetch.endian()?,
            &mut entry_image_data,
        );
        let mut image_rgba8 = Self::decompress_image_data(
            entry_image_data,
            gpu_fetch.texture_format()?,
            usize::try_from(image_width)?,
            usize::try_from(image_height)?,
        )?;
        apply_swizzle(
            &mut image_rgba8,
            usize::try_from(gpu_fetch.swizzle_x())?,
            usize::try_from(gpu_fetch.swizzle_y())?,
            usize::try_from(gpu_fetch.swizzle_z())?,
            usize::try_from(gpu_fetch.swizzle_w())?,
        );
        let image_rgba8_len = &image_rgba8.len();
        let image = match image::RgbaImage::from_raw(image_width, image_height, image_rgba8) {
            Some(image_buffer) => image::DynamicImage::ImageRgba8(image_buffer),
            None => {
                let msg = format!(
                    "Failed to create image with width {} and height {} from {} bytes of RGBA data.",
                    image_width, image_height, image_rgba8_len
                );
                return Err(msg.into());
            }
        };
        Ok(Some(image))
    }

    pub fn padded_image_rgba8(&self, asset_type: AssetType) -> GenericResult<Option<Vec<u8>>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        let image_data = match self.padded_image(asset_type)? {
            Some(image) => match image.as_rgba8() {
                Some(rgba8_image) => Some(rgba8_image.clone().into_vec()),
                None => {
                    let msg = format!(
                        "Failed to create RGBA8 padded image for asset type '{}'.",
                        asset_type.display()
                    );
                    return Err(msg.into());
                }
            },
            None => None,
        };
        Ok(image_data)
    }

    pub fn write_padded_image_to_file(
        &self,
        asset_type: AssetType,
        file_path: &Path,
    ) -> GenericResult<Option<()>> {
        if !self.is_image_set(asset_type) {
            return Ok(None);
        }
        match self.padded_image(asset_type)? {
            Some(image) => {
                create_parent_directories(&file_path)?;
                image.save(file_path)?;
                Ok(Some(()))
            }
            None => Ok(None),
        }
    }

    // methods to update asset images

    pub fn set_image(
        &mut self,
        image: image::DynamicImage,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        let endian = TextureEndian::Endian8in16;
        let swizzle_x: u32 = 0;
        let swizzle_y: u32 = 1;
        let swizzle_z: u32 = 2;
        let swizzle_w: u32 = 3;
        let asset_type_usize = asset_type.to_usize();
        let padded_image = Self::pad_image(&image);
        let mut padded_image_rgba8 = match padded_image.as_rgba8() {
            Some(rgba8_image) => rgba8_image.clone().into_vec(),
            None => {
                let msg = format!(
                    "Failed to create RGBA8 padded image for asset type '{}'.",
                    asset_type.display()
                );
                return Err(msg.into());
            }
        };
        self.delete_image(asset_type)?;
        apply_swizzle(
            &mut padded_image_rgba8,
            usize::try_from(swizzle_x)?,
            usize::try_from(swizzle_y)?,
            usize::try_from(swizzle_z)?,
            usize::try_from(swizzle_w)?,
        );
        let mut padded_image_rgba8 = Self::compress_image_data(
            padded_image_rgba8,
            texture_format,
            usize::try_from(padded_image.width())?,
            usize::try_from(padded_image.height())?,
        )?;
        apply_endian(endian, &mut padded_image_rgba8);
        // update asset header and image_data
        let asset_pack_entry = &mut self.header.asset_packs[asset_type_usize];
        let texture_header = &mut asset_pack_entry.texture_header;
        let gpu_fetch = &mut texture_header.gpu_texture_fetch;
        asset_pack_entry.image_data_index = u32::try_from(self.image_data.len())?;
        asset_pack_entry.image_data_length = u32::try_from(padded_image_rgba8.len())?;
        texture_header.common = 3;
        texture_header.reference_count = 1;
        texture_header.base_flush = 0xFFFF0000;
        texture_header.mip_flush = 0xFFFF0000;
        gpu_fetch.set_pitch(u32::div_ceil(image.width(), 32));
        gpu_fetch.set_fetch_constant_type(2);
        gpu_fetch.set_endian(endian);
        gpu_fetch.set_texture_format(texture_format);
        gpu_fetch.set_swizzle_w(swizzle_w);
        gpu_fetch.set_swizzle_z(swizzle_z);
        gpu_fetch.set_swizzle_y(swizzle_y);
        gpu_fetch.set_swizzle_x(swizzle_x);
        gpu_fetch.set_packed_mips(1);
        gpu_fetch.set_dimension(1);
        gpu_fetch.set_width(image.width().saturating_sub(1))?;
        gpu_fetch.set_height(image.height().saturating_sub(1))?;
        self.image_data.extend_from_slice(&padded_image_rgba8);
        self.header.image_data_length = u32::try_from(self.image_data.len())?;
        if asset_type.is_screenshot() {
            self.header.screenshot_count += 1;
        }
        self.header.asset_types_flag |= 1 << asset_type_usize;
        Ok(())
    }

    pub fn set_image_from_rgba8(
        &mut self,
        width: u32,
        height: u32,
        rgba8: Vec<u8>,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        let rgba8_len = rgba8.len();
        let image = match image::RgbaImage::from_raw(width, height, rgba8) {
            Some(image_buffer) => image::DynamicImage::ImageRgba8(image_buffer),
            None => {
                let msg = format!(
                    "Failed to create image with width {} and height {} from {} bytes of RGBA data.",
                    width, height, rgba8_len
                );
                return Err(msg.into());
            }
        };
        self.set_image(image, asset_type, texture_format)
    }

    pub fn set_image_from_file(
        &mut self,
        file_path: &Path,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        let image = image::io::Reader::open(file_path)?.decode()?;
        self.set_image(image, asset_type, texture_format)
    }

    // misc methods

    pub fn is_image_set(&self, asset_type: AssetType) -> bool {
        self.header.asset_types_flag & (1 << asset_type.to_usize()) != 0
    }

    pub fn image_dimensions(
        &self,
        asset_type: AssetType,
    ) -> (Option<u32>, Option<u32>, Option<u32>) {
        if !self.is_image_set(asset_type) {
            return (None, None, None);
        }
        let gpu_fetch = &self.header.asset_packs[asset_type.to_usize()]
            .texture_header
            .gpu_texture_fetch;
        let width = match gpu_fetch.width() {
            Some(x) => Some(x + 1),
            None => None,
        };
        let height = match gpu_fetch.height() {
            Some(x) => Some(x + 1),
            None => None,
        };
        let depth = match gpu_fetch.depth() {
            Some(x) => Some(x + 1),
            None => None,
        };
        (width, height, depth)
    }

    pub fn padded_image_dimensions(
        &self,
        asset_type: AssetType,
    ) -> GenericResult<(Option<u32>, Option<u32>, Option<u32>)> {
        if !self.is_image_set(asset_type) {
            return Ok((None, None, None));
        }
        let asset_pack = &self.header.asset_packs[asset_type.to_usize()];
        let gpu_fetch = &asset_pack.texture_header.gpu_texture_fetch;
        let pitch = gpu_fetch.pitch();
        let texture_format = gpu_fetch.texture_format()?;
        if pitch == 0 {
            let msg = "Cannot calculate image dimensions because pitch is 0.";
            return Err(msg.into());
        }
        if gpu_fetch.stacked() || gpu_fetch.dimension() != 1 {
            let msg = "Cannot calculate padded image dimensions of image that is stacked or not 2 dimensional.";
            return Err(msg.into());
        }
        let width = 32 * pitch;
        let height = &asset_pack.image_data_length / (width * texture_format.bytes_per_pixel_u32());
        Ok((Some(width), Some(height), None))
    }

    // private methods

    fn compress_image_data(
        image_data: Vec<u8>,
        texture_format: TextureFormat,
        width: usize,
        height: usize,
    ) -> GenericResult<Vec<u8>> {
        match texture_format {
            TextureFormat::RGBA8 => Ok(image_data.clone()),
            TextureFormat::BC3 => {
                let bc3 = texpresso::Format::Bc3;
                let mut compressed_image_data: Vec<u8> =
                    vec![0; bc3.compressed_size(width, height)];
                bc3.compress(
                    &image_data,
                    width,
                    height,
                    texpresso::Params::default(),
                    &mut compressed_image_data,
                );
                Ok(compressed_image_data)
            }
        }
    }

    fn decompress_image_data(
        image_data: Vec<u8>,
        texture_format: TextureFormat,
        width: usize,
        height: usize,
    ) -> GenericResult<Vec<u8>> {
        match texture_format {
            TextureFormat::RGBA8 => Ok(image_data.clone()),
            TextureFormat::BC3 => {
                let mut rgba8 = vec![0; width * height * TextureFormat::RGBA8.bytes_per_pixel()];
                texpresso::Format::Bc3.decompress(&image_data, width, height, &mut rgba8);
                Ok(rgba8)
            }
        }
    }

    fn pad_image(image: &image::DynamicImage) -> image::DynamicImage {
        let width_pitch = u32::div_ceil(image.width(), 32);
        let height_pitch = u32::div_ceil(image.height(), 32);
        let padded_image_dimensions = (32 * width_pitch, 32 * height_pitch);
        let mut padded_image =
            image::DynamicImage::new_rgba8(padded_image_dimensions.0, padded_image_dimensions.1);
        image::imageops::overlay(&mut padded_image, image, 0, 0);
        padded_image
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub image_data_length: u32,
    pub asset_types_flag: u32,
    pub screenshot_count: u32,
    pub asset_packs: Vec<AssetPackEntry>,
    pub padding: Vec<u8>,
}

impl Header {
    pub fn new() -> Self {
        let magic = 0x52584541;
        let version = 1;
        let image_data_length = 0;
        let asset_types_flag = 0;
        let screenshot_count = 0;
        let asset_packs = vec![AssetPackEntry::new(); 25];
        let padding = vec![0; 428];
        Self {
            magic,
            version,
            image_data_length,
            asset_types_flag,
            screenshot_count,
            asset_packs,
            padding,
        }
    }

    pub fn from_be_bytes(buffer: &Vec<u8>) -> GenericResult<Self> {
        let magic = u32::from_be_bytes(buffer[0x0..0x4].try_into()?);
        let version = u32::from_be_bytes(buffer[0x4..0x8].try_into()?);
        let image_data_length = u32::from_be_bytes(buffer[0x8..0xC].try_into()?);
        let asset_types_flag = u32::from_be_bytes(buffer[0xC..0x10].try_into()?);
        let screenshot_count = u32::from_be_bytes(buffer[0x10..0x14].try_into()?);
        let mut asset_packs = vec![];
        for i in 0..25 {
            let asset_pack = AssetPackEntry::from_be_bytes(
                buffer[i * 0x40 + 0x14..i * 0x40 + 0x54].try_into()?,
            )?;
            asset_packs.push(asset_pack);
        }
        let padding = buffer[0x654..0x800].try_into()?;
        Ok(Self {
            magic,
            version,
            image_data_length,
            asset_types_flag,
            screenshot_count,
            asset_packs,
            padding,
        })
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.magic.to_be_bytes());
        buffer.extend_from_slice(&self.version.to_be_bytes());
        buffer.extend_from_slice(&self.image_data_length.to_be_bytes());
        buffer.extend_from_slice(&self.asset_types_flag.to_be_bytes());
        buffer.extend_from_slice(&self.screenshot_count.to_be_bytes());
        for asset_pack in &self.asset_packs {
            buffer.append(&mut asset_pack.to_be_bytes());
        }
        buffer.append(&mut self.padding.clone());
        buffer
    }

    pub fn print(&self) -> () {
        println!("AuroraAssetHeader Magic: 0x{:X}", self.magic);
        println!("AuroraAssetHeader Version: 0x{:X}", self.version);
        println!(
            "AuroraAssetHeader ImageDataLength: 0x{:X}",
            self.image_data_length
        );
        println!(
            "AuroraAssetHeader AssetTypesFlag: 0b{:b}",
            self.asset_types_flag
        );
        println!(
            "AuroraAssetHeader ScreenshotCount: {}",
            self.screenshot_count
        );
        for (i, asset_pack) in self.asset_packs.iter().enumerate() {
            if asset_pack.image_data_length == 0 {
                continue;
            }
            println!("AuroraAssetHeader AssetPack[{i}]:");
            asset_pack.print();
        }
        println!("AuroraAssetHeader Padding: {:?}", self.padding);
    }
}

#[derive(Clone, Debug)]
pub struct AssetPackEntry {
    pub image_data_index: u32,
    pub image_data_length: u32,
    pub extended_info: u32,
    pub texture_header: AssetPackTextureHeader,
}

impl AssetPackEntry {
    pub fn new() -> Self {
        let image_data_index = 0x0;
        let image_data_length = 0x0;
        let extended_info = 0x0;
        let texture_header = AssetPackTextureHeader::new();
        Self {
            image_data_index,
            image_data_length,
            extended_info,
            texture_header,
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        let image_data_index = u32::from_be_bytes(buffer[0x0..0x4].try_into()?);
        let image_data_length = u32::from_be_bytes(buffer[0x4..0x8].try_into()?);
        let extended_info = u32::from_be_bytes(buffer[0x8..0xC].try_into()?);
        let texture_header = AssetPackTextureHeader::from_be_bytes(buffer[0xC..0x40].try_into()?)?;
        Ok(Self {
            image_data_index,
            image_data_length,
            extended_info,
            texture_header,
        })
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.image_data_index.to_be_bytes());
        buffer.extend_from_slice(&self.image_data_length.to_be_bytes());
        buffer.extend_from_slice(&self.extended_info.to_be_bytes());
        buffer.append(&mut self.texture_header.to_be_bytes());
        buffer
    }

    pub fn print(&self) -> () {
        println!(
            "  AssetPackEntry ImageDataIndex: 0x{:X}",
            self.image_data_index
        );
        println!(
            "  AssetPackEntry ImageDataLength: 0x{:X}",
            self.image_data_length
        );
        println!("  AssetPackEntry ExtendedInfo: 0x{:X}", self.extended_info);
        self.texture_header.print();
    }
}

#[derive(Clone, Debug)]
pub struct AssetPackTextureHeader {
    pub common: u32,
    pub reference_count: u32,
    pub fence: u32,
    pub read_fence: u32,
    pub identifier: u32,
    pub base_flush: u32,
    pub mip_flush: u32,
    pub gpu_texture_fetch: GPUTextureFetch,
}

impl AssetPackTextureHeader {
    pub fn new() -> Self {
        let common = 0x0;
        let reference_count = 0x0;
        let fence = 0x0;
        let read_fence = 0x0;
        let identifier = 0x0;
        let base_flush = 0x0;
        let mip_flush = 0x0;
        let gpu_texture_fetch = GPUTextureFetch::new();
        Self {
            common,
            reference_count,
            fence,
            read_fence,
            identifier,
            base_flush,
            mip_flush,
            gpu_texture_fetch,
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        let common = u32::from_be_bytes(buffer[0x0..0x4].try_into()?);
        let reference_count = u32::from_be_bytes(buffer[0x4..0x8].try_into()?);
        let fence = u32::from_be_bytes(buffer[0x8..0xC].try_into()?);
        let read_fence = u32::from_be_bytes(buffer[0xC..0x10].try_into()?);
        let identifier = u32::from_be_bytes(buffer[0x10..0x14].try_into()?);
        let base_flush = u32::from_be_bytes(buffer[0x14..0x18].try_into()?);
        let mip_flush = u32::from_be_bytes(buffer[0x18..0x1C].try_into()?);
        let gpu_texture_fetch = GPUTextureFetch::from_be_bytes(buffer[0x1C..0x34].try_into()?)?;
        Ok(Self {
            common,
            reference_count,
            fence,
            read_fence,
            identifier,
            base_flush,
            mip_flush,
            gpu_texture_fetch,
        })
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.common.to_be_bytes());
        buffer.extend_from_slice(&self.reference_count.to_be_bytes());
        buffer.extend_from_slice(&self.fence.to_be_bytes());
        buffer.extend_from_slice(&self.read_fence.to_be_bytes());
        buffer.extend_from_slice(&self.identifier.to_be_bytes());
        buffer.extend_from_slice(&self.base_flush.to_be_bytes());
        buffer.extend_from_slice(&self.mip_flush.to_be_bytes());
        buffer.append(&mut self.gpu_texture_fetch.to_be_bytes());
        buffer
    }

    pub fn print(&self) -> () {
        println!("    AssetPackTextureHeader Common: {}", self.common);
        println!(
            "    AssetPackTextureHeader ReferenceCount: {}",
            self.reference_count
        );
        println!("    AssetPackTextureHeader Fence: {}", self.fence);
        println!("    AssetPackTextureHeader ReadFence: {}", self.read_fence);
        println!("    AssetPackTextureHeader Identifier: {}", self.identifier);
        println!(
            "    AssetPackTextureHeader BaseFlush: 0x{:X}",
            self.base_flush
        );
        println!(
            "    AssetPackTextureHeader MipFlush: 0x{:X}",
            self.mip_flush
        );
        self.gpu_texture_fetch.print();
    }
}

#[derive(Clone, Debug)]
pub struct GPUTextureFetch {
    pub constant0: u32,
    pub constant1: u32,
    pub constant2: u32,
    pub constant3: u32,
    pub constant4: u32,
    pub constant5: u32,
}

impl GPUTextureFetch {
    pub fn new() -> Self {
        let constant0 = 0;
        let constant1 = 0;
        let constant2 = 0;
        let constant3 = 0;
        let constant4 = 0;
        let constant5 = 0;
        Self {
            constant0,
            constant1,
            constant2,
            constant3,
            constant4,
            constant5,
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        let constant0 = u32::from_be_bytes(buffer[0x00..0x04].try_into()?);
        let constant1 = u32::from_be_bytes(buffer[0x04..0x08].try_into()?);
        let constant2 = u32::from_be_bytes(buffer[0x08..0x0C].try_into()?);
        let constant3 = u32::from_be_bytes(buffer[0x0C..0x10].try_into()?);
        let constant4 = u32::from_be_bytes(buffer[0x10..0x14].try_into()?);
        let constant5 = u32::from_be_bytes(buffer[0x14..0x18].try_into()?);
        Ok(Self {
            constant0,
            constant1,
            constant2,
            constant3,
            constant4,
            constant5,
        })
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.constant0.to_be_bytes());
        buffer.extend_from_slice(&self.constant1.to_be_bytes());
        buffer.extend_from_slice(&self.constant2.to_be_bytes());
        buffer.extend_from_slice(&self.constant3.to_be_bytes());
        buffer.extend_from_slice(&self.constant4.to_be_bytes());
        buffer.extend_from_slice(&self.constant5.to_be_bytes());
        buffer
    }

    pub fn print(&self) -> () {
        println!("      GPUTextureFetch Constant0: 0x{:0>8X}", self.constant0);
        println!("        Tiled: {}", self.tiled());
        println!("        Pitch: {}", self.pitch());
        println!("        Unknown0: {}", self.fc0_unknown0());
        println!(
            "        SignedRepeatingFractionMode: {}",
            self.signed_repeating_fraction_mode()
        );
        println!("        ClampZ: {}", self.clamp_z());
        println!("        ClampY: {}", self.clamp_y());
        println!("        ClampX: {}", self.clamp_x());
        println!("        SignW: {}", self.sign_w());
        println!("        SignZ: {}", self.sign_z());
        println!("        SignY: {}", self.sign_y());
        println!("        SignX: {}", self.sign_x());
        println!("        FetchConstantType: {}", self.fetch_constant_type());
        println!("      GPUTextureFetch Constant1: 0x{:0>8X}", self.constant1);
        println!("        BaseAddress: {}", self.base_address());
        println!("        ClampPolicy: {}", self.clamp_policy());
        println!("        Stacked: {}", self.stacked());
        println!("        RequestSize: {}", self.request_size());
        println!("        Endian: {:?}", self.endian());
        println!("        TextureFormat: {:?}", self.texture_format());
        println!("      GPUTextureFetch Constant2: 0x{:0>8X}", self.constant2);
        println!("        Depth: {:?}", self.depth());
        println!("        Height: {:?}", self.height());
        println!("        Width: {:?}", self.width());
        println!("      GPUTextureFetch Constant3: 0x{:0>8X}", self.constant3);
        println!("        BorderSize: {}", self.border_size());
        println!("        ArbitraryFilter: {}", self.arbitrary_filter());
        println!("        AnisoFilter: {}", self.aniso_filter());
        println!("        MipFilter: {}", self.mip_filter());
        println!("        MinFilter: {}", self.min_filter());
        println!("        MagFilter: {}", self.mag_filter());
        println!("        ExpAdjust: {}", self.exp_adjust());
        println!("        SwizzleW: {}", self.swizzle_w());
        println!("        SwizzleZ: {}", self.swizzle_z());
        println!("        SwizzleY: {}", self.swizzle_y());
        println!("        SwizzleX: {}", self.swizzle_x());
        println!("        NumFormat: {}", self.num_format());
        println!("      GPUTextureFetch Constant4: 0x{:0>8X}", self.constant4);
        println!("        GradExpAdjustV: {}", self.grad_exp_adjust_v());
        println!("        GradExpAdjustH: {}", self.grad_exp_adjust_h());
        println!("        LodBias: {}", self.lod_bias());
        println!("        MinAnisoWalk: {}", self.min_aniso_walk());
        println!("        MagAnisoWalk: {}", self.mag_aniso_walk());
        println!("        MaxMipLevel: {}", self.max_mip_level());
        println!("        MinMipLevel: {}", self.min_mip_level());
        println!("        VolMinFilter: {}", self.vol_min_filter());
        println!("        VolMagFilter: {}", self.vol_mag_filter());
        println!("      GPUTextureFetch Constant5: 0x{:0>8X}", self.constant5);
        println!("        MipAddress: {}", self.mip_address());
        println!("        PackedMips: {}", self.packed_mips());
        println!("        Dimension: {}", self.dimension());
        println!("        AnisoBias: {}", self.aniso_bias());
        println!("        TriClamp: {}", self.tri_clamp());
        println!("        ForceBcwToMax: {}", self.force_bcw_to_max());
        println!("        BorderColor: {}", self.border_color());
    }

    // Fetch Constant 0 Properties
    pub fn constant0(&self) -> u32 {
        self.constant0
    }

    pub fn set_constant0(&mut self, constant0: u32) -> () {
        self.constant0 = constant0
    }

    pub fn tiled(&self) -> bool {
        (self.constant0 & 0x80000000) >> 31 != 0
    }

    pub fn set_tiled(&mut self, value: bool) -> () {
        self.constant0 = (self.constant0 & !0x80000000) | ((value as u32) & 0b1) << 31
    }

    pub fn pitch(&self) -> u32 {
        (self.constant0 & 0x7FC00000) >> 22
    }

    pub fn set_pitch(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x7FC00000) | (value & 0b111_111_111) << 22
    }

    pub fn fc0_unknown0(&self) -> u32 {
        (self.constant0 & 0x00300000) >> 20
    }

    pub fn set_fc0_unknown0(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00300000) | (value & 0b11) << 20
    }

    pub fn signed_repeating_fraction_mode(&self) -> u32 {
        (self.constant0 & 0x00080000) >> 19
    }

    pub fn set_signed_repeating_fraction_mode(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00080000) | (value & 0b1) << 19
    }

    pub fn clamp_z(&self) -> u32 {
        (self.constant0 & 0x00070000) >> 16
    }

    pub fn set_clamp_z(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00070000) | (value & 0b111) << 16
    }

    pub fn clamp_y(&self) -> u32 {
        (self.constant0 & 0x0000E000) >> 13
    }

    pub fn set_clamp_y(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x0000E000) | (value & 0b111) << 13
    }

    pub fn clamp_x(&self) -> u32 {
        (self.constant0 & 0x00001C00) >> 10
    }

    pub fn set_clamp_x(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00001C00) | (value & 0b111) << 10
    }

    pub fn sign_w(&self) -> u32 {
        (self.constant0 & 0x00000300) >> 8
    }

    pub fn set_sign_w(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00000300) | (value & 0b11) << 8
    }

    pub fn sign_z(&self) -> u32 {
        (self.constant0 & 0x000000C0) >> 6
    }

    pub fn set_sign_z(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x000000C0) | (value & 0b11) << 6
    }

    pub fn sign_y(&self) -> u32 {
        (self.constant0 & 0x00000030) >> 4
    }

    pub fn set_sign_y(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00000030) | (value & 0b11) << 4
    }

    pub fn sign_x(&self) -> u32 {
        (self.constant0 & 0x0000000C) >> 2
    }

    pub fn set_sign_x(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x0000000C) | (value & 0b11) << 2
    }

    pub fn fetch_constant_type(&self) -> u32 {
        self.constant0 & 0x00000003
    }

    pub fn set_fetch_constant_type(&mut self, value: u32) -> () {
        self.constant0 = (self.constant0 & !0x00000003) | value & 0b11
    }

    // Fetch Constant 1 Properties
    pub fn constant1(&self) -> u32 {
        self.constant1
    }

    pub fn base_address(&self) -> u32 {
        (self.constant1 & 0xFFFFF000) >> 12
    }

    pub fn set_base_address(&mut self, value: u32) -> () {
        self.constant1 =
            (self.constant1 & !0xFFFFF000) | (value & 0b1111_1111_1111_1111_1111) << 12;
    }

    pub fn clamp_policy(&self) -> u32 {
        (self.constant1 & 0x00000800) >> 11
    }

    pub fn set_clamp_policy(&mut self, value: u32) -> () {
        self.constant1 = (self.constant1 & !0x00000800) | (value & 0b1) << 11
    }

    pub fn stacked(&self) -> bool {
        (self.constant1 & 0x00000400) >> 10 != 0
    }

    pub fn set_stacked(&mut self, value: bool) -> () {
        self.constant1 = (self.constant1 & !0x00000400) | ((value as u32) & 0b1) << 10
    }

    pub fn request_size(&self) -> u32 {
        (self.constant1 & 0x00000300) >> 8
    }

    pub fn set_request_size(&mut self, value: u32) -> () {
        self.constant1 = (self.constant1 & !0x00000300) | (value & 0b11) << 8
    }

    pub fn endian(&self) -> GenericResult<TextureEndian> {
        let value = (self.constant1 & 0x000000C0) >> 6;
        match value {
            0 => Ok(TextureEndian::EndianNone),
            1 => Ok(TextureEndian::Endian8in16),
            2 => Ok(TextureEndian::Endian8in32),
            3 => Ok(TextureEndian::Endian16in32),
            _ => Err(format!("The value {} is not a valid Texture Endian.", value).into()),
        }
    }

    pub fn set_endian(&mut self, endian: TextureEndian) -> () {
        self.constant1 = (self.constant1 & !0x000000C0) | (endian.to_u32() & 0b11) << 6
    }

    pub fn texture_format(&self) -> GenericResult<TextureFormat> {
        let value = self.constant1 & 0x0000003F;
        match value {
            6 => Ok(TextureFormat::RGBA8),
            20 => Ok(TextureFormat::BC3),
            _ => Err(format!("The value {} is not a supported Texture Format.", value).into()),
        }
    }

    pub fn set_texture_format(&mut self, format: TextureFormat) -> () {
        self.constant1 = (self.constant1 & !0x0000003F) | (format.to_u32() & 0b111_111)
    }

    // Fetch Constant 2 Properties
    pub fn constant2(&self) -> u32 {
        self.constant2
    }

    pub fn depth(&self) -> Option<u32> {
        match (self.stacked(), self.dimension()) {
            (false, 2) => Some((self.constant2 & 0xFFC00000) >> 22),
            (true, 1) => Some((self.constant2 & 0xFC000000) >> 26),
            _ => None,
        }
    }

    pub fn set_depth(&mut self, value: u32) -> GenericResult<()> {
        match (self.stacked(), self.dimension()) {
            (false, 2) => {
                self.constant2 = (self.constant2 & !0xFFC00000) | (value & 0b11_1111_1111) << 22;
                Ok(())
            }
            (true, 1) => {
                self.constant2 = (self.constant2 & !0xFC000000) | (value & 0b11_1111) << 26;
                Ok(())
            }
            (s, d) => Err(format!(
                "Cannot set depth on asset with stacked '{}' and dimension '{}'.",
                s, d
            )
            .into()),
        }
    }

    pub fn height(&self) -> Option<u32> {
        match (self.stacked(), self.dimension()) {
            (false, 1) => Some((self.constant2 & 0x03FFE000) >> 13),
            (false, 2) => Some((self.constant2 & 0x003FF800) >> 11),
            (true, 1) => Some((self.constant2 & 0x03FFE000) >> 13),
            _ => None,
        }
    }

    pub fn set_height(&mut self, value: u32) -> GenericResult<()> {
        match (self.stacked(), self.dimension()) {
            (false, 1) => {
                self.constant2 =
                    (self.constant2 & !0x03FFE000) | (value & 0b1_1111_1111_1111) << 13;
                Ok(())
            }
            (false, 2) => {
                self.constant2 = (self.constant2 & !0x003FF800) | (value & 0b111_1111_1111) << 11;
                Ok(())
            }
            (true, 1) => {
                self.constant2 =
                    (self.constant2 & !0x03FFE000) | (value & 0b1_1111_1111_1111) << 13;
                Ok(())
            }
            (s, d) => Err(format!(
                "Cannot set height on asset with stacked '{}' and dimension '{}'.",
                s, d
            )
            .into()),
        }
    }

    pub fn width(&self) -> Option<u32> {
        match (self.stacked(), self.dimension()) {
            (false, 0) => Some(self.constant2 & 0x00FFFFFF),
            (false, 1) => Some(self.constant2 & 0x00001FFF),
            (false, 2) => Some(self.constant2 & 0x000007FF),
            (false, 3) => Some(self.constant2 & 0x00001FFF),
            (true, 1) => Some(self.constant2 & 0x00001FFF),
            _ => None,
        }
    }

    pub fn set_width(&mut self, value: u32) -> GenericResult<()> {
        match (self.stacked(), self.dimension()) {
            (false, 0) => {
                self.constant2 =
                    (self.constant2 & !0x00FFFFFF) | (value & 0b1111_1111_1111_1111_1111_1111);
                Ok(())
            }
            (false, 1) => {
                self.constant2 = (self.constant2 & !0x00001FFF) | (value & 0b1_1111_1111_1111);
                Ok(())
            }
            (false, 2) => {
                self.constant2 = (self.constant2 & !0x000007FF) | (value & 0b111_1111_1111);
                Ok(())
            }
            (false, 3) => {
                self.constant2 = (self.constant2 & !0x00001FFF) | (value & 0b1_1111_1111_1111);
                Ok(())
            }
            (true, 1) => {
                self.constant2 = (self.constant2 & !0x00001FFF) | (value & 0b1_1111_1111_1111);
                Ok(())
            }
            (s, d) => Err(format!(
                "Cannot set width on asset with stacked '{}' and dimension '{}'.",
                s, d
            )
            .into()),
        }
    }

    // Fetch Constant 3 Properties
    pub fn constant3(&self) -> u32 {
        self.constant3
    }

    pub fn border_size(&self) -> u32 {
        (self.constant3 & 0x80000000) >> 31
    }

    pub fn set_border_size(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x80000000) | (value & 0b1) << 31
    }

    pub fn arbitrary_filter(&self) -> u32 {
        (self.constant3 & 0x70000000) >> 28
    }

    pub fn set_arbitrary_filter(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x70000000) | (value & 0b111) << 28
    }

    pub fn aniso_filter(&self) -> u32 {
        (self.constant3 & 0x0E000000) >> 25
    }

    pub fn set_aniso_filter(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x0E000000) | (value & 0b111) << 25
    }

    pub fn mip_filter(&self) -> u32 {
        (self.constant3 & 0x01800000) >> 23
    }

    pub fn set_mip_filter(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x01800000) | (value & 0b11) << 23
    }

    pub fn min_filter(&self) -> u32 {
        (self.constant3 & 0x00600000) >> 21
    }

    pub fn set_min_filter(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00600000) | (value & 0b11) << 21
    }

    pub fn mag_filter(&self) -> u32 {
        (self.constant3 & 0x00180000) >> 19
    }

    pub fn set_mag_filter(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00180000) | (value & 0b11) << 19
    }

    pub fn exp_adjust(&self) -> u32 {
        (self.constant3 & 0x0007E000) >> 13
    }

    pub fn set_exp_adjust(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x0007E000) | (value & 0b11_1111) << 13
    }

    pub fn swizzle_w(&self) -> u32 {
        (self.constant3 & 0x00001C00) >> 10
    }

    pub fn set_swizzle_w(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00001C00) | (value & 0b111) << 10
    }

    pub fn swizzle_z(&self) -> u32 {
        (self.constant3 & 0x00000380) >> 7
    }

    pub fn set_swizzle_z(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00000380) | (value & 0b111) << 7
    }

    pub fn swizzle_y(&self) -> u32 {
        (self.constant3 & 0x00000070) >> 4
    }

    pub fn set_swizzle_y(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00000070) | (value & 0b111) << 4
    }

    pub fn swizzle_x(&self) -> u32 {
        (self.constant3 & 0x0000000E) >> 1
    }

    pub fn set_swizzle_x(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x0000000E) | (value & 0b111) << 1
    }

    pub fn num_format(&self) -> u32 {
        self.constant3 & 0x00000001
    }

    pub fn set_num_format(&mut self, value: u32) -> () {
        self.constant3 = (self.constant3 & !0x00000001) | (value & 0b1)
    }

    // Fetch Constant 4 Properties
    pub fn constant4(&self) -> u32 {
        self.constant4
    }

    pub fn grad_exp_adjust_v(&self) -> u32 {
        (self.constant4 & 0xF8000000) >> 27
    }

    pub fn set_grad_exp_adjust_v(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0xF8000000) | (value & 0b1_1111) << 27
    }

    pub fn grad_exp_adjust_h(&self) -> u32 {
        (self.constant4 & 0x07C00000) >> 22
    }

    pub fn set_grad_exp_adjust_h(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x07C00000) | (value & 0b1_1111) << 22
    }

    pub fn lod_bias(&self) -> u32 {
        (self.constant4 & 0x003FF000) >> 12
    }

    pub fn set_lod_bias(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x003FF000) | (value & 0b11_1111_1111) << 12
    }

    pub fn min_aniso_walk(&self) -> u32 {
        (self.constant4 & 0x00000800) >> 11
    }

    pub fn set_min_aniso_walk(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x00000800) | (value & 0b1) << 11
    }

    pub fn mag_aniso_walk(&self) -> u32 {
        (self.constant4 & 0x00000400) >> 10
    }

    pub fn set_mag_aniso_walk(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x00000400) | (value & 0b1) << 10
    }

    pub fn max_mip_level(&self) -> u32 {
        (self.constant4 & 0x000003C0) >> 6
    }

    pub fn set_max_mip_level(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x000003C0) | (value & 0b1111) << 6
    }

    pub fn min_mip_level(&self) -> u32 {
        (self.constant4 & 0x0000003C) >> 2
    }

    pub fn set_min_mip_level(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x0000003C) | (value & 0b1111) << 2
    }

    pub fn vol_min_filter(&self) -> u32 {
        (self.constant4 & 0x00000002) >> 1
    }

    pub fn set_vol_min_filter(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x00000002) | (value & 0b1) << 1
    }

    pub fn vol_mag_filter(&self) -> u32 {
        self.constant4 & 0x00000001
    }

    pub fn set_vol_mag_filter(&mut self, value: u32) -> () {
        self.constant4 = (self.constant4 & !0x00000001) | (value & 0b1)
    }

    // Fetch Constant 5 Properties
    pub fn constant5(&self) -> u32 {
        self.constant5
    }

    pub fn mip_address(&self) -> u32 {
        (self.constant5 & 0xFFFFF000) >> 12
    }

    pub fn set_mip_address(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0xFFFFF000) | (value & 0b1111_1111_1111_1111_1111) << 12
    }

    pub fn packed_mips(&self) -> u32 {
        (self.constant5 & 0x00000800) >> 11
    }

    pub fn set_packed_mips(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x00000800) | (value & 0b1) << 11
    }

    pub fn dimension(&self) -> u32 {
        (self.constant5 & 0x00000600) >> 9
    }

    pub fn set_dimension(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x00000600) | (value & 0b11) << 9
    }

    pub fn aniso_bias(&self) -> u32 {
        (self.constant5 & 0x000001E0) >> 5
    }

    pub fn set_aniso_bias(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x000001E0) | (value & 0b1111) << 5
    }

    pub fn tri_clamp(&self) -> u32 {
        (self.constant5 & 0x00000018) >> 3
    }

    pub fn set_tri_clamp(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x00000018) | (value & 0b11) << 3
    }

    pub fn force_bcw_to_max(&self) -> u32 {
        (self.constant5 & 0x00000004) >> 2
    }

    pub fn set_force_bcw_to_max(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x00000004) | (value & 0b1) << 2
    }

    pub fn border_color(&self) -> u32 {
        self.constant5 & 0x00000003
    }

    pub fn set_border_color(&mut self, value: u32) -> () {
        self.constant5 = (self.constant5 & !0x00000003) | (value & 0b11)
    }
}
