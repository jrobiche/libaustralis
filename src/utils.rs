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

pub fn create_parent_directories(file_path: &Path) -> GenericResult<()> {
    match Path::new(file_path).parent() {
        Some(parent_path) => match std::fs::create_dir_all(parent_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        },
        None => Ok(()),
    }
}
