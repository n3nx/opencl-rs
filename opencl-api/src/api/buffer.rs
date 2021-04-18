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
use crate::helpers::{
    status_update, APIResult, ContextPtr, EventPtr, GetSetGo, LongProperties, MemPtr, QueuePtr,
    WrappedMutablePointer, WrappedPointer,
};
use crate::structs::{BufferCreateType, MapFlags, MemFlags, StatusCode};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::types::*;
use std::ptr;

pub fn create_buffer(
    context: &ContextPtr,
    flags: MemFlags,
    size: size_t,
    host_ptr: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let fn_name = "clCreateBuffer";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem = unsafe {
        ffi::clCreateBuffer(
            context.unwrap(),
            flags.get(),
            size,
            host_ptr.unwrap(),
            &mut status_code,
        )
    };
    eprintln!("POINTER: {:?}", mem);
    status_update(status_code, fn_name, MemPtr::from_ptr(mem, fn_name)?)
}
// #[cfg(feature = "cl_3_0")]
pub fn create_buffer_with_properties(
    context: ContextPtr,
    properties: &LongProperties,
    flags: MemFlags,
    size: size_t,
    host_ptr: WrappedMutablePointer<c_void>,
) -> APIResult<MemPtr> {
    let mut status_code = StatusCode::INVALID_VALUE;
    let fn_name = "clCreateBufferWithProperties";
    let properties = match properties {
        Some(x) => x.as_ptr(),
        None => ptr::null(),
    };
    let mem = unsafe {
        ffi::clCreateBufferWithProperties(
            context.unwrap(),
            properties,
            flags.get(),
            size,
            host_ptr.unwrap(),
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem, fn_name)?)
}

// TODO: buffer_create_info takes cl_buffer_region as input
/***
typedef struct cl_buffer_region {
    size_t    origin;
    size_t    size;
} cl_buffer_region;
 */
