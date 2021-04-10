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

use crate::enums::{ParamValue, Size};
// use crate::errors::Error;
use crate::helpers::*;
use crate::size_getter;
use crate::structs::PlatformInfo;
use libc::{c_void, size_t};
use opencl_heads::ffi::*;
use std::ptr;
use std::vec;

pub type PlatformsList = Vec<cl_platform_id>;

fn get_platform_count(num_entries: cl_uint) -> APIResult<cl_uint> {
    let mut platform_count: cl_uint = 0;
    let status_code =
        unsafe { clGetPlatformIDs(num_entries, ptr::null_mut(), &mut platform_count) };
    status_update(status_code, "clGetPlatformIDs", platform_count)
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
    let fn_name = "clGetPlatformIDs";
    if platform_count == 0 {
        Ok(Vec::default())
    } else {
        let vector_length = platform_count as usize;
        let mut all_platforms: PlatformsList = vec::from_elem(ptr::null_mut(), vector_length);
        let status_code = unsafe {
            clGetPlatformIDs(platform_count, all_platforms.as_mut_ptr(), ptr::null_mut())
        };
        status_update(status_code, fn_name, all_platforms)
    }
}

pub fn get_platform_info(
    platform: cl_platform_id,
    param_name: cl_platform_info,
) -> APIResult<ParamValue> {
    size_getter!(get_platform_info_size, clGetPlatformInfo);
    let fn_name = "clGetPlatformInfo";
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
                    fn_name,
                    ParamValue::String(bytes_into_string(param_value)?),
                )
            }
        }
        // >= CL 2.1
        PlatformInfo::HOST_TIMER_RESOLUTION => {
            let size = Size::u64.get();
            let mut param_value: u64 = 0;
            let status_code = unsafe {
                clGetPlatformInfo(
                    platform,
                    param_name,
                    size,
                    to_mut_ptr(&mut param_value) as *mut c_void,
                    ptr::null_mut(),
                )
            };
            status_update(status_code, fn_name, ParamValue::ULong(param_value))
        }
        // >= CL 3.0
        PlatformInfo::NUMERIC_VERSION => {
            let size = Size::u32.get();
            let mut param_value: u32 = 0;
            let status_code = unsafe {
                clGetPlatformInfo(
                    platform,
                    param_name,
                    size,
                    to_mut_ptr(&mut param_value) as *mut c_void,
                    ptr::null_mut(),
                )
            };
            status_update(status_code, fn_name, ParamValue::UInt(param_value))
        }
        // >= CL 3.0
        PlatformInfo::EXTENSIONS_WITH_VERSION => {
            let size = get_platform_info_size(platform, param_name)?;
            if size == 0 {
                Ok(ParamValue::NameVersion(Vec::default()))
            } else {
                let bytearr_len = size / Size::cl_name_version.get();
                let filler = cl_name_version {
                    version: 0,
                    name: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                };
                let mut param_value: Vec<cl_name_version> = vec::from_elem(filler, bytearr_len);
                let status_code = unsafe {
                    clGetPlatformInfo(
                        platform,
                        param_name,
                        size,
                        param_value.as_mut_ptr() as *mut c_void,
                        ptr::null_mut(),
                    )
                };
                status_update(status_code, fn_name, ParamValue::NameVersion(param_value))
            }
        }
        _ => status_update(666666, fn_name, ParamValue::default()),
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
        let id = all_platforms[0];

        let name = get_platform_info(id, PlatformInfo::NAME).unwrap();
        println!("CL_PLATFORM_NAME: {:?}", name);
        assert_ne!(name.unwrap_string().unwrap(), "");

        let version = get_platform_info(id, PlatformInfo::VERSION).unwrap();
        println!("CL_PLATFORM_VERSION: {:?}", version);
        assert_ne!(version.unwrap_string().unwrap(), "");

        let vendor = get_platform_info(id, PlatformInfo::VENDOR).unwrap();
        println!("CL_PLATFORM_VENDOR: {:?}", vendor);
        assert_ne!(vendor.unwrap_string().unwrap(), "");

        let profile = get_platform_info(id, PlatformInfo::PROFILE).unwrap();
        println!("CL_PLATFORM_PROFILE: {:?}", profile);
        assert_ne!(profile.unwrap_string().unwrap(), "");

        let extensions = get_platform_info(id, PlatformInfo::EXTENSIONS).unwrap();
        println!("CL_PLATFORM_EXTENSIONS: {:?}", extensions);
        assert_ne!(extensions.unwrap_string().unwrap(), "");
    }

    #[test]
    #[cfg(feature = "cl_2_0")]
    fn test_get_platform_info_v2() {
        let all_platforms = get_platform_ids().unwrap();
        let id = all_platforms[0];

        let extversion = get_platform_info(id, PlatformInfo::HOST_TIMER_RESOLUTION).unwrap();
        println!("CL_PLATFORM_HOST_TIMER_RESOLUTION: {:?}", extversion);
        assert_ne!(extversion.unwrap_ulong().unwrap(), 0);
    }

    #[test]
    #[cfg(feature = "cl_3_0")]
    fn test_get_platform_info_v3() {
        let all_platforms = get_platform_ids().unwrap();
        println!("Number of platforms: {}", all_platforms.len());
        let id = all_platforms[0];

        let numver = get_platform_info(id, PlatformInfo::NUMERIC_VERSION).unwrap();
        println!("CL_PLATFORM_NUMERIC_VERSION: {:?}", numver);
        assert_eq!(numver.unwrap_uint().unwrap(), 0);

        let extversion = get_platform_info(id, PlatformInfo::EXTENSIONS_WITH_VERSION).unwrap();
        println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {:?}", extversion);
        assert_ne!(extversion.unwrap_name_version().unwrap().len(), 0);
    }
}
