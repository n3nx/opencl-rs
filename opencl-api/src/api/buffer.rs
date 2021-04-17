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
//! A buffer object stores a one-dimensional collection of elements.
//! Elements of a buffer object can be a scalar data type (such as an int, float), vector data type, or a user-defined structure.
//!
use crate::enums::ParamValue;
use crate::helpers::{status_update, APIResult, GetSetGo, LongProperties, WrappedPointer};
use crate::structs::{BufferCreateType, MapFlags, MemFlags, StatusCode};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::types::*;
use std::ptr;

pub fn create_buffer(
    context: cl_context,
    flags: MemFlags,
    size: size_t,
    host_ptr: *mut c_void,
) -> APIResult<cl_mem> {
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem =
        unsafe { ffi::clCreateBuffer(context, flags.get(), size, host_ptr, &mut status_code) };
    status_update(status_code, "clCreateBuffer", mem)
}
// #[cfg(feature = "cl_3_0")]
pub fn create_buffer_with_properties(
    context: cl_context,
    properties: &LongProperties,
    flags: MemFlags,
    size: size_t,
    host_ptr: *mut c_void,
) -> APIResult<cl_mem> {
    let mut status_code = StatusCode::INVALID_VALUE;
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let mem = unsafe {
        ffi::clCreateBufferWithProperties(
            context,
            properties,
            flags.get(),
            size,
            host_ptr,
            &mut status_code,
        )
    };
    status_update(status_code, "clCreateBufferWithProperties", mem)
}

// TODO: buffer_create_info takes cl_buffer_region as input
/***
typedef struct cl_buffer_region {
    size_t    origin;
    size_t    size;
} cl_buffer_region;
 */
pub fn create_sub_buffer(
    buffer: cl_mem,
    flags: MemFlags,
    buffer_create_type: cl_buffer_create_type,
    buffer_create_info: WrappedPointer<c_void>,
) -> APIResult<ParamValue> {
    let fn_name = "clCreateSubBuffer";
    match buffer_create_type {
        BufferCreateType::REGION => {
            let mut status_code = StatusCode::INVALID_VALUE;
            let mem = unsafe {
                ffi::clCreateSubBuffer(
                    buffer,
                    flags.get(),
                    buffer_create_type,
                    buffer_create_info.unwrap(),
                    &mut status_code,
                )
            };
            let value = status_update(status_code, fn_name, mem)?;
            Ok(ParamValue::CPtr(value as isize))
        }
        _ => status_update(40404, fn_name, ParamValue::default()),
    }
}

// Readingm, writing and copying buffer objects.
pub fn enqueue_read_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueReadBuffer(
            command_queue,
            buffer,
            blocking_read,
            offset,
            size,
            ptr,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueReadBuffer", event)
}
pub fn enqueue_write_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_write: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: WrappedPointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueWriteBuffer(
            command_queue,
            buffer,
            blocking_write,
            offset,
            size,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueWriteBuffer", event)
}

