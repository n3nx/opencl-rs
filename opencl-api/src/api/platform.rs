/*
 * platform.rs - Platform API wrappers.
 *
 * Copyright 2020-2021 Naman Bishnoi
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![allow(dead_code, unused_assignments, unused_macros)]

use crate::enums::{Size, ParamValue};
use crate::errors::Error;
use crate::helpers::*;
use crate::size_getter;
use crate::structs::{PlatformInfo} ;
use libc::{c_void, size_t};
use opencl_heads::ffi::*;
use std::ptr;
use std::vec;

pub type PlatformsList = Vec<cl_platform_id>;

// fn get_size(num_entries: cl_uint) -> Result<cl_uint, Status> {
//     let mut platform_count: cl_uint = 0;
//     let status_code =
//         unsafe { clGetPlatformIDs(num_entries, ptr::null_mut(), &mut platform_count) };
//     status_update(status_code, platform_count)
// }

fn get_platform_count(num_entries: cl_uint) -> Result<cl_uint, Error> {
    let mut platform_count: cl_uint = 0;
    let status_code =
        unsafe { clGetPlatformIDs(num_entries, ptr::null_mut(), &mut platform_count) };
    status_update(status_code, platform_count)
}

/// Returns the list of all platforms available
///
/// `num_entries` is number of cl_platform_id entries can be added to platforms.
/// if platforms is not null, num_entries must be greater than zero.
///
/// # Examples
/// ```
/// use opencl_api::api::platform::*;
///
/// let id = get_platform_ids();
/// ```
pub fn get_platform_ids() -> APIResult<PlatformsList> {
    let platform_count = get_platform_count(0)?;
    if platform_count == 0 {
        Ok(Vec::default())
    } else {
        let vector_length = platform_count as usize;
        let mut all_platforms: PlatformsList = vec::from_elem(ptr::null_mut(), vector_length);
        let status_code = unsafe {
            clGetPlatformIDs(platform_count, all_platforms.as_mut_ptr(), ptr::null_mut())
        };
        status_update(status_code, all_platforms)
    }
}

pub fn get_platform_info(
    platform: cl_platform_id,
    param_name: cl_platform_info,
) -> APIResult<ParamValue> {
    size_getter!(get_platform_info_size, clGetPlatformInfo);
    match param_name {
        PlatformInfo::PROFILE
        | PlatformInfo::VERSION
        | PlatformInfo::VENDOR
        | PlatformInfo::NAME
        | PlatformInfo::EXTENSIONS => {
            let size = get_platform_info_size(platform, param_name)?;
            if size == 0 {
                Ok(ParamValue::String(bytes_into_string(Vec::default())?))
            } else {
                let bytearr_len = size / Size::u8.get();
                let mut param_value: Vec<u8> = vec::from_elem(0, bytearr_len);
                let status_code = unsafe {
                    clGetPlatformInfo(
                        platform,
                        param_name,
                        size,
                        param_value.as_mut_ptr() as *mut c_void,
                        ptr::null_mut(),
                    )
                };
                status_update(
                    status_code,
                    ParamValue::String(bytes_into_string(param_value)?),
                )
            }
        }
        _ => status_update(666666, ParamValue::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn test_get_platform_ids() {
        let id = get_platform_ids().unwrap();
        assert_ne!(id, vec!(ptr::null_mut()));
        assert_ne!(id, vec!());
    }
    #[test]
    // #[ignore]
    fn test_get_platform_info() {
        let all_platforms = get_platform_ids().unwrap();
        println!("Number of platforms: {}", all_platforms.len());
        let id = all_platforms[0];

        let name = get_platform_info(id, PlatformInfo::VENDOR).unwrap();
        println!("CL_PLATFORM_NAME: {:?}", name);
        assert_ne!(name.unwrap_string().unwrap(), "");
    }
}
