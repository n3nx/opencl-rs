/*
 * image.rs - Image API wrappers (Part of OpenCL Runtime Layer).
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
//! Shared virtual memory (a.k.a. SVM) allows the host and kernels executing on devices to directly share complex, pointer-containing data structures such as trees and linked lists.
//! It also eliminates the need to marshal data between the host and devices.
//! As a result, SVM substantially simplifies OpenCL programming and may improve performance.
//!
use crate::enums::{ParamValue, Size};
use crate::helpers::{
    status_update, APIResult, ContextPtr, EventPtr, GetSetGo, MemPtr, QueuePtr, SVMPtr,
    WrappedMutablePointer, WrappedPointer,
};
use crate::structs::{MapFlags, MemFlags, MemInfo, MemMigrationFlags, StatusCode};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi;
use opencl_heads::ffi::clGetMemObjectInfo;
use opencl_heads::types::*;
use std::ptr;

pub fn retain_mem_object(memobj: &MemPtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clRetainMemObject(memobj.unwrap()) };
    status_update(status_code, "clRetainMemObject", ())
}

pub fn release_mem_object(memobj: MemPtr) -> APIResult<()> {
    let status_code = unsafe { ffi::clReleaseMemObject(memobj.unwrap()) };
    status_update(status_code, "clReleaseMemObject", ())
}

pub fn set_mem_object_destructor_callback(
    memobj: &MemPtr,
    pfn_notify: Option<extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
    user_data: WrappedMutablePointer<c_void>,
) -> APIResult<()> {
    let status_code = unsafe {
        ffi::clSetMemObjectDestructorCallback(memobj.unwrap(), pfn_notify, user_data.unwrap())
    };
    status_update(status_code, "clSetMemObjectDestructorCallback", ())
}

pub fn enqueue_unmap_mem_object(
    command_queue: &QueuePtr,
    memobj: &MemPtr,
    mapped_ptr: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueUnmapMemObject";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueUnmapMemObject(
            command_queue.unwrap(),
            memobj.unwrap(),
            mapped_ptr.unwrap(),
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

pub fn enqueue_migrate_mem_objects(
    command_queue: &QueuePtr,
    num_mem_objects: cl_uint,
    mem_objects: WrappedPointer<cl_mem>,
    flags: MemMigrationFlags,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueMigrateMemObjects";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueMigrateMemObjects(
            command_queue.unwrap(),
            num_mem_objects,
            mem_objects.unwrap(),
            flags.get(),
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

pub fn get_mem_object_info(memobj: MemPtr, param_name: cl_mem_info) -> APIResult<ParamValue> {
    let memobj = memobj.unwrap();
    size_getter!(get_mem_object_info_size, clGetMemObjectInfo);
    match param_name {
        MemInfo::MAP_COUNT
        | MemInfo::REFERENCE_COUNT
        | MemInfo::USES_SVM_POINTER
        | MemInfo::TYPE => {
            let param_value = gen_param_value!(clGetMemObjectInfo, u32, memobj, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        MemInfo::HOST_PTR | MemInfo::CONTEXT | MemInfo::ASSOCIATED_MEMOBJECT => {
            let param_value = gen_param_value!(clGetMemObjectInfo, isize, memobj, param_name);
            Ok(ParamValue::CPtr(param_value))
        }
        MemInfo::FLAGS => {
            let param_value = gen_param_value!(clGetMemObjectInfo, u64, memobj, param_name);
            Ok(ParamValue::ULong(param_value))
        }
        MemInfo::SIZE | MemInfo::OFFSET => {
            let param_value = gen_param_value!(clGetMemObjectInfo, usize, memobj, param_name);
            Ok(ParamValue::CSize(param_value))
        }
        MemInfo::PROPERTIES => {
            let size = get_mem_object_info_size(memobj, param_name)?;
            let param_value = gen_param_value!(clGetMemObjectInfo, u64, memobj, param_name, size);
            Ok(ParamValue::ArrULong(param_value))
        }
        _ => status_update(40404, "clGetMemObjectInfo", ParamValue::default()),
    }
}

pub fn svm_alloc(
    context: &ContextPtr,
    flags: &MemFlags,
    size: size_t,
    alignment: cl_uint,
) -> APIResult<SVMPtr> {
    let fn_name = "clSVMAlloc";
    let mem_ptr = unsafe { ffi::clSVMAlloc(context.unwrap(), flags.get(), size, alignment) };
    status_update(
        StatusCode::SUCCESS,
        fn_name,
        SVMPtr::from_ptr(mem_ptr, fn_name)?,
    )
}

pub fn svm_free(context: &ContextPtr, svm_pointer: WrappedMutablePointer<c_void>) {
    unsafe { ffi::clSVMFree(context.unwrap(), svm_pointer.unwrap()) };
}

pub fn enqueue_svm_free(
    command_queue: &QueuePtr,
    num_svm_pointers: cl_uint,
    svm_pointers: WrappedPointer<*const c_void>,
    pfn_free_func: Option<
        extern "C" fn(
            queue: cl_command_queue,
            num_svm_pointers: cl_uint,
            svm_pointes: *const *const c_void,
            user_data: *mut c_void,
        ),
    >,
    user_data: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMFree";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMFree(
            command_queue.unwrap(),
            num_svm_pointers,
            svm_pointers.unwrap(),
            pfn_free_func,
            user_data.unwrap(),
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

pub fn enqueue_svm_memcpy(
    command_queue: QueuePtr,
    blocking_copy: cl_bool,
    dst_ptr: WrappedMutablePointer<c_void>,
    src_ptr: WrappedPointer<c_void>,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMMemcpy";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMMemcpy(
            command_queue.unwrap(),
            blocking_copy,
            dst_ptr.unwrap(),
            src_ptr.unwrap(),
            size,
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

pub fn enqueue_svm_memfill(
    command_queue: &QueuePtr,
    svm_ptr: WrappedMutablePointer<c_void>,
    pattern: WrappedPointer<c_void>,
    pattern_size: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMMemfill";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMMemFill(
            command_queue.unwrap(),
            svm_ptr.unwrap(),
            pattern.unwrap(),
            pattern_size,
            size,
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

pub fn enqueue_svm_map(
    command_queue: &QueuePtr,
    blocking_map: cl_bool,
    flags: MapFlags,
    svm_ptr: WrappedMutablePointer<c_void>,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMMap";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMMap(
            command_queue.unwrap(),
            blocking_map,
            flags.get(),
            svm_ptr.unwrap(),
            size,
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

pub fn enqueue_svm_unmap(
    command_queue: &QueuePtr,
    svm_ptr: WrappedMutablePointer<c_void>,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMUnmap";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMUnmap(
            command_queue.unwrap(),
            svm_ptr.unwrap(),
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

pub fn enqueue_svm_migrate_mem(
    command_queue: QueuePtr,
    num_svm_pointers: cl_uint,
    svm_pointers: WrappedPointer<*const c_void>,
    sizes: WrappedPointer<size_t>,
    flags: MemMigrationFlags,
    num_events_in_wait_list: cl_uint,
    event_wait_list: WrappedPointer<cl_event>,
) -> APIResult<EventPtr> {
    let fn_name = "clEnqueueSVMMigrateMem";
    let mut event_ptr = ptr::null_mut();
    let status_code = unsafe {
        ffi::clEnqueueSVMMigrateMem(
            command_queue.unwrap(),
            num_svm_pointers,
            svm_pointers.unwrap(),
            sizes.unwrap(),
            flags.get(),
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
