/*
 * device.rs - Device API wrappers.
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
use crate::helpers::{bytes_into_string, status_update, APIResult, DeviceList, GetSetGo};
use crate::structs::{DeviceInfo, DeviceType};
use crate::{gen_object_elem, gen_object_list, size_getter};
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
    // let device_type = match device_type.get() {
    //     Some(x) => x,
    //     None => return Err(ValidationError::InvalidBitfield(fn_name).to_error()),
    // };
    let device_type = device_type.get();
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
            gen_object_list!(get_device_info_string, clGetDeviceInfo, u8);
            let param_value = get_device_info_string(device, param_name, size, 0)?;
            Ok(ParamValue::String(bytes_into_string(param_value)?))
        }
        D::VENDOR_ID
        | D::MAX_COMPUTE_UNITS
        | D::MAX_WORK_ITEM_DIMENSIONS
        | D::PREFERRED_VECTOR_WIDTH_CHAR
        | D::PREFERRED_VECTOR_WIDTH_SHORT
        | D::PREFERRED_VECTOR_WIDTH_INT
        | D::PREFERRED_VECTOR_WIDTH_LONG
        | D::PREFERRED_VECTOR_WIDTH_FLOAT
        | D::PREFERRED_VECTOR_WIDTH_DOUBLE
        | D::PREFERRED_VECTOR_WIDTH_HALF
        | D::NATIVE_VECTOR_WIDTH_CHAR
        | D::NATIVE_VECTOR_WIDTH_SHORT
        | D::NATIVE_VECTOR_WIDTH_INT
        | D::NATIVE_VECTOR_WIDTH_LONG
        | D::NATIVE_VECTOR_WIDTH_FLOAT
        | D::NATIVE_VECTOR_WIDTH_DOUBLE
        | D::NATIVE_VECTOR_WIDTH_HALF
        | D::MAX_CLOCK_FREQUENCY
        | D::ADDRESS_BITS
        | D::MAX_READ_IMAGE_ARGS
        | D::MAX_WRITE_IMAGE_ARGS
        | D::MAX_READ_WRITE_IMAGE_ARGS
        | D::MAX_SAMPLERS
        | D::IMAGE_PITCH_ALIGNMENT
        | D::IMAGE_BASE_ADDRESS_ALIGNMENT
        | D::MAX_PIPE_ARGS
        | D::PIPE_MAX_ACTIVE_RESERVATIONS
        | D::PIPE_MAX_PACKET_SIZE
        | D::MEM_BASE_ADDR_ALIGN
        | D::MIN_DATA_TYPE_ALIGN_SIZE
        | D::GLOBAL_MEM_CACHELINE_SIZE
        | D::MAX_CONSTANT_ARGS
        | D::QUEUE_ON_DEVICE_PREFERRED_SIZE
        | D::QUEUE_ON_DEVICE_MAX_SIZE
        | D::MAX_ON_DEVICE_QUEUES
        | D::MAX_ON_DEVICE_EVENTS
        | D::PARTITION_MAX_SUB_DEVICES
        | D::REFERENCE_COUNT
        | D::PREFERRED_PLATFORM_ATOMIC_ALIGNMENT
        | D::PREFERRED_GLOBAL_ATOMIC_ALIGNMENT
        | D::PREFERRED_LOCAL_ATOMIC_ALIGNMENT
        | D::MAX_NUM_SUB_GROUPS
        | D::IMAGE_SUPPORT
        | D::ERROR_CORRECTION_SUPPORT
        | D::HOST_UNIFIED_MEMORY
        | D::ENDIAN_LITTLE
        | D::AVAILABLE
        | D::COMPILER_AVAILABLE
        | D::LINKER_AVAILABLE
        | D::PREFERRED_INTEROP_USER_SYNC
        | D::SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS
        | D::NON_UNIFORM_WORK_GROUP_SUPPORT
        | D::WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT
        | D::GENERIC_ADDRESS_SPACE_SUPPORT
        | D::PIPE_SUPPORT
        | D::NUMERIC_VERSION
        | D::GLOBAL_MEM_CACHE_TYPE
        | D::LOCAL_MEM_TYPE => {
            gen_object_elem!(get_device_info_uint, clGetDeviceInfo, u32);
            let param_value = get_device_info_uint(device, param_name)?;
            Ok(ParamValue::UInt(param_value))
        }
        D::MAX_MEM_ALLOC_SIZE
        | D::GLOBAL_MEM_CACHE_SIZE
        | D::GLOBAL_MEM_SIZE
        | D::MAX_CONSTANT_BUFFER_SIZE
        | D::LOCAL_MEM_SIZE
        | D::TYPE
        | D::SINGLE_FP_CONFIG
        | D::DOUBLE_FP_CONFIG
        | D::EXECUTION_CAPABILITIES
        | D::QUEUE_ON_HOST_PROPERTIES
        | D::QUEUE_ON_DEVICE_PROPERTIES
        | D::PARTITION_AFFINITY_DOMAIN
        | D::SVM_CAPABILITIES
        | D::ATOMIC_MEMORY_CAPABILITIES
        | D::ATOMIC_FENCE_CAPABILITIES
        | D::DEVICE_ENQUEUE_CAPABILITIES => {
            gen_object_elem!(get_device_info_ulong, clGetDeviceInfo, u64);
            let param_value = get_device_info_ulong(device, param_name)?;
            Ok(ParamValue::ULong(param_value))
        }
        D::ILS_WITH_VERSION
        | D::BUILT_IN_KERNELS_WITH_VERSION
        | D::OPENCL_C_ALL_VERSIONS
        | D::OPENCL_C_FEATURES
        | D::EXTENSIONS_WITH_VERSION => {
            let size = get_device_info_size(device, param_name)?;
            let filler = cl_name_version {
                version: 0,
                name: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            };
            gen_object_list!(
                get_device_info_name_version,
                clGetDeviceInfo,
                cl_name_version
            );
            let param_value = get_device_info_name_version(device, param_name, size, filler)?;
            Ok(ParamValue::NameVersion(param_value))
        }
        D::MAX_WORK_GROUP_SIZE
        | D::IMAGE2D_MAX_WIDTH
        | D::IMAGE2D_MAX_HEIGHT
        | D::IMAGE3D_MAX_WIDTH
        | D::IMAGE3D_MAX_HEIGHT
        | D::IMAGE3D_MAX_DEPTH
        | D::IMAGE_MAX_BUFFER_SIZE
        | D::IMAGE_MAX_ARRAY_SIZE
        | D::MAX_PARAMETER_SIZE
        | D::MAX_GLOBAL_VARIABLE_SIZE
        | D::GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE
        | D::PROFILING_TIMER_RESOLUTION
        | D::PRINTF_BUFFER_SIZE
        | D::PREFERRED_WORK_GROUP_SIZE_MULTIPLE => {
            gen_object_elem!(get_device_info_usize, clGetDeviceInfo, usize);
            let param_value = get_device_info_usize(device, param_name)?;
            Ok(ParamValue::CSize(param_value))
        }
        D::PLATFORM | D::PARENT_DEVICE => {
            gen_object_elem!(get_device_info_ptr, clGetDeviceInfo, isize);
            let param_value = get_device_info_ptr(device, param_name)?;
            Ok(ParamValue::CPtr(param_value))
        }
        D::PARTITION_PROPERTIES | D::PARTITION_TYPE => {
            let size = get_device_info_size(device, param_name)?;
            gen_object_list!(get_device_info_vec_cptr, clGetDeviceInfo, isize);
            let param_value = get_device_info_vec_cptr(device, param_name, size, 0)?;
            Ok(ParamValue::ArrCPtr(param_value))
        }
        D::MAX_WORK_ITEM_SIZES => {
            let size = get_device_info_size(device, param_name)?;
            gen_object_list!(get_device_info_vec_csize, clGetDeviceInfo, usize);
            let param_value = get_device_info_vec_csize(device, param_name, size, 0)?;
            Ok(ParamValue::ArrCSize(param_value))
        }
        _ => status_update(40404, fn_name, ParamValue::default()),
    }
}



/*****************************************************************************
 * 
 * 
 *                                  Tests
 * 
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::platform;
    use crate::structs::PlatformInfo;
    #[test]
    fn test_get_device_info() {
        let all_platforms = platform::get_platform_ids().unwrap();
        assert_ne!(all_platforms.len(), 0);
        let id = all_platforms[0];

        let name = platform::get_platform_info(id, PlatformInfo::NAME).unwrap();
        let platform_name = name.unwrap_string().unwrap();
        assert_ne!(platform_name, "");
        println!("CL_PLATFORM_NAME: {}", platform_name);

        let mut device =
            DeviceType::new(DeviceType::CPU).unwrap() + DeviceType::new(DeviceType::GPU).unwrap();

        device.set(DeviceType::GPU).unwrap();
        let device_ids = get_device_ids(id, device).unwrap();
        assert!(0 < device_ids.len());
        // Choose the first GPU device
        let device_id = device_ids[0];
        let vendor_name = get_device_info(device_id, DeviceInfo::VENDOR).unwrap();
        let vendor_name = vendor_name.unwrap_string().unwrap();
        println!("CL_DEVICE_VENDOR_NAME: {}", vendor_name);
        assert_ne!(vendor_name, "");
        let vendor_id = get_device_info(device_id, DeviceInfo::VENDOR_ID).unwrap();
        let vendor_id = vendor_id.unwrap_uint().unwrap();
        println!("CL_DEVICE_VENDOR_ID: {:X}", vendor_id);
        assert_ne!(vendor_id, 0);
    }
}
