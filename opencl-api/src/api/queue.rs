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
use crate::helpers::{
    status_update, APIResult, ContextPtr, DevicePtr, GetSetGo, LongProperties, QueuePtr,
};
use crate::structs::{CommandQueueInfo, CommandQueueProperties, StatusCode};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetCommandQueueInfo;
use opencl_heads::types::*;
use std::ptr;

// #[cfg(feature = "depr_2_0")]
pub fn create_command_queue(
    context: &ContextPtr,
    device: &DevicePtr,
    properties: &CommandQueueProperties,
) -> APIResult<QueuePtr> {
    let fn_name = "clCreateCommandQueue";
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    let queue_ptr = unsafe {
        ffi::clCreateCommandQueue(
            context.unwrap(),
            device.unwrap(),
            properties.get(),
            &mut status_code,
        )
    };
    status_update(
        status_code,
        fn_name,
        QueuePtr::from_ptr(queue_ptr, fn_name)?,
    )
}

pub fn create_command_queue_with_properties(
    context: &ContextPtr,
    device: &DevicePtr,
    properties: &LongProperties,
) -> APIResult<QueuePtr> {
    let fn_name = "clCreateCommandQueueWithProperties";
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let queue_ptr = unsafe {
        ffi::clCreateCommandQueueWithProperties(
            context.unwrap(),
            device.unwrap(),
            properties,
            &mut status_code,
        )
    };
    status_update(
        status_code,
        fn_name,
        QueuePtr::from_ptr(queue_ptr, fn_name)?,
    )
}

pub fn set_default_device_command_queue(
    context: &ContextPtr,
    device: &DevicePtr,
    command_queue: &QueuePtr,
) -> APIResult<()> {
    let status_code = unsafe {
        ffi::clSetDefaultDeviceCommandQueue(
            context.unwrap(),
            device.unwrap(),
            command_queue.unwrap(),
        )
    };
    status_update(status_code, "clSetDefaultDeviceCommandQueue", ())
}

pub fn retain_command_queue(command_queue: &QueuePtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clRetainCommandQueue(command_queue.unwrap()) };
    status_update(status_code, "clRetainCommandQueue", ())
}

pub fn release_command_queue(command_queue: QueuePtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clReleaseCommandQueue(command_queue.unwrap()) };
    status_update(status_code, "clReleaseCommandQueue", ())
}

pub fn get_command_queue_info(
    command_queue: &QueuePtr,
    param_name: cl_command_queue_info,
) -> APIResult<ParamValue> {
    let command_queue = command_queue.unwrap();
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
            let param_value =
                gen_param_value!(clGetCommandQueueInfo, u64, command_queue, param_name, size);
            Ok(ParamValue::ArrULong(param_value))
        }
        _ => status_update(40404, "clGetContextInfo", ParamValue::default()),
    }
}

#[cfg(feature = "depr_1_0")]
pub fn set_command_queue_property(
    command_queue: &QueuePtr,
    properties: &CommandQueueProperties,
    enable: cl_bool,
) -> APIResult<()> {
    let status_code = unsafe {
        ffi::clSetCommandQueueProperty(command_queue.unwrap(), properties, enable, ptr::null_mut())
    };
    status_update(status_code, "clSetCommandQueueProperty", ())
}

pub fn flush(command_queue: &QueuePtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clFlush(command_queue.unwrap()) };
    status_update(status_code, "clFlush", ())
}

pub fn finish(command_queue: &QueuePtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clFinish(command_queue.unwrap()) };
    status_update(status_code, "clFinish", ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::helpers::{DevicePtr, GetSetGo, PlatformPtr, WrapMutPtr};
    use crate::structs::{CommandQueueInfo, CommandQueueProperties, ContextProperties, DeviceType};

    #[test]
    // #[ignore]
    fn test_command_queue() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        let device_ids =
            get_device_ids(&platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        // eprintln!("{:?}", device_ids);

        // let device_id = device_ids[0];
        let device_id = DevicePtr::from_ptr(device_ids[0], "test_fn").unwrap();

        let properties = ContextProperties.platform(&platform_id);

        let context = create_context(&properties, device_ids, None, WrapMutPtr::null());
        let context = context.unwrap();

        // Queue v1
        let properties = CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE)
            .unwrap()
            + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE_DEFAULT).unwrap();
        let queue = create_command_queue(&context, &device_id, &properties).unwrap();

        release_command_queue(queue).unwrap();

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE_DEFAULT).unwrap(),
        );
        eprintln!("{:?}", properties);
        let queue = create_command_queue_with_properties(&context, &device_id, &None).unwrap();

        release_command_queue(queue).unwrap();

        release_context(context).unwrap();
    }

    #[test]
    // #[ignore]
    fn test_get_command_queue_info() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        // let platform_id = platform_ids[0];
        let platform_id = PlatformPtr::from_ptr(platform_ids[0], "test_fn").unwrap();

        let device_ids =
            get_device_ids(&platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());

        // let device_id = device_ids[0];
        let device_id = DevicePtr::from_ptr(device_ids[0], "test_fn").unwrap();

        let properties = ContextProperties.platform(&platform_id);

        let context = create_context(&properties, device_ids, None, WrapMutPtr::null());
        let context = context.unwrap();

        // Queue v1
        let properties = CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE)
            .unwrap()
            + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap();
        let queue = create_command_queue(&context, &device_id, &properties).unwrap();
        // Get command queue info v1
        let command_info = get_command_queue_info(&queue, CommandQueueInfo::PROPERTIES).unwrap();
        assert_eq!(properties.get(), command_info.unwrap_ulong().unwrap());

        release_command_queue(queue).unwrap();

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        eprintln!("{:?}", properties);
        let queue =
            create_command_queue_with_properties(&context, &device_id, &properties).unwrap();

        let command_info = get_command_queue_info(&queue, CommandQueueInfo::PROPERTIES).unwrap();
        assert_eq!(properties.unwrap()[1], command_info.unwrap_ulong().unwrap());

        release_command_queue(queue).unwrap();

        release_context(context).unwrap();
    }
}
