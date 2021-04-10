/*
 * queue.rs - Command Queue APIs.
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
use crate::errors::{ToLibraryError, ValidationError};
use crate::helpers::{status_update, APIResult};
use crate::structs::{CommandQueueProperties, StatusCode};
use opencl_heads::ffi;
use opencl_heads::types::*;
// use std::ptr;

pub fn create_command_queue(
    context: cl_context,
    device: cl_device_id,
    properties: CommandQueueProperties,
) -> APIResult<cl_command_queue> {
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    let properties = match properties.get() {
        Some(x) => x,
        None => return Err(ValidationError::InvalidBitfield("clCreateCommandQueue").to_error()),
    };
    let queue_ptr =
        unsafe { ffi::clCreateCommandQueue(context, device, properties, &mut status_code) };
    status_update(status_code, "clCreateCommandQueue", queue_ptr)
}

pub fn create_command_queue_with_properties(
    context: cl_context,
    device: cl_device_id,
    properties: CommandQueueProperties,
) -> APIResult<cl_command_queue> {
    let mut status_code: cl_int = StatusCode::INVALID_COMMAND_QUEUE;
    let properties: *const cl_queue_properties = match properties.get() {
        Some(x) => &x,
        None => {
            return Err(
                ValidationError::InvalidBitfield("clCreateCommandQueueWithProperties").to_error(),
            )
        }
    };
    let queue_ptr = unsafe {
        ffi::clCreateCommandQueueWithProperties(context, device, properties, &mut status_code)
    };
    status_update(status_code, "clCreateCommandQueueWithProperties", queue_ptr)
}

// pub fn retain_command_queue(command_queue: cl_command_queue)
