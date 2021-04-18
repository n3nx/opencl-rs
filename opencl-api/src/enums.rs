/*
 * enums.rs - Enumerators containing OpenCL Constants.
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

#![allow(dead_code)]
use crate::errors::{OpenCLAPILibraryError, ToLibraryError, ValidationError};
use crate::helpers::{StatusCodeResult, WrappedMutablePointer, WrappedPointer};
use crate::structs::StatusCode;
use opencl_heads::consts::*;
use opencl_heads::types::*;

// Status Codes
#[repr(i32)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Success,
    DeviceNotFound,
    DeviceNotAvailable,
    CompilerNotAvailable,
    MemObjectAllocationFailure,
    OutOfResources,
    OutOfHostMemory,
    ProfilingInfoNotAvailable,
    MemCopyOverlap,
    ImageFormatMismatch,
    ImageFormatNotSupported,
    BuildProgramFailure,
    MapFailure,
    MisalignedSubBufferOffset,
    ExecStatusErrorForEventsInWaitList,
    CompileProgramFailure,
    LinkerNotAvailable,
    LinkProgramFailure,
    DevicePartitionFailed,
    KernelArgInfoNotAvailable,
    InvalidValue,
    InvalidDeviceType,
    InvalidPlatform,
    InvalidDevice,
    InvalidContext,
    InvalidQueueProperties,
    InvalidCommandQueue,
    InvalidHostPtr,
    InvalidMemObject,
    InvalidImageFormatDescriptor,
    InvalidImageSize,
    InvalidSampler,
    InvalidBinary,
    InvalidBuildOptions,
    InvalidProgram,
    InvalidProgramExecutable,
    InvalidKernelName,
    InvalidKernelDefinition,
    InvalidKernel,
    InvalidArgIndex,
    InvalidArgValue,
    InvalidArgSize,
    InvalidKernelArgs,
    InvalidWorkDimension,
    InvalidWorkGroupSize,
    InvalidWorkItemSize,
    InvalidGlobalOffset,
    InvalidEventWaitList,
    InvalidEvent,
    InvalidOperation,
    InvalidGLObject,
    InvalidBufferSize,
    InvalidMIPLevel,
    InvalidGlobalWorkSize,
    InvalidProperty,
    InvalidImageDescriptor,
    InvalidCompilerOptions,
    InvalidLinkerOptions,
    InvalidDevicePartitionCount,
    InvalidPipeSize,
    InvalidDeviceQueue,
    InvalidSpecId,
    MaxSizeRestrictionExceeded,
    InvalidStatusCode(&'static str),
}

impl Status {
    pub fn to_status_code(&self) -> StatusCodeResult {
        let data = match self {
            Status::Success => StatusCode::SUCCESS,
            Status::DeviceNotFound => StatusCode::DEVICE_NOT_FOUND,
            Status::DeviceNotAvailable => StatusCode::DEVICE_NOT_AVAILABLE,
            Status::CompilerNotAvailable => StatusCode::COMPILER_NOT_AVAILABLE,
            Status::MemObjectAllocationFailure => StatusCode::MEM_OBJECT_ALLOCATION_FAILURE,
            Status::OutOfResources => StatusCode::MEM_OBJECT_ALLOCATION_FAILURE,
            Status::OutOfHostMemory => StatusCode::OUT_OF_HOST_MEMORY,
            Status::ProfilingInfoNotAvailable => StatusCode::PROFILING_INFO_NOT_AVAILABLE,
            Status::MemCopyOverlap => StatusCode::MEM_COPY_OVERLAP,
            Status::ImageFormatMismatch => StatusCode::IMAGE_FORMAT_MISMATCH,
            Status::ImageFormatNotSupported => StatusCode::IMAGE_FORMAT_NOT_SUPPORTED,
            Status::BuildProgramFailure => StatusCode::BUILD_PROGRAM_FAILURE,
            Status::MapFailure => StatusCode::MAP_FAILURE,
            Status::MisalignedSubBufferOffset => StatusCode::MISALIGNED_SUB_BUFFER_OFFSET,
            Status::ExecStatusErrorForEventsInWaitList => {
                CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST
            }
            Status::CompileProgramFailure => StatusCode::COMPILE_PROGRAM_FAILURE,
            Status::LinkerNotAvailable => StatusCode::LINKER_NOT_AVAILABLE,
            Status::LinkProgramFailure => StatusCode::LINK_PROGRAM_FAILURE,
            Status::DevicePartitionFailed => StatusCode::DEVICE_PARTITION_FAILED,
            Status::KernelArgInfoNotAvailable => StatusCode::KERNEL_ARG_INFO_NOT_AVAILABLE,
            Status::InvalidValue => StatusCode::INVALID_VALUE,
            Status::InvalidDeviceType => StatusCode::INVALID_DEVICE_TYPE,
            Status::InvalidPlatform => StatusCode::INVALID_PLATFORM,
            Status::InvalidDevice => StatusCode::INVALID_DEVICE,
            Status::InvalidContext => StatusCode::INVALID_CONTEXT,
            Status::InvalidQueueProperties => StatusCode::INVALID_QUEUE_PROPERTIES,
            Status::InvalidCommandQueue => StatusCode::INVALID_COMMAND_QUEUE,
            Status::InvalidHostPtr => StatusCode::INVALID_HOST_PTR,
            Status::InvalidMemObject => StatusCode::INVALID_MEM_OBJECT,
            Status::InvalidImageFormatDescriptor => StatusCode::INVALID_IMAGE_FORMAT_DESCRIPTOR,
            Status::InvalidImageSize => StatusCode::INVALID_IMAGE_SIZE,
            Status::InvalidSampler => StatusCode::INVALID_SAMPLER,
            Status::InvalidBinary => StatusCode::INVALID_BINARY,
            Status::InvalidBuildOptions => StatusCode::INVALID_BUILD_OPTIONS,
            Status::InvalidProgram => StatusCode::INVALID_PROGRAM,
            Status::InvalidProgramExecutable => StatusCode::INVALID_PROGRAM_EXECUTABLE,
            Status::InvalidKernelName => StatusCode::INVALID_KERNEL_NAME,
            Status::InvalidKernelDefinition => StatusCode::INVALID_KERNEL_DEFINITION,
            Status::InvalidKernel => StatusCode::INVALID_KERNEL,
            Status::InvalidArgIndex => StatusCode::INVALID_ARG_INDEX,
            Status::InvalidArgValue => StatusCode::INVALID_ARG_VALUE,
            Status::InvalidArgSize => StatusCode::INVALID_ARG_SIZE,
            Status::InvalidKernelArgs => StatusCode::INVALID_KERNEL_ARGS,
            Status::InvalidWorkDimension => StatusCode::INVALID_WORK_DIMENSION,
            Status::InvalidWorkGroupSize => StatusCode::INVALID_WORK_GROUP_SIZE,
            Status::InvalidWorkItemSize => StatusCode::INVALID_WORK_ITEM_SIZE,
            Status::InvalidGlobalOffset => StatusCode::INVALID_GLOBAL_OFFSET,
            Status::InvalidEventWaitList => StatusCode::INVALID_EVENT_WAIT_LIST,
            Status::InvalidEvent => StatusCode::INVALID_EVENT,
            Status::InvalidOperation => StatusCode::INVALID_OPERATION,
            Status::InvalidGLObject => StatusCode::INVALID_GL_OBJECT,
            Status::InvalidBufferSize => StatusCode::INVALID_BUFFER_SIZE,
            Status::InvalidMIPLevel => StatusCode::INVALID_MIP_LEVEL,
            Status::InvalidGlobalWorkSize => StatusCode::INVALID_GLOBAL_WORK_SIZE,
            Status::InvalidProperty => StatusCode::INVALID_PROPERTY,
            Status::InvalidImageDescriptor => StatusCode::INVALID_IMAGE_DESCRIPTOR,
            Status::InvalidCompilerOptions => StatusCode::INVALID_COMPILER_OPTIONS,
            Status::InvalidLinkerOptions => StatusCode::INVALID_LINKER_OPTIONS,
            Status::InvalidDevicePartitionCount => StatusCode::INVALID_DEVICE_PARTITION_COUNT,
            Status::InvalidPipeSize => StatusCode::INVALID_PIPE_SIZE,
            Status::InvalidDeviceQueue => StatusCode::INVALID_DEVICE_QUEUE,
            Status::InvalidSpecId => StatusCode::INVALID_SPEC_ID,
            Status::MaxSizeRestrictionExceeded => StatusCode::MAX_SIZE_RESTRICTION_EXCEEDED,
            Status::InvalidStatusCode(fn_name) => {
                return Err(ValidationError::InvalidStatusCode(fn_name))
            }
        };
        Ok(data)
    }

    /// Generates Status from Status Code;
    pub fn from(status_code: cl_int, function_name: &'static str) -> Status {
        match status_code {
            StatusCode::SUCCESS => Status::Success,
            StatusCode::DEVICE_NOT_FOUND => Status::DeviceNotFound,
            StatusCode::DEVICE_NOT_AVAILABLE => Status::DeviceNotAvailable,
            StatusCode::COMPILER_NOT_AVAILABLE => Status::CompilerNotAvailable,
            StatusCode::MEM_OBJECT_ALLOCATION_FAILURE => Status::MemObjectAllocationFailure,
            StatusCode::OUT_OF_RESOURCES => Status::OutOfResources,
            StatusCode::OUT_OF_HOST_MEMORY => Status::OutOfHostMemory,
            StatusCode::PROFILING_INFO_NOT_AVAILABLE => Status::ProfilingInfoNotAvailable,
            StatusCode::MEM_COPY_OVERLAP => Status::MemCopyOverlap,
            StatusCode::IMAGE_FORMAT_MISMATCH => Status::ImageFormatMismatch,
            StatusCode::IMAGE_FORMAT_NOT_SUPPORTED => Status::ImageFormatNotSupported,
            StatusCode::BUILD_PROGRAM_FAILURE => Status::BuildProgramFailure,
            StatusCode::MAP_FAILURE => Status::MapFailure,
            StatusCode::MISALIGNED_SUB_BUFFER_OFFSET => Status::MisalignedSubBufferOffset,
            StatusCode::EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST => {
                Status::ExecStatusErrorForEventsInWaitList
            }
            StatusCode::COMPILE_PROGRAM_FAILURE => Status::CompileProgramFailure,
            StatusCode::LINKER_NOT_AVAILABLE => Status::LinkerNotAvailable,
            StatusCode::LINK_PROGRAM_FAILURE => Status::LinkProgramFailure,
            StatusCode::DEVICE_PARTITION_FAILED => Status::DevicePartitionFailed,
            StatusCode::KERNEL_ARG_INFO_NOT_AVAILABLE => Status::KernelArgInfoNotAvailable,
            StatusCode::INVALID_VALUE => Status::InvalidValue,
            StatusCode::INVALID_DEVICE_TYPE => Status::InvalidDeviceType,
            StatusCode::INVALID_PLATFORM => Status::InvalidPlatform,
            StatusCode::INVALID_DEVICE => Status::InvalidDevice,
            StatusCode::INVALID_CONTEXT => Status::InvalidContext,
            StatusCode::INVALID_QUEUE_PROPERTIES => Status::InvalidQueueProperties,
            StatusCode::INVALID_COMMAND_QUEUE => Status::InvalidCommandQueue,
            StatusCode::INVALID_HOST_PTR => Status::InvalidHostPtr,
            StatusCode::INVALID_MEM_OBJECT => Status::InvalidMemObject,
            StatusCode::INVALID_IMAGE_FORMAT_DESCRIPTOR => Status::InvalidImageFormatDescriptor,
            StatusCode::INVALID_IMAGE_SIZE => Status::InvalidImageSize,
            StatusCode::INVALID_SAMPLER => Status::InvalidSampler,
            StatusCode::INVALID_BINARY => Status::InvalidBinary,
            StatusCode::INVALID_BUILD_OPTIONS => Status::InvalidBuildOptions,
            StatusCode::INVALID_PROGRAM => Status::InvalidProgram,
            StatusCode::INVALID_PROGRAM_EXECUTABLE => Status::InvalidProgramExecutable,
            StatusCode::INVALID_KERNEL_NAME => Status::InvalidKernelName,
            StatusCode::INVALID_KERNEL_DEFINITION => Status::InvalidKernelDefinition,
            StatusCode::INVALID_KERNEL => Status::InvalidKernel,
            StatusCode::INVALID_ARG_INDEX => Status::InvalidArgIndex,
            StatusCode::INVALID_ARG_VALUE => Status::InvalidArgValue,
            StatusCode::INVALID_ARG_SIZE => Status::InvalidArgSize,
            StatusCode::INVALID_KERNEL_ARGS => Status::InvalidKernelArgs,
            StatusCode::INVALID_WORK_DIMENSION => Status::InvalidWorkDimension,
            StatusCode::INVALID_WORK_GROUP_SIZE => Status::InvalidWorkGroupSize,
            StatusCode::INVALID_WORK_ITEM_SIZE => Status::InvalidWorkItemSize,
            StatusCode::INVALID_GLOBAL_OFFSET => Status::InvalidGlobalOffset,
            StatusCode::INVALID_EVENT_WAIT_LIST => Status::InvalidEventWaitList,
            StatusCode::INVALID_EVENT => Status::InvalidEvent,
            StatusCode::INVALID_OPERATION => Status::InvalidOperation,
            StatusCode::INVALID_GL_OBJECT => Status::InvalidGLObject,
            StatusCode::INVALID_BUFFER_SIZE => Status::InvalidBufferSize,
            StatusCode::INVALID_PROPERTY => Status::InvalidProperty,
            StatusCode::INVALID_IMAGE_DESCRIPTOR => Status::InvalidImageDescriptor,
            StatusCode::INVALID_COMPILER_OPTIONS => Status::InvalidCompilerOptions,
            StatusCode::INVALID_LINKER_OPTIONS => Status::InvalidLinkerOptions,
            StatusCode::INVALID_DEVICE_PARTITION_COUNT => Status::InvalidDevicePartitionCount,
            StatusCode::INVALID_PIPE_SIZE => Status::InvalidPipeSize,
            StatusCode::INVALID_DEVICE_QUEUE => Status::InvalidDeviceQueue,
            StatusCode::INVALID_SPEC_ID => Status::InvalidSpecId,
            StatusCode::MAX_SIZE_RESTRICTION_EXCEEDED => Status::MaxSizeRestrictionExceeded,
            _ => Status::InvalidStatusCode(function_name),
        }
    }
}

impl ToLibraryError for Status {
    fn to_error(self) -> OpenCLAPILibraryError {
        OpenCLAPILibraryError::StatusError(self)
    }
}

#[allow(non_camel_case_types)]
pub enum Size {
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64,
    usize,
    isize,
    cl_image_desc,
    cl_image_format,
    cl_buffer_region,
    cl_name_version,
}
impl Size {
    pub fn get(&self) -> usize {
        match self {
            Size::i8 | Size::u8 => 1,
            Size::i16 | Size::u16 => 2,
            Size::i32 | Size::u32 | Size::f32 => 4,
            Size::i64
            | Size::u64
            | Size::f64
            | Size::usize
            | Size::isize
            | Size::cl_image_format => 8,
            Size::cl_buffer_region => 16,
            Size::cl_name_version => 68,
            Size::cl_image_desc => 80,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParamValue {
    String(String),
    UInt(cl_uint),
    ULong(cl_ulong),
    CSize(size_t),
    CPtr(intptr_t),
    ArrCSize(Vec<size_t>),
    ArrCPtr(Vec<intptr_t>),
    ArrULong(Vec<cl_ulong>),
    NameVersion(Vec<cl_name_version>),
}

impl ParamValue {
    pub fn unwrap_string(self) -> Option<String> {
        match self {
            ParamValue::String(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_uint(self) -> Option<cl_uint> {
        match self {
            ParamValue::UInt(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_ulong(self) -> Option<cl_ulong> {
        match self {
            ParamValue::ULong(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_csize(self) -> Option<size_t> {
        match self {
            ParamValue::CSize(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_cptr<T>(self) -> Option<WrappedPointer<T>> {
        match self {
            ParamValue::CPtr(dat) => Some(unsafe { WrappedPointer::from_raw(dat) }),
            _ => None,
        }
    }
    pub fn unwrap_mut_cptr<T>(self) -> Option<WrappedMutablePointer<T>> {
        match self {
            ParamValue::CPtr(dat) => Some(unsafe { WrappedMutablePointer::from_raw(dat) }),
            _ => None,
        }
    }
    pub fn unwrap_arr_ulong(self) -> Option<Vec<cl_ulong>> {
        match self {
            ParamValue::ArrULong(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_arr_csize(self) -> Option<Vec<size_t>> {
        match self {
            ParamValue::ArrCSize(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_arr_cptr(self) -> Option<Vec<intptr_t>> {
        match self {
            ParamValue::ArrCPtr(dat) => Some(dat),
            _ => None,
        }
    }
    pub fn unwrap_name_version(self) -> Option<Vec<cl_name_version>> {
        match self {
            ParamValue::NameVersion(dat) => Some(dat),
            _ => None,
        }
    }
}

impl Default for ParamValue {
    fn default() -> Self {
        ParamValue::UInt(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_code_to_status() {
        let status_code = -69;
        let fn_name = "test_status_code_to_status";
        let status = Status::from(status_code, fn_name);
        assert_eq!(Status::InvalidPipeSize, status);
    }
    #[test]
    fn test_status_to_status_code() {
        let status = Status::Success;
        let status_code = status.to_status_code().unwrap();
        assert_eq!(status_code, 0)
    }
    #[test]
    fn test_status_from_status_code() {
        let fn_name = "test_status_from_status_code";
        let status_code = -9999;
        let status = Status::from(status_code, fn_name);
        assert_eq!(status, Status::InvalidStatusCode(fn_name))
    }
    #[test]
    fn test_undefined_error_invalid_status_code() {
        let fn_name = "test_undefined_error_invalid_status_code";
        let status = Status::InvalidStatusCode(fn_name);
        let status_code = status.to_status_code().expect_err("FUNDS ARE SAIFU");
        assert_eq!(status_code, ValidationError::InvalidStatusCode(fn_name))
    }
}