pub fn create_sub_buffer(
    buffer: MemPtr,
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
                    buffer.unwrap(),
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
    command_queue: &QueuePtr,
    buffer: &MemPtr,
    blocking_read: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueReadBuffer";
    let mut event = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueReadBuffer(
            command_queue.unwrap(),
            buffer.unwrap(),
            blocking_read,
            offset,
            size,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}
pub fn enqueue_write_buffer(
    command_queue: &QueuePtr,
    buffer: &MemPtr,
    blocking_write: cl_bool,
    offset: size_t,
    size: size_t,
    ptr: WrappedPointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueWriteBuffer";
    let status_code = unsafe {
        ffi::clEnqueueWriteBuffer(
            command_queue.unwrap(),
            buffer.unwrap(),
            blocking_write,
            offset,
            size,
            ptr.unwrap(),
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_read_buffer_rect(
    command_queue: &QueuePtr,
    buffer: &MemPtr,
    blocking_read: cl_bool,
    buffer_origin: WrappedPointer<size_t>,
    host_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    buffer_row_pitch: size_t,
    buffer_slice_pitch: size_t,
    host_row_pitch: size_t,
    host_slice_pitch: size_t,
    ptr: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueReadBufferRect";
    let status_code = unsafe {
        ffi::clEnqueueReadBufferRect(
            command_queue.unwrap(),
            buffer.unwrap(),
            blocking_read,
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
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_write_buffer_rect(
    command_queue: &QueuePtr,
    buffer: &MemPtr,
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
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueWriteBufferRect";
    let status_code = unsafe {
        ffi::clEnqueueWriteBufferRect(
            command_queue.unwrap(),
            buffer.unwrap(),
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
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_copy_buffer(
    command_queue: &QueuePtr,
    src_buffer: &MemPtr,
    dst_buffer: &MemPtr,
    src_offset: size_t,
    dst_offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueCopyBuffer";
    let status_code = unsafe {
        ffi::clEnqueueCopyBuffer(
            command_queue.unwrap(),
            src_buffer.unwrap(),
            dst_buffer.unwrap(),
            src_offset,
            dst_offset,
            size,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_copy_buffer_rect(
    command_queue: &QueuePtr,
    src_buffer: &MemPtr,
    dst_buffer: &MemPtr,
    src_origin: WrappedPointer<size_t>,
    dst_origin: WrappedPointer<size_t>,
    region: WrappedPointer<size_t>,
    src_row_pitch: size_t,
    src_slice_pitch: size_t,
    dst_row_pitch: size_t,
    dst_slice_pitch: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueCopyBufferRect";
    let status_code = unsafe {
        ffi::clEnqueueCopyBufferRect(
            command_queue.unwrap(),
            src_buffer.unwrap(),
            dst_buffer.unwrap(),
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
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_fill_buffer(
    command_queue: &QueuePtr,
    buffer: &MemPtr,
    pattern: WrappedPointer<c_void>,
    pattern_size: size_t,
    offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueFillBuffer";
    let status_code = unsafe {
        ffi::clEnqueueFillBuffer(
            command_queue.unwrap(),
            buffer.unwrap(),
            pattern.unwrap(),
            pattern_size,
            offset,
            size,
            num_events_in_wait_list,
            event_wait_list.unwrap(),
            &mut event,
        )
    };
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

pub fn enqueue_map_buffer(
    command_queue: &QueuePtr,
    buffer: &MemPtr,
    blocking_map: cl_bool,
    map_flags: MapFlags,
    offset: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
    region_ptr: &mut cl_mem,
) -> APIResult<EventPtr> {
    let mut event = ptr::null_mut();
    let fn_name = "clEnqueueMapBuffer";
    let mut status_code = StatusCode::INVALID_VALUE;
    *region_ptr = unsafe {
        ffi::clEnqueueMapBuffer(
            command_queue.unwrap(),
            buffer.unwrap(),
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
    status_update(status_code, fn_name, EventPtr::from_ptr(event, fn_name)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::context::{create_context, release_context};
    use crate::api::device::get_device_ids;
    use crate::api::platform::get_platform_ids;
    use crate::api::queue::{create_command_queue_with_properties, release_command_queue};
    use crate::helpers::{DevicePtr, GetSetGo, PlatformPtr, WrapMutPtr, WrapPtr};
    use crate::structs::{CommandQueueInfo, CommandQueueProperties, ContextProperties, DeviceType};
    use std::ptr;

    #[test]
    #[ignore]
    fn test_create_buffer() {
        let platform_ids = get_platform_ids().unwrap();
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

        // Queue v2
        let properties = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        let queue =
            create_command_queue_with_properties(&context, &device_id, &properties).unwrap();

        // Start buffer test
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap();
        let size = 1048576 * 4; // 4MB memory
        let buffer_mem = create_buffer(&context, flags, size, WrapMutPtr::null()).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem.unwrap(), ptr::null_mut());
        // Start buffer test 2
        //NOTE: This OpenCL API flag is not stable as there is 21.3% chance that this will return a null pointer;
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap()
            + MemFlags::new(MemFlags::USE_HOST_PTR).unwrap();
        let size = 1048576; // 2MB memory
        let buffer_mem = create_buffer(&context, flags, size, buffer_mem).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem.unwrap(), ptr::null_mut());

        release_command_queue(queue).unwrap();
        release_context(context).unwrap();
    }

    #[test]
    // #[ignore]
    fn test_create_sub_buffer() {
        let platform_ids = get_platform_ids().unwrap();
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

        // Queue v2
        let _ = CommandQueueInfo.properties(
            CommandQueueProperties::new(CommandQueueProperties::PROFILING_ENABLE).unwrap()
                + CommandQueueProperties::new(CommandQueueProperties::ON_DEVICE).unwrap(),
        );
        let queue = create_command_queue_with_properties(&context, &device_id, &None).unwrap();

        // Start buffer test
        let flags = MemFlags::new(MemFlags::READ_WRITE).unwrap();
        let size = 1048576 * 4; // 4MB memory
        let buffer_mem = create_buffer(&context, flags.clone(), size, WrapMutPtr::null()).unwrap();
        // eprintln!("{:?}", buffer_mem);
        assert_ne!(buffer_mem.unwrap(), ptr::null_mut());
        // Create sub buffer test
        let creation_info = cl_buffer_region {
            origin: 0,
            size: size / 2,
        };
        let creation_info_ptr = WrapPtr::from(&creation_info);
        let sub_buffer_mem = create_sub_buffer(
            buffer_mem,
            flags,
            BufferCreateType::REGION,
            creation_info_ptr,
        )
        .unwrap();
        let sub_buffer_mem_ptr = sub_buffer_mem.unwrap_mut_cptr().unwrap();

        const ARRAY_SIZE: usize = 1000;
        let ones: [cl_float; ARRAY_SIZE] = [1.0; ARRAY_SIZE];
        let write_event = enqueue_write_buffer(
            &queue,
            &sub_buffer_mem_ptr,
            1,
            0,
            ones.len() * std::mem::size_of::<cl_float>(),
            WrappedPointer::from_owned(ones.as_ptr() as cl_mem),
            0,
            WrappedPointer::null(),
        )
        .unwrap();
        assert_ne!(write_event.unwrap(), ptr::null_mut());

        let read_event = enqueue_read_buffer(
            &queue,
            &sub_buffer_mem_ptr,
            1,
            0,
            ones.len() * std::mem::size_of::<cl_float>(),
            WrapMutPtr::from_owned(ones),
            0,
            WrappedPointer::null(),
        )
        .unwrap();
        assert_ne!(read_event.unwrap(), ptr::null_mut());

        release_command_queue(queue).unwrap();
        release_context(context).unwrap();
    }
}
