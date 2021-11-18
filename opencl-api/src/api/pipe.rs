/*
 * pipe.rs - Pipe API wrappers (Part of OpenCL Runtime Layer).
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
//! A pipe is a memory object that stores data organized as a FIFO.
//!
//! Pipe objects can only be accessed using built-in functions that read from and write to a pipe.
//! Pipe objects are not accessible from the host. A pipe object encapsulates the following information:
//! - Packet size in bytes
//! - Maximum capacity in packets
//! - Information about the number of packets currently in the pipe
//! - Data packets
//!
use crate::helpers::{status_update, APIResult, ContextPtr, GetSetGo, MemPtr};
use crate::structs::{MemFlags, PipeInfo, StatusCode};
use opencl_heads::ffi;
use opencl_heads::types::*;

use crate::enums::{ParamValue, Size};
use crate::{gen_param_value, size_getter};
use libc::c_void;
use opencl_heads::ffi::clGetPipeInfo;
use std::ptr;

pub fn create_pipe(
    context: &ContextPtr,
    flags: MemFlags,
    pipe_packet_size: cl_uint,
    pipe_max_packets: cl_uint,
    properties: *const cl_pipe_properties,
) -> APIResult<MemPtr> {
    let fn_name = "clCreatePipe";
    let mut status_code = StatusCode::INVALID_VALUE;
    let mem_ptr = unsafe {
        ffi::clCreatePipe(
            context.unwrap(),
            flags.get(),
            pipe_packet_size,
            pipe_max_packets,
            properties,
            &mut status_code,
        )
    };
    status_update(status_code, fn_name, MemPtr::from_ptr(mem_ptr, fn_name)?)
}

pub fn get_pipe_info(pipe: &MemPtr, param_name: cl_pipe_info) -> APIResult<ParamValue> {
    let pipe = pipe.unwrap();
    size_getter!(get_pipe_info_size, clGetPipeInfo);
    match param_name {
        PipeInfo::PACKET_SIZE | PipeInfo::MAX_PACKETS => {
            let param_value = gen_param_value!(clGetPipeInfo, u32, pipe, param_name);
            Ok(ParamValue::UInt(param_value))
        }
        PipeInfo::PROPERTIES => {
            let size = get_pipe_info_size(pipe, param_name)?;
            let param_value = gen_param_value!(clGetPipeInfo, isize, pipe, param_name, size);
            Ok(ParamValue::ArrCPtr(param_value))
        }
        _ => status_update(40404, "clGetPipeInfo", ParamValue::default()),
    }
}

// TODO: Add unit tests for this file.
