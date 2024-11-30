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
pub mod assets;
pub mod ftp;

use std::ffi::OsStr;
use std::path::Path;

use crate::utils::{
    apply_endian, apply_swizzle, create_parent_dirs, GenericResult, TextureEndian, TextureFormat,
};
use assets::{AssetPackEntry, AssetType, Header};

// TODO move to aurora/assets.rs
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

    pub fn from_be_bytes(buffer: &Vec<u8>) -> GenericResult<Self> {
        let header = Header::from_be_bytes(buffer)?;
        let mut image_data: Vec<u8> = Vec::new();
        if buffer.len() > 0x800 {
            image_data.extend_from_slice(&buffer[0x800..]);
        }
        Ok(Self { header, image_data })
    }

    pub fn as_be_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_be_bytes();
        buffer.extend(&self.image_data);
        buffer
    }

    pub fn read_file<P: AsRef<Path>>(file_path: P) -> GenericResult<Self> {
        let asset_bytes: Vec<u8> = std::fs::read(file_path)?;
        Self::from_be_bytes(&asset_bytes)
    }

    pub fn write_file<P: AsRef<OsStr> + AsRef<Path>>(&self, file_path: &P) -> GenericResult<()> {
        create_parent_dirs(&file_path)?;
        std::fs::write(file_path, self.as_be_bytes())?;
        Ok(())
    }

    pub fn delete_image(&mut self, asset_type: AssetType) -> GenericResult<()> {
        let asset_type_usize = asset_type.usize();
        // remove bytes from `image_data` that belong to entry being removed
        let entry_image_data_index =
            usize::try_from(self.header.asset_packs[asset_type_usize].image_data_index)?;
        let entry_image_data_length =
            usize::try_from(self.header.asset_packs[asset_type_usize].image_data_length)?;
        self.image_data
            .drain(entry_image_data_index..(entry_image_data_index + entry_image_data_length));
        // update header information
        self.header.image_data_length = u32::try_from(self.image_data.len())?;
        self.header.asset_types_flag &= !(1 << asset_type_usize);
        if entry_image_data_length > 0 && asset_type.is_screenshot() {
            self.header.screenshot_count = self.header.screenshot_count.saturating_sub(1);
        }
        // adjust `image_data_index` for all entries that have an `image_data_index` greater
        // than the `image_data_index` of the asset pack entry being removed
        for x in AssetType::iterator() {
            let x = x.usize();
            if self.header.asset_packs[x].image_data_index
                > self.header.asset_packs[asset_type_usize].image_data_index
            {
                self.header.asset_packs[x].image_data_index = self.header.asset_packs[x]
                    .image_data_index
                    .saturating_sub(self.header.asset_packs[asset_type_usize].image_data_length);
            }
        }
        // replace asset pack `asset_type` with a new `AssetPackEntry`
        self.header.asset_packs[asset_type_usize] = AssetPackEntry::new();
        Ok(())
    }

    pub fn export_image<P: AsRef<OsStr> + AsRef<Path>>(
        &self,
        asset_type: AssetType,
        file_path: &P,
    ) -> GenericResult<Option<()>> {
        if !self.is_asset_set(asset_type) {
            return Ok(None);
        }
        // get padded image information
        let (padded_image_width, padded_image_height, padded_image_data) =
            self.padded_image(asset_type)?;
        let padded_image_data_len = &padded_image_data.len();
        // determine dimensions of the image without padding
        let (image_width, image_height) = match self.image_dimensions(asset_type) {
            (Some(width), Some(height), _) => (width, height),
            _ => {
                return Err(format!(
                    "Could not get image width and height for asset type {:?}.",
                    asset_type
                )
                .into());
            }
        };
        // create cropped image from padded image data
        let image = match image::RgbaImage::from_raw(
            padded_image_width,
            padded_image_height,
            padded_image_data,
        ) {
            Some(image_buffer) => {
                image::DynamicImage::ImageRgba8(image_buffer).crop(0, 0, image_width, image_height)
            }
            None => {
                return Err(format!(
                "Failed to create image with width {} and height {} from {} bytes of image data.",
                padded_image_width, padded_image_height, padded_image_data_len
            )
                .into())
            }
        };
        // write cropped image to `file_path`
        create_parent_dirs(&file_path)?;
        image.save(file_path)?;
        Ok(Some(()))
    }

    pub fn import_image<P: AsRef<Path>>(
        &mut self,
        file_path: &P,
        asset_type: AssetType,
        texture_format: Option<TextureFormat>,
    ) -> GenericResult<()> {
        let texture_format = match texture_format {
            Some(x) => x,
            None => TextureFormat::RGBA8,
        };
        let endian = TextureEndian::Endian8in16;
        let swizzle_x: u32 = 0;
        let swizzle_y: u32 = 1;
        let swizzle_z: u32 = 2;
        let swizzle_w: u32 = 3;
        let asset_type_usize = asset_type.usize();
        let (
            (cropped_image_width, cropped_image_height),
            (padded_image_width, padded_image_height),
            pitch,
            mut padded_image_data,
        ) = self.create_padded_image_from_file(file_path)?;
        self.delete_image(asset_type)?;
        apply_swizzle(
            &mut padded_image_data,
            usize::try_from(swizzle_x)?,
            usize::try_from(swizzle_y)?,
            usize::try_from(swizzle_z)?,
            usize::try_from(swizzle_w)?,
        );
        let mut padded_image_data = Self::compress_image_data(
            padded_image_data,
            texture_format,
            usize::try_from(padded_image_width)?,
            usize::try_from(padded_image_height)?,
        )?;
        apply_endian(&mut padded_image_data, endian);
        // update asset header and image_data
        let asset_pack_entry = &mut self.header.asset_packs[asset_type_usize];
        let texture_header = &mut asset_pack_entry.texture_header;
        let gpu_fetch = &mut texture_header.gpu_texture_fetch;
        asset_pack_entry.image_data_index = u32::try_from(self.image_data.len())?;
        asset_pack_entry.image_data_length = u32::try_from(padded_image_data.len())?;
        texture_header.common = 3;
        texture_header.base_flush = 0xFFFF0000;
        texture_header.mip_flush = 0xFFFF0000;
        gpu_fetch.set_pitch(pitch);
        gpu_fetch.set_fetch_constant_type(2);
        gpu_fetch.set_endian(endian);
        gpu_fetch.set_texture_format(texture_format);
        gpu_fetch.set_swizzle_w(swizzle_w);
        gpu_fetch.set_swizzle_z(swizzle_z);
        gpu_fetch.set_swizzle_y(swizzle_y);
        gpu_fetch.set_swizzle_x(swizzle_x);
        gpu_fetch.set_packed_mips(1);
        gpu_fetch.set_dimension(1);
        gpu_fetch.set_width(cropped_image_width)?;
        gpu_fetch.set_height(cropped_image_height)?;
        self.image_data.extend_from_slice(&padded_image_data);
        self.header.image_data_length = u32::try_from(self.image_data.len())?;
        if asset_type.is_screenshot() {
            self.header.screenshot_count += 1;
        }
        self.header.asset_types_flag |= 1 << asset_type_usize;
        Ok(())
    }

    fn bytes_per_pixel(texture_format: TextureFormat) -> GenericResult<u32> {
        match texture_format {
            TextureFormat::RGBA8 => Ok(4),
            TextureFormat::BC3 => Ok(1),
        }
    }

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

    fn create_padded_image_from_file<P: AsRef<Path>>(
        &self,
        image_file_path: P,
    ) -> GenericResult<((u32, u32), (u32, u32), u32, Vec<u8>)> {
        let image = image::io::Reader::open(image_file_path)?.decode()?;
        let width_pitch = u32::div_ceil(image.width(), 32);
        let height_pitch = u32::div_ceil(image.height(), 32);
        let padded_image_dimensions = (32 * width_pitch, 32 * height_pitch);
        let mut padded_image =
            image::DynamicImage::new_rgba8(padded_image_dimensions.0, padded_image_dimensions.1);
        image::imageops::overlay(&mut padded_image, &image, 0, 0);
        // return:
        // - original image dimensions
        // - padded image dimensions
        // - padded image pitch
        // - padded image data (RGBA8)
        Ok((
            (image.width(), image.height()),
            padded_image_dimensions,
            width_pitch,
            padded_image.to_rgba8().into_vec(),
        ))
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
                let mut rgba8 = vec![0; width * height * 4];
                texpresso::Format::Bc3.decompress(&image_data, width, height, &mut rgba8);
                Ok(rgba8)
            }
        }
    }

    fn image_dimensions(&self, asset_type: AssetType) -> (Option<u32>, Option<u32>, Option<u32>) {
        // return dimensions of image without padding
        if !self.is_asset_set(asset_type) {
            return (None, None, None);
        }
        let gpu_fetch = &self.header.asset_packs[asset_type.usize()]
            .texture_header
            .gpu_texture_fetch;
        (gpu_fetch.width(), gpu_fetch.height(), gpu_fetch.depth())
    }

    fn is_asset_set(&self, asset_type: AssetType) -> bool {
        self.header.asset_types_flag & (1 << asset_type.usize()) != 0
    }

    fn padded_image(&self, asset_type: AssetType) -> GenericResult<(u32, u32, Vec<u8>)> {
        // return image width (with padding), image height (with padding), and image data (as rgba8)
        let asset_pack_entry = &self.header.asset_packs[asset_type.usize()];
        let gpu_fetch = &asset_pack_entry.texture_header.gpu_texture_fetch;
        let image_data_length = usize::try_from(asset_pack_entry.image_data_length)?;
        let image_data_index = usize::try_from(asset_pack_entry.image_data_index)?;
        // determine image dimensions with padding
        let (width, height) = self.padded_image_dimensions(asset_type)?;
        // determine which bytes in `image_data` correspond to texture for `asset_type`
        let mut entry_image_data: Vec<u8> = vec![0; image_data_length];
        entry_image_data.copy_from_slice(
            &self.image_data[image_data_index..image_data_index + image_data_length],
        );
        apply_endian(
            &mut entry_image_data,
            asset_pack_entry.texture_header.gpu_texture_fetch.endian()?,
        );
        let mut rgba8 = Self::decompress_image_data(
            entry_image_data,
            gpu_fetch.texture_format()?,
            usize::try_from(width)?,
            usize::try_from(height)?,
        )?;
        apply_swizzle(
            &mut rgba8,
            usize::try_from(gpu_fetch.swizzle_x())?,
            usize::try_from(gpu_fetch.swizzle_y())?,
            usize::try_from(gpu_fetch.swizzle_z())?,
            usize::try_from(gpu_fetch.swizzle_w())?,
        );
        Ok((width, height, rgba8))
    }

    fn padded_image_dimensions(&self, asset_type: AssetType) -> GenericResult<(u32, u32)> {
        // return image dimensions with padding
        let image_data_length = self.header.asset_packs[asset_type.usize()].image_data_length;
        let pitch = self.header.asset_packs[asset_type.usize()]
            .texture_header
            .gpu_texture_fetch
            .pitch();
        let bytes_per_pixel = Self::bytes_per_pixel(
            self.header.asset_packs[asset_type.usize()]
                .texture_header
                .gpu_texture_fetch
                .texture_format()?,
        )?;
        if pitch == 0 {
            return Err("Cannot calculate image dimensions because pitch is 0.".into());
        }
        let width = 32 * pitch;
        let height = image_data_length / (width * bytes_per_pixel);
        Ok((width, height))
    }
}
