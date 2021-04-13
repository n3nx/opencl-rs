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
use crate::enums::{ParamValue, Size};
use crate::errors::{ToLibraryError, ValidationError};
use crate::helpers::*;
use crate::structs::{DeviceInfo, DeviceType};
use crate::{gen_object_list, size_getter};
use libc::c_void;
use opencl_heads::ffi::*;
use std::ptr;
use std::vec;

fn get_device_count(
    platform: cl_platform_id,
    device_type: cl_device_type,
    num_entries: cl_uint,
) -> APIResult<cl_uint> {
    let fn_name = "clGetDeviceIDs";
    let mut device_count: cl_uint = 0;
    let status_code = unsafe {
        clGetDeviceIDs(
            platform,
            device_type,
            num_entries,
            ptr::null_mut(),
            &mut device_count,
        )
    };
    status_update(status_code, fn_name, device_count)
}

pub fn get_device_ids(platform: cl_platform_id, device_type: DeviceType) -> APIResult<DeviceList> {
    let fn_name = "clGetDeviceIDs";
    let device_type = match device_type.get() {
        Some(x) => x,
        None => return Err(ValidationError::InvalidBitfield(fn_name).to_error()),
    };
    let device_count = get_device_count(platform, device_type, 0)?;
    if device_count == 0 {
        Ok(Vec::default())
    } else {
        let vector_length = device_count as usize;
        let mut all_devices: DeviceList = vec::from_elem(ptr::null_mut(), vector_length);
        let status_code = unsafe {
            clGetDeviceIDs(
                platform,
                device_type,
                device_count,
                all_devices.as_mut_ptr(),
                ptr::null_mut(),
            )
        };
        status_update(status_code, fn_name, all_devices)
    }
}

pub fn get_device_info(device: cl_device_id, param_name: cl_device_info) -> APIResult<ParamValue> {
    type D = DeviceInfo;
    let fn_name = "clGetDeviceInfo";
    size_getter!(get_device_info_size, clGetDeviceInfo);
    match param_name {
        D::NAME
        | D::VENDOR
        | D::VERSION
        | D::PROFILE
        | D::DRIVER_VERSION
        | D::EXTENSIONS
        | D::OPENCL_C_VERSION
        | D::BUILT_IN_KERNELS
        | D::IL_VERSION
        | D::LATEST_CONFORMANCE_VERSION_PASSED => {
            let size = get_device_info_size(device, param_name)?;
            gen_object_list!(get_device_info_string, clGetPlatformInfo, u8);
            let param_value = get_device_info_string(device, param_name, size, 0)?;
            Ok(ParamValue::String(bytes_into_string(param_value)?))
        }
        _ => status_update(666666, fn_name, ParamValue::default()),
    }
}
