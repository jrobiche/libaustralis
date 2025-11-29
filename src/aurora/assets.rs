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
pub use image;
use std::fmt;
use std::path::Path;

use crate::utils::{create_parent_directories, GenericResult};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
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

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            AssetType::Icon => "Icon",
            AssetType::Banner => "Banner",
            AssetType::Boxart => "Boxart",
            AssetType::Slot => "Slot",
            AssetType::Background => "Background",
            AssetType::Screenshot1 => "Screenshot1",
            AssetType::Screenshot2 => "Screenshot2",
            AssetType::Screenshot3 => "Screenshot3",
            AssetType::Screenshot4 => "Screenshot4",
            AssetType::Screenshot5 => "Screenshot5",
            AssetType::Screenshot6 => "Screenshot6",
            AssetType::Screenshot7 => "Screenshot7",
            AssetType::Screenshot8 => "Screenshot8",
            AssetType::Screenshot9 => "Screenshot9",
            AssetType::Screenshot10 => "Screenshot10",
            AssetType::Screenshot11 => "Screenshot11",
            AssetType::Screenshot12 => "Screenshot12",
            AssetType::Screenshot13 => "Screenshot13",
            AssetType::Screenshot14 => "Screenshot14",
            AssetType::Screenshot15 => "Screenshot15",
            AssetType::Screenshot16 => "Screenshot16",
            AssetType::Screenshot17 => "Screenshot17",
            AssetType::Screenshot18 => "Screenshot18",
            AssetType::Screenshot19 => "Screenshot19",
            AssetType::Screenshot20 => "Screenshot20",
        };
        write!(f, "{}", text)
    }
}

impl AssetType {
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
            Self::Icon,
            Self::Banner,
            Self::Boxart,
            Self::Slot,
            Self::Background,
            Self::Screenshot1,
            Self::Screenshot2,
            Self::Screenshot3,
            Self::Screenshot4,
            Self::Screenshot5,
            Self::Screenshot6,
            Self::Screenshot7,
            Self::Screenshot8,
            Self::Screenshot9,
            Self::Screenshot10,
            Self::Screenshot11,
            Self::Screenshot12,
            Self::Screenshot13,
            Self::Screenshot14,
            Self::Screenshot15,
            Self::Screenshot16,
            Self::Screenshot17,
            Self::Screenshot18,
            Self::Screenshot19,
            Self::Screenshot20,
        ]
        .iter()
        .copied()
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum TextureEndian {
    EndianNone = 0,
    Endian8in16 = 1,
    Endian8in32 = 2,
    Endian16in32 = 3,
}

impl fmt::Display for TextureEndian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            TextureEndian::EndianNone => "EndianNone",
            TextureEndian::Endian8in16 => "Endian8in16",
            TextureEndian::Endian8in32 => "Endian8in32",
            TextureEndian::Endian16in32 => "Endian16in32",
        };
        write!(f, "{}", text)
    }
}

impl TextureEndian {
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

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum TextureFormat {
    RGBA8 = 6,
    BC3 = 20,
}

impl fmt::Display for TextureFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            TextureFormat::RGBA8 => "RGBA8",
            TextureFormat::BC3 => "BC3",
        };
        write!(f, "{}", text)
    }
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

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub header: Header,
    pub image_data: Vec<u8>,
}

