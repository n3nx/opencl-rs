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
use libc::{c_void, size_t};
use opencl_heads::ffi;
use opencl_heads::types::*;
use std::ptr;

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

pub fn enqueue_read_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    offset: usize,
    size: usize,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> APIResult<cl_event> {
    let fn_name = "clEnqueueReadBuffer";
    let mut event_ptr: cl_event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueReadBuffer(
            command_queue,
            buffer,
            blocking_read,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list,
            &mut event_ptr,
        )
    };
    status_update(status_code, fn_name, event_ptr)
}
