/*
 * buffer.rs - Buffer API wrappers (Part of OpenCL Runtime Layer).
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
//!
//! An image object is used to store a one-, two- or three-dimensional texture, frame-buffer or image.
//! The elements of an image object are selected from a list of predefined image formats.
//! The minimum number of elements in a memory object is one.

use crate::enums::{ParamValue, Size};
use crate::helpers::{
    status_update, APIResult, ContextPtr, EventPtr, GetSetGo, MemFormatList, MemPtr, QueuePtr,
    WrappedMutablePointer, WrappedPointer,
};
use crate::structs::{MemFlags, StatusCode};
use crate::{gen_object_list, gen_param_value, get_count, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetSupportedImageFormats;
use opencl_heads::types::*;
use std::ptr;

pub fn create_image(
    context: &ContextPtr,
    flags: MemFlags,
    image_format: WrappedPointer<cl_image_format>,
    image_desc: WrappedPointer<cl_image_desc>,
    host: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let fn_name = "clCreateImage";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem_ptr = unsafe {
        ffi::clCreateImage(
            context.unwrap(),
            flags.get(),
            image_format.unwrap(),
            image_desc.unwrap(),
            host.unwrap(),
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem_ptr, fn_name)?)
}

pub fn create_image_with_properties(
    context: &ContextPtr,
    properties: WrappedPointer<cl_mem_properties>,
    flags: MemFlags,
    image_format: WrappedPointer<cl_image_format>,
    image_desc: WrappedPointer<cl_image_desc>,
    host_ptr: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let fn_name = "clCreateImageWithProperties";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem_ptr = unsafe {
        ffi::clCreateImageWithProperties(
            context.unwrap(),
            properties.unwrap(),
            flags.get(),
            image_format.unwrap(),
            image_desc.unwrap(),
            host_ptr.unwrap(),
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem_ptr, fn_name)?)
}

pub fn create_image_2d(
    context: &ContextPtr,
    flags: MemFlags,
    image_format: WrappedPointer<cl_image_format>,
    image_width: size_t,
    image_height: size_t,
    image_row_pitch: size_t,
    host_ptr: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let fn_name = "clCreateImage2D";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem_ptr = unsafe {
        ffi::clCreateImage2D(
            context.unwrap(),
            flags.get(),
            image_format.unwrap(),
            image_width,
            image_height,
            image_row_pitch,
            host_ptr.unwrap(),
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem_ptr, fn_name)?)
}

pub fn create_image_3d(
    context: &ContextPtr,
    flags: MemFlags,
    image_format: WrappedPointer<cl_image_format>,
    image_width: size_t,
    image_height: size_t,
    image_depth: size_t,
    image_row_pitch: size_t,
    image_slice_pitch: size_t,
    host_ptr: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let fn_name = "clCreateImage3D";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem_ptr = unsafe {
        ffi::clCreateImage3D(
            context.unwrap(),
            image_format.unwrap(),
            image_width,
            image_height,
            image_depth,
            image_row_pitch,
            image_slice_pitch,
            host_ptr.unwrap(),
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem_ptr, fn_name)?)
}

// TODO: Check buffer types for api compatiblity

pub fn get_supported_image_formats(
    context: &ContextPtr,
    flags: MemFlags,
    image_type: cl_mem_object_type,
) -> APIResult<MemFormatList> {
    let flags = flags.get();
    let context = context.unwrap();
    let image_format_count = get_count!(clGetSupportedImageFormats, context, flags, image_type);

    gen_object_list!(
        clGetSupportedImageFormats,
        cl_image_format,
        image_format_count,
        context,
        flags,
        image_type
    )
}

// TODO: Check supported image struct's return types

pub fn enqueue_read_image(
    command_queue: &QueuePtr,
    image: &MemPtr,
    blocking_read: cl_bool,
    origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    row_pitch: size_t,
    slice_pitch: size_t,
    ptr: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueReadImage";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueReadImage(
            command_queue.unwrap(),
            image.unwrap(),
            blocking_read,
            origin.unwrap(),
            region.unwrap(),
            row_pitch,
            slice_pitch,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}
pub fn enqueue_write_image(
    command_queue: &QueuePtr,
    image: &MemPtr,
    blocking_write: cl_bool,
    origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    row_pitch: size_t,
    slice_pitch: size_t,
    ptr: WrappedPointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueReadImage";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueWriteImage(
            command_queue.unwrap(),
            image.unwrap(),
            blocking_write,
            origin.unwrap(),
            region.unwrap(),
            row_pitch,
            slice_pitch,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}

pub fn enqueue_copy_image(
    command_queue: &QueuePtr,
    src_image: &MemPtr,
    dst_image: &MemPtr,
    src_origin: WrappedPointer<size_t>,
    dst_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueCopyImage";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueCopyImage(
            command_queue.unwrap(),
            src_image.unwrap(),
            dst_image.unwrap(),
            src_origin.unwrap(),
            dst_origin.unwrap(),
            region.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}

pub fn enqueue_fill_image(
    command_queue: &QueuePtr,
    image: &MemPtr,
    fill_color: WrappedPointer<c_void>,
    origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueFillImage";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueFillImage(
            command_queue.unwrap(),
            image.unwrap(),
            fill_color.unwrap(),
            origin.unwrap(),
            region.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}

pub fn enqueue_copy_image_to_buffer(
    command_queue: &QueuePtr,
    src_image: &MemPtr,
    dst_buffer: &MemPtr,
    src_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    dst_offset: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueCopyImageToBuffer";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueCopyImageToBuffer(
            command_queue.unwrap(),
            src_image.unwrap(),
            dst_buffer.unwrap(),
            src_origin.unwrap(),
            region.unwrap(),
            dst_offset,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}

pub fn enqueue_copy_buffer_to_image(
    command_queue: QueuePtr,
    src_buffer: MemPtr,
    dst_image: MemPtr,
    src_offset: size_t,
    dst_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueCopyBufferToImage";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueCopyBufferToImage(
            command_queue.unwrap(),
            src_buffer.unwrap(),
            dst_image.unwrap(),
            src_offset,
            dst_origin.unwrap(),
            region.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event_ptr,
        )
    };
    status_update(
        status_code,
        fn_name,
        EventPtr::from_ptr(event_ptr, fn_name)?,
    )
}
