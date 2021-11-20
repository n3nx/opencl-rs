/*
 * platform.rs - Platform API wrappers (Part of OpenCL Platform Layer).
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
//! The platform layer allows the host program to discover OpenCL devices
//! and their capabilities and to create contexts.
//!
use crate::objects::enums::{ParamValue, Size};
use crate::objects::functions::{bytes_into_string, status_update};
use crate::objects::structs::PlatformInfo;
use crate::objects::types::{APIResult, PlatformList, PlatformPtr};
use crate::{gen_object_list, gen_param_value, get_count, size_getter};
use libc::c_void;
use opencl_heads::ffi::*;
use std::ptr;

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
pub fn get_platform_ids() -> APIResult<PlatformList> {
    let platform_count = get_count!(clGetPlatformIDs);

    if platform_count == 0 {
        Ok(Vec::default())
    } else {
        gen_object_list!(clGetPlatformIDs, PlatformList, platform_count)
    }
}

pub fn get_platform_info(
    platform: &PlatformPtr,
    param_name: cl_platform_info,
) -> APIResult<ParamValue> {
    type P = PlatformInfo;
    let platform = platform.unwrap();
    size_getter!(get_platform_info_size, clGetPlatformInfo);
    let fn_name = "clGetPlatformInfo";
    match param_name {
        P::PROFILE | P::VERSION | P::VENDOR | P::NAME | P::EXTENSIONS => {
            let size = get_platform_info_size(platform, param_name)?;
            let param_value = gen_param_value!(clGetPlatformInfo, u8, platform, param_name, size);
            Ok(ParamValue::String(bytes_into_string(param_value)?))
        }
        // >= CL 2.1
        P::HOST_TIMER_RESOLUTION => {
            let param_value = gen_param_value!(clGetPlatformInfo, u64, platform, param_name);
            Ok(ParamValue::ULong(param_value))
        }
        // >= CL 3.0
        P::NUMERIC_VERSION => {
            let param_value = gen_param_value!(clGetPlatformInfo, u32, platform, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        // >= CL 3.0
        P::EXTENSIONS_WITH_VERSION => {
            let size = get_platform_info_size(platform, param_name)?;
            let param_value = gen_param_value!(
                clGetPlatformInfo,
                cl_name_version,
                platform,
                param_name,
                size
            );
            Ok(ParamValue::NameVersion(param_value))
        }
        _ => status_update(40404, fn_name, ParamValue::default()),
    }
}

/************************/
/* /\ /\ /\ /\ /\ /\ /\ */
/*|__|__|__|__|__|__|__|*/
/*|  |  |  |  |  |  |  |*/
/*|  |  Unit Tests  |  |*/
/*|__|__|__|__|__|__|__|*/
/*|__|__|__|__|__|__|__|*/
/************************/

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
        let id = PlatformPtr::from_ptr(all_platforms[0], "main_fn").unwrap();

        let name = get_platform_info(&id, PlatformInfo::NAME).unwrap();
        println!("CL_PLATFORM_NAME: {:?}", name);
        assert_ne!(name.unwrap_string().unwrap(), "");

        let version = get_platform_info(&id, PlatformInfo::VERSION).unwrap();
        println!("CL_PLATFORM_VERSION: {:?}", version);
        assert_ne!(version.unwrap_string().unwrap(), "");

        let vendor = get_platform_info(&id, PlatformInfo::VENDOR).unwrap();
        println!("CL_PLATFORM_VENDOR: {:?}", vendor);
        assert_ne!(vendor.unwrap_string().unwrap(), "");

        let profile = get_platform_info(&id, PlatformInfo::PROFILE).unwrap();
        println!("CL_PLATFORM_PROFILE: {:?}", profile);
        assert_ne!(profile.unwrap_string().unwrap(), "");

        let extensions = get_platform_info(&id, PlatformInfo::EXTENSIONS).unwrap();
        println!("CL_PLATFORM_EXTENSIONS: {:?}", extensions);
        assert_ne!(extensions.unwrap_string().unwrap(), "");
    }

    #[test]
    #[ignore]
    fn test_get_platform_info_fail() {
        let all_platforms = get_platform_ids().unwrap();
        // let id = all_platforms[0];
        let id = PlatformPtr::from_ptr(all_platforms[0], "main_fn").unwrap();
        let wrong_id = 666;

        let extensions = get_platform_info(&id, wrong_id)
            .expect_err("Wrong ID entered, check your structs dude!");
        println!("CL_PLATFORM_INVALID: {:?}", extensions);
        // Bullshit! I will do this later on.
        // assert_ne!(extensions, );
    }

    #[test]
    #[cfg(feature = "cl_2_0")]
    fn test_get_platform_info_v2() {
        let all_platforms = get_platform_ids().unwrap();
        // let id = all_platforms[0];
        //TODO: Add auto-detection of platforms
        let id = PlatformPtr::from_ptr(all_platforms[0], "main_fn").unwrap();

        let extversion = get_platform_info(&id, PlatformInfo::HOST_TIMER_RESOLUTION).unwrap();
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