pub fn enqueue_read_buffer_rect(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_read: cl_bool,
    buffer_origin: WrappedPointer<size_t>,
    host_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    buffer_row_pitch: size_t,
    buffer_slice_pitch: size_t,
    host_row_pitch: size_t,
    host_slice_pitch: size_t,
    ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueReadBufferRect(
            command_queue,
            buffer,
            blocking_read,
            buffer_origin.unwrap(),
            host_origin.unwrap(),
            region.unwrap(),
            buffer_row_pitch,
            buffer_slice_pitch,
            host_row_pitch,
            host_slice_pitch,
            ptr,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueReadBufferRect", event)
}

pub fn enqueue_write_buffer_rect(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_write: cl_bool,
    buffer_origin: WrappedPointer<size_t>,
    host_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    buffer_row_pitch: size_t,
    buffer_slice_pitch: size_t,
    host_row_pitch: size_t,
    host_slice_pitch: size_t,
    ptr: WrappedPointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueWriteBufferRect(
            command_queue,
            buffer,
            blocking_write,
            buffer_origin.unwrap(),
            host_origin.unwrap(),
            region.unwrap(),
            buffer_row_pitch,
            buffer_slice_pitch,
            host_row_pitch,
            host_slice_pitch,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueWriteBufferRect", event)
}

pub fn enqueue_copy_buffer(
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_offset: size_t,
    dst_offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueCopyBuffer(
            command_queue,
            src_buffer,
            dst_buffer,
            src_offset,
            dst_offset,
            size,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueCopyBuffer", event)
}

pub fn enqueue_copy_buffer_rect(
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_origin: WrappedPointer<size_t>,
    dst_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    src_row_pitch: size_t,
    src_slice_pitch: size_t,
    dst_row_pitch: size_t,
    dst_slice_pitch: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueCopyBufferRect(
            command_queue,
            src_buffer,
            dst_buffer,
            src_origin.unwrap(),
            dst_origin.unwrap(),
            region.unwrap(),
            src_row_pitch,
            src_slice_pitch,
            dst_row_pitch,
            dst_slice_pitch,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueCopyBufferRect", event)
}

pub fn enqueue_fill_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    pattern: WrappedPointer<c_void>,
    pattern_size: size_t,
    offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueFillBuffer(
            command_queue,
            buffer,
            pattern.unwrap(),
            pattern_size,
            offset,
            size,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, "clEnqueueFillBuffer", event)
}

pub fn enqueue_map_buffer(
    command_queue: cl_command_queue,
    buffer: cl_mem,
    blocking_map: cl_bool,
    map_flags: MapFlags,
    offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
    region_ptr: &mut cl_mem,
) -> APIResult<cl_event> {
    let mut event = ptr::null_mut();
    let mut status_code = StatusCode::INVALID_VALUE;
    *region_ptr = unsafe {
        ffi::clEnqueueMapBuffer(
            command_queue,
            buffer,
            blocking_map,
            map_flags.get(),
            offset,
            size,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
            &mut status_code,
        )
    };
    status_update(status_code, "clEnqueueMapBuffer", event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::api::queue::{create_command_queue_with_properties, release_command_queue};
    use crate::helpers::{GetSetGo, WrappedPointer};
    use crate::structs::{CommandQueueInfo, CommandQueueProperties, ContextProperties, DeviceType};
    use std::ptr;

    #[test]
    // #[ignore]
    fn test_create_buffer() {
        let platform_ids = get_platform_ids().unwrap();
        let platform_id = platform_ids[0];
        let device_ids =
            get_device_ids(platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());
        let device_id = device_ids[0];

        let properties = ContextProperties.platform(platform_id);
        let context = create_context(&properties, device_ids, None, ptr::null_mut());
        let context = context.unwrap();

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        let queue = create_command_queue_with_properties(context, device_id, &properties).unwrap();

        // Start buffer test
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap();
        let size = 1048576 * 4; // 4MB memory
        let buffer_mem = create_buffer(context, flags, size, ptr::null_mut()).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem, ptr::null_mut());
        // Start buffer test 2
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap()
            + MemFlags::new(MemFlags::USE_HOST_PTR).unwrap();
        let size = 1048576 * 2; // 2MB memory
        let buffer_mem = create_buffer(context, flags, size, buffer_mem).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem, ptr::null_mut());

        release_command_queue(queue).unwrap();
        release_context(context).unwrap();
    }

    #[test]
    fn test_create_sub_buffer() {
        let platform_ids = get_platform_ids().unwrap();
        let platform_id = platform_ids[0];
        let device_ids =
            get_device_ids(platform_id, DeviceType::new(DeviceType::DEFAULT).unwrap()).unwrap();
        assert!(0 < device_ids.len());
        let device_id = device_ids[0];

        let properties = ContextProperties.platform(platform_id);
        let context = create_context(&properties, device_ids, None, ptr::null_mut());
        let context = context.unwrap();

        // Queue v2
        let _ = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        let queue = create_command_queue_with_properties(context, device_id, &None).unwrap();

        // Start buffer test
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap();
        let size = 1048576 * 4; // 4MB memory
        let buffer_mem = create_buffer(context, flags.clone(), size, ptr::null_mut()).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem, ptr::null_mut());
        // Create sub buffer test
        let creation_info = cl_buffer_region {
            origin: 0,
            size: size / 2,
        };
        let creation_info_ptr = WrappedPointer::from(&creation_info);
        let sub_buffer_mem = create_sub_buffer(
            buffer_mem,
            flags,
            BufferCreateType::REGION,
            creation_info_ptr,
        ).unwrap();
        let sub_buffer_mem_ptr = sub_buffer_mem.unwrap_cptr().unwrap() as cl_mem;

        const ARRAY_SIZE: usize = 1000;
        let ones: [cl_float; ARRAY_SIZE] = [1.0; ARRAY_SIZE];
        let write_event = enqueue_write_buffer(
            queue,
            sub_buffer_mem_ptr,
            1,
            0,
            ones.len() * std::mem::size_of::<cl_float>(),
            WrappedPointer::from_owned(ones.as_ptr() as cl_mem),
            0,
            WrappedPointer::null(),
        );
        assert_ne!(write_event.unwrap(), ptr::null_mut());

        let read_event = enqueue_read_buffer(
            queue,
            sub_buffer_mem_ptr,
            1,
            0,
            ones.len() * std::mem::size_of::<cl_float>(),
            ones.as_ptr() as cl_mem,
            0,
            WrappedPointer::null(),
        );
        assert_ne!(read_event.unwrap(), ptr::null_mut());

        release_command_queue(queue).unwrap();
        release_context(context).unwrap();
    }
}
