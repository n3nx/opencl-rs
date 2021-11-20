/*
 * device.rs - Device API wrappers (Part of OpenCL Platform Layer).
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
use crate::helpers::{
    bytes_into_string, status_update, APIResult, DeviceList, DevicePtr, GetSetGo, PlatformPtr,
    Properties,
};
use crate::structs::{DeviceInfo, DeviceType};
use crate::{gen_object_list, gen_param_value, get_count, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::{clCreateSubDevices, clGetDeviceIDs, clGetDeviceInfo};
use opencl_heads::types::*;
use std::ptr;

pub fn get_device_ids(platform: &PlatformPtr, device_type: DeviceType) -> APIResult<DeviceList> {
    let device_type = device_type.get();
    let platform = platform.unwrap();
    let device_count = get_count!(clGetDeviceIDs, platform, device_type);

    if device_count == 0 {
        Ok(Vec::default())
    } else {
        gen_object_list!(
            clGetDeviceIDs,
            DeviceList,
            device_count,
            platform,
            device_type
        )
    }
}

pub fn get_device_info(device: &DevicePtr, param_name: cl_device_info) -> APIResult<ParamValue> {
    type D = DeviceInfo;
    let fn_name = "clGetDeviceInfo";
    let device = device.unwrap();
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
            let param_value = gen_param_value!(clGetDeviceInfo, u8, device, param_name, size);
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
            let param_value = gen_param_value!(clGetDeviceInfo, u32, device, param_name);
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
            let param_value = gen_param_value!(clGetDeviceInfo, u64, device, param_name);
            Ok(ParamValue::ULong(param_value))
        }
        D::ILS_WITH_VERSION
        | D::BUILT_IN_KERNELS_WITH_VERSION
        | D::OPENCL_C_ALL_VERSIONS
        | D::OPENCL_C_FEATURES
        | D::EXTENSIONS_WITH_VERSION => {
            let size = get_device_info_size(device, param_name)?;
            let param_value =
                gen_param_value!(clGetDeviceInfo, cl_name_version, device, param_name, size);
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
            let param_value = gen_param_value!(clGetDeviceInfo, usize, device, param_name);
            Ok(ParamValue::CSize(param_value))
        }
        D::PLATFORM | D::PARENT_DEVICE => {
            let param_value = gen_param_value!(clGetDeviceInfo, isize, device, param_name);
            Ok(ParamValue::CPtr(param_value))
        }
        D::PARTITION_PROPERTIES | D::PARTITION_TYPE => {
            let size = get_device_info_size(device, param_name)?;
            let param_value = gen_param_value!(clGetDeviceInfo, isize, device, param_name, size);
            Ok(ParamValue::ArrCPtr(param_value))
        }
        D::MAX_WORK_ITEM_SIZES => {
            let size = get_device_info_size(device, param_name)?;
            let param_value = gen_param_value!(clGetDeviceInfo, usize, device, param_name, size);
            Ok(ParamValue::ArrCSize(param_value))
        }
        _ => status_update(40404, fn_name, ParamValue::default()),
    }
}

pub fn get_device_and_host_timer(device: &DevicePtr) -> APIResult<(cl_ulong, cl_ulong)> {
    let fn_name = "clGetDeviceAndHostTimer";
    let mut device_timestamp = cl_ulong::default();
    let mut host_timestamp = cl_ulong::default();
    let status_code = unsafe {
        ffi::clGetDeviceAndHostTimer(device.unwrap(), &mut device_timestamp, &mut host_timestamp)
    };
    status_update(status_code, fn_name, (device_timestamp, host_timestamp))
}
/// `clGetHostTimer` returns the current value of the host clock as seen by device.
/// This value is in the same timebase as the host_timestamp returned from `clGetDeviceAndHostTimer`.
/// The implementation will return with as low a latency as possible to allow a correlation
/// with a subsequent application sampled time. The host timestamp and device timestamp returned
/// by this function and `clGetDeviceAndHostTimer` each have an implementation defined timebase.
/// The timestamps will always be in their respective timebases regardless of which query function
/// is used. The timestamp returned from `clGetEventProfilingInfo` for an event on a device and a
/// device timestamp queried from the same device will always be in the same timebase.
///
/// `clGetHostTimer` will return CL_SUCCESS with a time value in host_timestamp if provided. Otherwise, it returns one of the following errors:
///
/// - CL_INVALID_DEVICE if device is not a valid device.
///
/// - CL_INVALID_OPERATION if the platform associated with device does not support device and host timer synchronization.
///
/// - CL_INVALID_VALUE if host_timestamp is NULL.
///
/// - CL_OUT_OF_RESOURCES if there is a failure to allocate resources required by the OpenCL implementation on the device.
///
/// - CL_OUT_OF_HOST_MEMORY if there is a failure to allocate resources required by the OpenCL implementation on the host.
pub fn get_host_timer(device: &DevicePtr) -> APIResult<cl_ulong> {
    let fn_name = "clGetHostTimer";
    let mut host_timestamp = cl_ulong::default();
    let status_code = unsafe { ffi::clGetHostTimer(device.unwrap(), &mut host_timestamp) };
    status_update(status_code, fn_name, host_timestamp)
}

/// Partitioning a device
pub fn create_sub_devices(in_device: &DevicePtr, properties: &Properties) -> APIResult<DeviceList> {
    let in_device = in_device.unwrap();
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let device_partition_count = get_count!(clCreateSubDevices, in_device, properties);
    let device_partition_count = device_partition_count * 8;
    gen_object_list!(
        clCreateSubDevices,
        DeviceList,
        device_partition_count,
        in_device,
        properties
    )
}

pub fn retain_device(device: &DevicePtr) -> APIResult<()> {
    let fn_name = "clRetainDevice";
    let status_code = unsafe { ffi::clRetainDevice(device.unwrap()) };
    status_update(status_code, fn_name, ())
}

pub fn release_device(device: DevicePtr) -> APIResult<()> {
    let fn_name = "clReleaseDevice";
    let status_code = unsafe { ffi::clReleaseDevice(device.unwrap()) };
    status_update(status_code, fn_name, ())
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
    use crate::api::platform;
    use crate::helpers::PlatformPtr;
    use crate::structs::{DevicePartitionProperty, PlatformInfo};
    #[test]
    fn test_get_device_info() {
        let all_platforms = platform::get_platform_ids().unwrap();
        assert_ne!(all_platforms.len(), 0);
        // let id = all_platforms[0];
        let id = PlatformPtr::from_ptr(all_platforms[0], "main_fn").unwrap();

        let name = platform::get_platform_info(&id, PlatformInfo::NAME).unwrap();
        let platform_name = name.unwrap_string().unwrap();
        assert_ne!(platform_name, "");
        println!("CL_PLATFORM_NAME: {}", platform_name);

        let device =
            DeviceType::new(DeviceType::CPU + DeviceType::GPU + DeviceType::DEFAULT).unwrap();

        // device.set(DeviceType::GPU).unwrap();
        let device_ids = get_device_ids(&id, device).unwrap();
        assert!(0 < device_ids.len());
        // Choose the first GPU device
        let device_id = DevicePtr::from_ptr(device_ids[0], "test_fn").unwrap();
        let vendor_name = get_device_info(&device_id, DeviceInfo::VENDOR).unwrap();
        let vendor_name = vendor_name.unwrap_string().unwrap();
        println!("CL_DEVICE_VENDOR_NAME: {}", vendor_name);
        assert_ne!(vendor_name, "");
        let vendor_id = get_device_info(&device_id, DeviceInfo::VENDOR_ID).unwrap();
        let vendor_id = vendor_id.unwrap_uint().unwrap();
        println!("CL_DEVICE_VENDOR_ID: {:X}", vendor_id);
        assert_ne!(vendor_id, 0);
    }

    #[test]
    #[ignore]
    fn test_create_sub_device() {
        let all_platforms = platform::get_platform_ids().unwrap();
        assert_ne!(all_platforms.len(), 0);
        let id = PlatformPtr::from_ptr(all_platforms[0], "main_fn").unwrap();

        let name = platform::get_platform_info(&id, PlatformInfo::NAME).unwrap();
        let platform_name = name.unwrap_string().unwrap();
        assert_ne!(platform_name, "");
        println!("CL_PLATFORM_NAME: {}", platform_name);

        let device =
            DeviceType::new(DeviceType::CPU + DeviceType::GPU + DeviceType::DEFAULT).unwrap();

        let device_ids = get_device_ids(&id, device).unwrap();
        assert!(0 < device_ids.len());
        // Choose the first GPU device
        // let device_id = device_ids[0];
        let device_id = DevicePtr::from_ptr(device_ids[0], "test_fn").unwrap();
        let vendor_name =
            get_device_info(&device_id, DeviceInfo::PARTITION_MAX_SUB_DEVICES).unwrap();
        let max_sub_devices = vendor_name.unwrap_uint().unwrap();
        println!("CL_DEVICE_COMPUTE_UNITS: {}", max_sub_devices);

        if max_sub_devices > 1 {
            let sub_cu_count = max_sub_devices / 4;
            let properties = DevicePartitionProperty.equally(sub_cu_count);

            let sub_device_list = match create_sub_devices(&device_id, &properties) {
                Ok(x) => x,
                Err(x) => {
                    eprintln!("{}", x);
                    Vec::new()
                }
            };
            println!("CL_DEVICE_LIST: {:?}", sub_device_list);
            assert!(0 < sub_device_list.len());
        }
    }
}