impl Asset {
    ////////////////////////////////////////////////////////////////////////////////
    // methods related to the entire asset file
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            image_data: Vec::new(),
        }
    }

    pub fn load(file_path: &Path) -> GenericResult<Self> {
        Self::from_be_bytes(&std::fs::read(file_path)?)
    }

    pub fn save(&self, file_path: &Path) -> GenericResult<()> {
        create_parent_directories(&file_path)?;
        std::fs::write(&file_path, self.to_be_bytes())?;
        Ok(())
    }

    pub fn from_be_bytes(buffer: &Vec<u8>) -> GenericResult<Self> {
        if buffer.len() < 0x800 {
            let msg = "Could not create Asset from BE bytes. Not enough bytes to be a valid Asset.";
            return Err(msg.into());
        }
        let header = Header::from_be_bytes(buffer)?;
        let image_data = match buffer.len() > 0x800 {
            true => buffer[0x800..].to_vec(),
            false => Vec::new(),
        };
        Ok(Self { header, image_data })
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.to_be_bytes();
        buffer.extend(&self.image_data);
        buffer
    }

    ////////////////////////////////////////////////////////////////////////////////
    // methods related to asset images
    ////////////////////////////////////////////////////////////////////////////////
    pub fn export_image(
        &self,
        asset_type: AssetType,
        file_path: &Path,
    ) -> GenericResult<Option<()>> {
        if !self.has_image(asset_type) {
            return Ok(None);
        }
        let result = match self.image(asset_type)? {
            Some(image) => {
                create_parent_directories(&file_path)?;
                image.save(file_path)?;
                Some(())
            }
            None => None,
        };
        Ok(result)
    }

    pub fn import_image(
        &mut self,
        file_path: &Path,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        let image = image::ImageReader::open(file_path)?.decode()?;
        self.set_image(image, asset_type, texture_format)
    }

    pub fn delete_image(&mut self, asset_type: AssetType) -> GenericResult<()> {
        let entry_image_data_index_u32 =
            self.header.asset_packs[asset_type.as_usize()].image_data_index;
        let entry_image_data_index_usize = usize::try_from(entry_image_data_index_u32)?;
        let entry_image_data_length_u32 =
            self.header.asset_packs[asset_type.as_usize()].image_data_length;
        let entry_image_data_length_usize = usize::try_from(entry_image_data_length_u32)?;
        // remove bytes from `image_data` that belong to entry being removed
        self.image_data.drain(
            entry_image_data_index_usize
                ..(entry_image_data_index_usize.saturating_add(entry_image_data_length_usize)),
        );
        // update header information
        self.header.asset_types_flag &= !(1 << asset_type.as_usize());
        self.header.image_data_length = u32::try_from(self.image_data.len())?;
        if entry_image_data_length_usize > 0 && asset_type.is_screenshot() {
            self.header.screenshot_count = self.header.screenshot_count.saturating_sub(1);
        }
        self.header.asset_packs[asset_type.as_usize()] = AssetPackEntry::new();
        // adjust `image_data_index` for all entries that have an `image_data_index` greater
        // than the `image_data_index` of the asset pack entry being removed
        for asset_type in AssetType::into_iter() {
            let asset_pack = &mut self.header.asset_packs[asset_type.as_usize()];
            if asset_pack.image_data_index > entry_image_data_index_u32 {
                asset_pack.image_data_index = asset_pack
                    .image_data_index
                    .saturating_sub(entry_image_data_length_u32);
            }
        }
        Ok(())
    }

    // TODO review
    pub fn image(&self, asset_type: AssetType) -> GenericResult<Option<image::DynamicImage>> {
        if !self.has_image(asset_type) {
            return Ok(None);
        }
        let (image_width, image_height) = match self.image_dimensions(asset_type) {
            (Some(w), Some(h), None) => (w, h),
            _ => {
                let msg = format!(
                    "Could not determine image width and height for asset type '{}'.",
                    asset_type
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

    // TODO review
    pub fn image_rgba8(&self, asset_type: AssetType) -> GenericResult<Option<Vec<u8>>> {
        if !self.has_image(asset_type) {
            return Ok(None);
        }
        let rgba8 = match self.image(asset_type)? {
            Some(image) => match image.as_rgba8() {
                Some(rgba8_image) => Some(rgba8_image.clone().into_vec()),
                None => {
                    let msg = format!(
                        "Failed to create RGBA8 image for asset type '{}'.",
                        asset_type
                    );
                    return Err(msg.into());
                }
            },
            None => None,
        };
        Ok(rgba8)
    }

    // TODO review
    // TODO make private method that contains the complex logic so that multiple public methods can easily call it
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
        let padded_image = Self::pad_image(&image);
        let mut padded_image_rgba8 = match padded_image.as_rgba8() {
            Some(rgba8_image) => rgba8_image.clone().into_vec(),
            None => {
                let msg = format!(
                    "Failed to create RGBA8 padded image for asset type '{}'.",
                    asset_type
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
        apply_endian(&mut padded_image_rgba8, endian);
        // update asset header and image_data
        let asset_pack_entry = &mut self.header.asset_packs[asset_type.as_usize()];
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
        self.header.asset_types_flag |= 1 << asset_type.as_usize();
        Ok(())
    }

    // TODO review
    pub fn set_image_from_rgba8(
        &mut self,
        width: u32,
        height: u32,
        rgba8: Vec<u8>,
        asset_type: AssetType,
        texture_format: TextureFormat,
    ) -> GenericResult<()> {
        let rgba8_len = rgba8.len();
        match image::RgbaImage::from_raw(width, height, rgba8) {
            Some(image_buffer) => {
                let image = image::DynamicImage::ImageRgba8(image_buffer);
                self.set_image(image, asset_type, texture_format)
            }
            None => {
                let msg = format!(
                    "Failed to create image with width {} and height {} from {} bytes of RGBA data.",
                    width, height, rgba8_len
                );
                Err(msg.into())
            }
        }
    }

    pub fn has_image(&self, asset_type: AssetType) -> bool {
        self.header.asset_types_flag & (1 << asset_type.as_usize()) != 0
    }

    ////////////////////////////////////////////////////////////////////////////////
    // private methods
    ////////////////////////////////////////////////////////////////////////////////
    // TODO review
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

    // TODO review
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

    fn image_dimensions(&self, asset_type: AssetType) -> (Option<u32>, Option<u32>, Option<u32>) {
        if !self.has_image(asset_type) {
            return (None, None, None);
        }
        let gpu_fetch = &self.header.asset_packs[asset_type.as_usize()]
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

    // TODO review
    fn pad_image(image: &image::DynamicImage) -> image::DynamicImage {
        let width = 32 * u32::div_ceil(image.width(), 32);
        let height = 32 * u32::div_ceil(image.height(), 32);
        let mut padded_image = image::DynamicImage::new_rgba8(width, height);
        image::imageops::overlay(&mut padded_image, image, 0, 0);
        padded_image
    }

    // TODO review
    fn padded_image(&self, asset_type: AssetType) -> GenericResult<Option<image::DynamicImage>> {
        if !self.has_image(asset_type) {
            return Ok(None);
        }
        let asset_pack_entry = &self.header.asset_packs[asset_type.as_usize()];
        let gpu_fetch = &asset_pack_entry.texture_header.gpu_texture_fetch;
        let image_data_length = usize::try_from(asset_pack_entry.image_data_length)?;
        let image_data_index = usize::try_from(asset_pack_entry.image_data_index)?;
        let (image_width, image_height) = match self.padded_image_dimensions(asset_type)? {
            (Some(w), Some(h), None) => (w, h),
            _ => {
                let msg = format!(
                    "Could not determine padded image dimensions for asset type '{}'.",
                    asset_type
                );
                return Err(msg.into());
            }
        };
        // determine which bytes in `image_data` correspond to texture for `asset_type`
        let image_data_index_end = image_data_index.saturating_add(image_data_length);
        if self.image_data.len() < image_data_index_end {
            let msg = format!(
                "Asset type '{}' requested image data range [{}, {}], but the length of all image data is only {}.",
                asset_type,
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
            &mut entry_image_data,
            asset_pack_entry.texture_header.gpu_texture_fetch.endian()?,
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

    // TODO review
    fn padded_image_dimensions(
        &self,
        asset_type: AssetType,
    ) -> GenericResult<(Option<u32>, Option<u32>, Option<u32>)> {
        if !self.has_image(asset_type) {
            return Ok((None, None, None));
        }
        let asset_pack = &self.header.asset_packs[asset_type.as_usize()];
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
}

#[derive(Clone, Debug)]
pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub image_data_length: u32,
    pub asset_types_flag: u32,
    pub screenshot_count: u32,
    pub asset_packs: Vec<AssetPackEntry>, // TODO should this be HashMap<AssetType, AssetPackEntry>?
    pub padding: Vec<u8>,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut asset_pack_text = String::from("");
        for (i, asset_pack) in self.asset_packs.iter().enumerate() {
            let prefix = format!("\nasset_packs[{}].", i);
            let new_asset_pack_text = format!("\n{}", asset_pack);
            let new_asset_pack_text = new_asset_pack_text.replace('\n', &prefix);
            asset_pack_text.push_str(&new_asset_pack_text);
        }
        let text = format!(
            "magic: 0x{:08X}\n\
             version: 0x{:08X}\n\
             image_data_length: 0x{:08X}\n\
             asset_types_flag: 0b{:032b}\n\
             screenshot_count: {}{}\n\
             padding: {:?}",
            self.magic,
            self.version,
            self.image_data_length,
            self.asset_types_flag,
            self.screenshot_count,
            asset_pack_text,
            self.padding,
        );
        write!(f, "{}", text)
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            magic: 0x52584541,
            version: 1,
            image_data_length: 0,
            asset_types_flag: 0,
            screenshot_count: 0,
            asset_packs: vec![AssetPackEntry::new(); 25],
            padding: vec![0; 428],
        }
    }

    pub fn from_be_bytes(buffer: &Vec<u8>) -> GenericResult<Self> {
        // TODO validate buffer size
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
}

#[derive(Clone, Debug)]
pub struct AssetPackEntry {
    pub image_data_index: u32,
    pub image_data_length: u32,
    pub extended_info: u32,
    pub texture_header: AssetPackTextureHeader,
}

impl fmt::Display for AssetPackEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texture_header_text = format!("texture_header.{}", self.texture_header);
        let texture_header_text = texture_header_text.replace('\n', "\ntexture_header.");
        let text = format!(
            "image_data_index: 0x{:08X}\n\
             image_data_length: 0x{:08X}\n\
             extended_info: 0x{:08X}\n\
             {}",
            self.image_data_index, self.image_data_length, self.extended_info, texture_header_text
        );
        write!(f, "{}", text)
    }
}

impl AssetPackEntry {
    pub fn new() -> Self {
        Self {
            image_data_index: 0,
            image_data_length: 0,
            extended_info: 0,
            texture_header: AssetPackTextureHeader::new(),
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        // TODO validate buffer size
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

impl fmt::Display for AssetPackTextureHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texture_fetch_text = format!("gpu_texture_fetch.{}", self.gpu_texture_fetch);
        let texture_fetch_text = texture_fetch_text.replace('\n', "\ngpu_texture_fetch.");
        let text = format!(
            "common: 0x{:08X}\n\
             reference_count: 0x{:08X}\n\
             fence: 0x{:08X}\n\
             read_fence: 0x{:08X}\n\
             identifier: 0x{:08X}\n\
             base_flush: 0x{:08X}\n\
             mip_flush: 0x{:08X}\n\
             {}",
            self.common,
            self.reference_count,
            self.fence,
            self.read_fence,
            self.identifier,
            self.base_flush,
            self.mip_flush,
            texture_fetch_text,
        );
        write!(f, "{}", text)
    }
}

impl AssetPackTextureHeader {
    pub fn new() -> Self {
        Self {
            common: 0,
            reference_count: 0,
            fence: 0,
            read_fence: 0,
            identifier: 0,
            base_flush: 0,
            mip_flush: 0,
            gpu_texture_fetch: GPUTextureFetch::new(),
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        // TODO validate buffer size
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

impl fmt::Display for GPUTextureFetch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = format!(
            "constant0: 0x{:08X}\n\
             tiled: {}\n\
             pitch: {}\n\
             fc0_unknown0: {}\n\
             signed_repeating_fraction_mode: {}\n\
             clamp_z: {}\n\
             clamp_y: {}\n\
             clamp_x: {}\n\
             sign_w: {}\n\
             sign_z: {}\n\
             sign_y: {}\n\
             sign_x: {}\n\
             fetch_constant_type: {}\n\
             constant1: 0x{:08X}\n\
             base_address: {}\n\
             clamp_policy: {}\n\
             stacked: {}\n\
             request_size: {}\n\
             endian_u32: {}\n\
             texture_format_u32: {}\n\
             constant2: 0x{:08X}\n\
             depth: {:?}\n\
             height: {:?}\n\
             width: {:?}\n\
             constant3: 0x{:08X}\n\
             border_size: {}\n\
             arbitrary_filter: {}\n\
             aniso_filter: {}\n\
             mip_filter: {}\n\
             min_filter: {}\n\
             mag_filter: {}\n\
             exp_adjust: {}\n\
             swizzle_w: {}\n\
             swizzle_z: {}\n\
             swizzle_y: {}\n\
             swizzle_x: {}\n\
             num_format: {}\n\
             constant4: 0x{:08X}\n\
             grad_exp_adjust_v: {}\n\
             grad_exp_adjust_h: {}\n\
             lod_bias: {}\n\
             min_aniso_walk: {}\n\
             mag_aniso_walk: {}\n\
             max_mip_level: {}\n\
             min_mip_level: {}\n\
             vol_min_filter: {}\n\
             vol_mag_filter: {}\n\
             constant5: 0x{:08X}\n\
             mip_address: {}\n\
             packed_mips: {}\n\
             dimension: {}\n\
             aniso_bias: {}\n\
             tri_clamp: {}\n\
             force_bcw_to_max: {}\n\
             border_color: {}",
            self.constant0,
            self.tiled(),
            self.pitch(),
            self.fc0_unknown0(),
            self.signed_repeating_fraction_mode(),
            self.clamp_z(),
            self.clamp_y(),
            self.clamp_x(),
            self.sign_w(),
            self.sign_z(),
            self.sign_y(),
            self.sign_x(),
            self.fetch_constant_type(),
            self.constant1,
            self.base_address(),
            self.clamp_policy(),
            self.stacked(),
            self.request_size(),
            self.endian_u32(),
            self.texture_format_u32(),
            self.constant2,
            self.depth(),
            self.height(),
            self.width(),
            self.constant3,
            self.border_size(),
            self.arbitrary_filter(),
            self.aniso_filter(),
            self.mip_filter(),
            self.min_filter(),
            self.mag_filter(),
            self.exp_adjust(),
            self.swizzle_w(),
            self.swizzle_z(),
            self.swizzle_y(),
            self.swizzle_x(),
            self.num_format(),
            self.constant4,
            self.grad_exp_adjust_v(),
            self.grad_exp_adjust_h(),
            self.lod_bias(),
            self.min_aniso_walk(),
            self.mag_aniso_walk(),
            self.max_mip_level(),
            self.min_mip_level(),
            self.vol_min_filter(),
            self.vol_mag_filter(),
            self.constant5,
            self.mip_address(),
            self.packed_mips(),
            self.dimension(),
            self.aniso_bias(),
            self.tri_clamp(),
            self.force_bcw_to_max(),
            self.border_color(),
        );
        write!(f, "{}", text)
    }
}

impl GPUTextureFetch {
    pub fn new() -> Self {
        Self {
            constant0: 0,
            constant1: 0,
            constant2: 0,
            constant3: 0,
            constant4: 0,
            constant5: 0,
        }
    }

    pub fn from_be_bytes(buffer: Vec<u8>) -> GenericResult<Self> {
        // TODO validate buffer size
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
        TextureEndian::from_u32(self.endian_u32())
    }

    pub fn endian_u32(&self) -> u32 {
        (self.constant1 & 0x000000C0) >> 6
    }

    pub fn set_endian(&mut self, endian: TextureEndian) -> () {
        self.constant1 = (self.constant1 & !0x000000C0) | (endian.as_u32() & 0b11) << 6
    }

    pub fn texture_format(&self) -> GenericResult<TextureFormat> {
        TextureFormat::from_u32(self.texture_format_u32())
    }

    pub fn texture_format_u32(&self) -> u32 {
        self.constant1 & 0x0000003F
    }

    pub fn set_texture_format(&mut self, format: TextureFormat) -> () {
        self.constant1 = (self.constant1 & !0x0000003F) | (format.as_u32() & 0b111_111)
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

pub fn apply_endian(buffer: &mut Vec<u8>, texture_endian: TextureEndian) -> () {
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
