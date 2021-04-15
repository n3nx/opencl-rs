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
use crate::helpers::{status_update, APIResult, GetSetGo, QueueProperties};
use crate::structs::{CommandQueueProperties, StatusCode};
use opencl_heads::ffi;
use opencl_heads::types::*;
use std::ptr;

pub fn create_command_queue(
    context: cl_context,
    device: cl_device_id,
    properties: CommandQueueProperties,
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

pub fn retain_command_queue(command_queue: cl_command_queue) -> APIResult<()> {
    let fn_name = "clRetainCommandQueue";
    let status_code = unsafe { ffi::clRetainCommandQueue(command_queue) };
    status_update(status_code, fn_name, ())
}

pub fn release_command_queue(command_queue: cl_command_queue) -> APIResult<()> {
    let fn_name = "clReleaseCommandQueue";
    let status_code = unsafe { ffi::clReleaseCommandQueue(command_queue) };
    status_update(status_code, fn_name, ())
}

pub fn flush(command_queue: cl_command_queue) -> APIResult<()> {
    let fn_name = "clFlush";
    let status_code = unsafe { ffi::clFlush(command_queue) };
    status_update(status_code, fn_name, ())
}

pub fn finish(command_queue: cl_command_queue) -> APIResult<()> {
    let fn_name = "clFinish";
    let status_code = unsafe { ffi::clFinish(command_queue) };
    status_update(status_code, fn_name, ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::structs::{CommandQueueInfo, CommandQueueProperties, ContextProperties, DeviceType};
    use std::ptr;
    // use crate::error_codes::error_text;

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
        let queue = create_command_queue(context, device_id, properties).unwrap();

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
}
