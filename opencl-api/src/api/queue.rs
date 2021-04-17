/*
 * queue.rs - Command Queue APIs (Part of OpenCL Runtime Layer).
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
//! Considering OpenCL 1.x as base.

// use crate::enums::Status;
use crate::enums::{ParamValue, Size};
use crate::helpers::{status_update, APIResult, GetSetGo, QueueProperties};
use crate::structs::{CommandQueueInfo, CommandQueueProperties, StatusCode};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetCommandQueueInfo;
use opencl_heads::types::*;
use std::ptr;
use std::vec;

// #[cfg(feature = "depr_2_0")]
pub fn create_command_queue(
    context: cl_context,
    device: cl_device_id,
    properties: &CommandQueueProperties,
) -> APIResult<cl_command_queue> {
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    let properties = properties.get();
    let queue_ptr =
        unsafe { ffi::clCreateCommandQueue(context, device, properties, &mut status_code) };
    status_update(status_code, "clCreateCommandQueue", queue_ptr)
}

pub fn create_command_queue_with_properties(
    context: cl_context,
    device: cl_device_id,
    properties: &QueueProperties,
) -> APIResult<cl_command_queue> {
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    // let properties: *const cl_queue_properties = &properties.get();
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let queue_ptr = unsafe {
        ffi::clCreateCommandQueueWithProperties(context, device, properties, &mut status_code)
    };
    status_update(status_code, "clCreateCommandQueueWithProperties", queue_ptr)
}

pub fn set_default_device_command_queue(
    context: cl_context,
    device: cl_device_id,
    command_queue: cl_command_queue,
) -> APIResult<()> {
    let status_code =
        unsafe { ffi::clSetDefaultDeviceCommandQueue(context, device, command_queue) };
    status_update(status_code, "clSetDefaultDeviceCommandQueue", ())
}

pub fn retain_command_queue(command_queue: cl_command_queue) -> APIResult<()> {
    let status_code = unsafe { ffi::clRetainCommandQueue(command_queue) };
    status_update(status_code, "clRetainCommandQueue", ())
}

pub fn release_command_queue(command_queue: cl_command_queue) -> APIResult<()> {
    let status_code = unsafe { ffi::clReleaseCommandQueue(command_queue) };
    status_update(status_code, "clReleaseCommandQueue", ())
}

pub fn get_command_queue_info(
    command_queue: cl_command_queue,
    param_name: cl_command_queue_info,
) -> APIResult<ParamValue> {
    size_getter!(get_command_queue_info_size, clGetCommandQueueInfo);
    match param_name {
        CommandQueueInfo::CONTEXT | CommandQueueInfo::DEVICE | CommandQueueInfo::DEVICE_DEFAULT => {
            let param_value =
                gen_param_value!(clGetCommandQueueInfo, isize, command_queue, param_name);
            Ok(ParamValue::CPtr(param_value))
        }
        CommandQueueInfo::REFERENCE_COUNT | CommandQueueInfo::SIZE => {
            let param_value =
                gen_param_value!(clGetCommandQueueInfo, u32, command_queue, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        CommandQueueInfo::PROPERTIES => {
            let param_value =
                gen_param_value!(clGetCommandQueueInfo, u64, command_queue, param_name);
            Ok(ParamValue::ULong(param_value))
        }
        // #[cfg(feature = "cl_3_0")]
        CommandQueueInfo::PROPERTIES_ARRAY => {
            let size = get_command_queue_info_size(command_queue, param_name)?;
            let filler = 0;
            let param_value = gen_param_value!(
                clGetCommandQueueInfo,
                u64,
                command_queue,
                param_name,
                size,
                filler
            );
            Ok(ParamValue::ArrULong(param_value))
        }
        _ => status_update(40404, "clGetContextInfo", ParamValue::default()),
    }
}

#[cfg(feature = "depr_1_0")]
pub fn set_command_queue_property(
    command_queue: cl_command_queue,
    properties: CommandQueueProperties,
    enable: cl_bool,
) -> APIResult<()> {
    let status_code = unsafe {
        ffi::clSetCommandQueueProperty(command_queue, properties, enable, ptr::null_mut())
    };
    status_update(status_code, "clSetCommandQueueProperty", ())
}

pub fn flush(command_queue: cl_command_queue) -> APIResult<()> {
    let status_code = unsafe { ffi::clFlush(command_queue) };
    status_update(status_code, "clFlush", ())
}

pub fn finish(command_queue: cl_command_queue) -> APIResult<()> {
    let status_code = unsafe { ffi::clFinish(command_queue) };
    status_update(status_code, "clFinish", ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::helpers::GetSetGo;
    use crate::structs::{CommandQueueInfo, CommandQueueProperties, ContextProperties, DeviceType};
    use std::ptr;

    #[test]
    // #[ignore]
    fn test_command_queue() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids =
            get_device_ids(platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        // eprintln!("{:?}", device_ids);

        let device_id = device_ids[0];

        let properties = ContextProperties.platform(platform_id);

        let context = create_context(&properties, device_ids, None, ptr::null_mut());
        let context = context.unwrap();

        // Queue v1
        let properties = CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE)
            .unwrap()
            + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE_DEFAULT).unwrap();
        let queue = create_command_queue(context, device_id, &properties).unwrap();

        release_command_queue(queue).unwrap();

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE_DEFAULT).unwrap(),
        );
        eprintln!("{:?}", properties);
        let queue = create_command_queue_with_properties(context, device_id, &None).unwrap();

        release_command_queue(queue).unwrap();

        release_context(context).unwrap();
    }

    #[test]
    // #[ignore]
    fn test_get_command_queue_info() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids =
            get_device_ids(platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let properties = ContextProperties.platform(platform_id);

        let context = create_context(&properties, device_ids, None, ptr::null_mut());
        let context = context.unwrap();

        // Queue v1
        let properties = CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE)
            .unwrap()
            + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap();
        let queue = create_command_queue(context, device_id, &properties).unwrap();
        // Get command queue info v1
        let command_info = get_command_queue_info(queue, CommandQueueInfo::PROPERTIES).unwrap();
        assert_eq!(properties.get(), command_info.unwrap_ulong().unwrap());

        release_command_queue(queue).unwrap();

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        eprintln!("{:?}", properties);
        let queue = create_command_queue_with_properties(context, device_id, &properties).unwrap();

        let command_info = get_command_queue_info(queue, CommandQueueInfo::PROPERTIES).unwrap();
        assert_eq!(properties.unwrap()[1], command_info.unwrap_ulong().unwrap());

        release_command_queue(queue).unwrap();

        release_context(context).unwrap();
    }
}
