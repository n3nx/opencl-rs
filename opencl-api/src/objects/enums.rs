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
use crate::objects::structs::StatusCode;
use crate::objects::wrappers::*;
use opencl_heads::consts::*;
use opencl_heads::types::*;
use std::convert::From;

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
    InvalidStatusCode(cl_int),
}

/// Generates Status from Status Code;
impl From<cl_int> for Status {
    fn from(status_code: cl_int) -> Self {
        type Sc = StatusCode;
        type S = Status;
        match status_code {
            Sc::SUCCESS => S::Success,
            Sc::DEVICE_NOT_FOUND => S::DeviceNotFound,
            Sc::DEVICE_NOT_AVAILABLE => S::DeviceNotAvailable,
            Sc::COMPILER_NOT_AVAILABLE => S::CompilerNotAvailable,
            Sc::MEM_OBJECT_ALLOCATION_FAILURE => S::MemObjectAllocationFailure,
            Sc::OUT_OF_RESOURCES => S::OutOfResources,
            Sc::OUT_OF_HOST_MEMORY => S::OutOfHostMemory,
            Sc::PROFILING_INFO_NOT_AVAILABLE => S::ProfilingInfoNotAvailable,
            Sc::MEM_COPY_OVERLAP => S::MemCopyOverlap,
            Sc::IMAGE_FORMAT_MISMATCH => S::ImageFormatMismatch,
            Sc::IMAGE_FORMAT_NOT_SUPPORTED => S::ImageFormatNotSupported,
            Sc::BUILD_PROGRAM_FAILURE => S::BuildProgramFailure,
            Sc::MAP_FAILURE => S::MapFailure,
            Sc::MISALIGNED_SUB_BUFFER_OFFSET => S::MisalignedSubBufferOffset,
            Sc::EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST => S::ExecStatusErrorForEventsInWaitList,
            Sc::COMPILE_PROGRAM_FAILURE => S::CompileProgramFailure,
            Sc::LINKER_NOT_AVAILABLE => S::LinkerNotAvailable,
            Sc::LINK_PROGRAM_FAILURE => S::LinkProgramFailure,
            Sc::DEVICE_PARTITION_FAILED => S::DevicePartitionFailed,
            Sc::KERNEL_ARG_INFO_NOT_AVAILABLE => S::KernelArgInfoNotAvailable,
            Sc::INVALID_VALUE => S::InvalidValue,
            Sc::INVALID_DEVICE_TYPE => S::InvalidDeviceType,
            Sc::INVALID_PLATFORM => S::InvalidPlatform,
            Sc::INVALID_DEVICE => S::InvalidDevice,
            Sc::INVALID_CONTEXT => S::InvalidContext,
            Sc::INVALID_QUEUE_PROPERTIES => S::InvalidQueueProperties,
            Sc::INVALID_COMMAND_QUEUE => S::InvalidCommandQueue,
            Sc::INVALID_HOST_PTR => S::InvalidHostPtr,
            Sc::INVALID_MEM_OBJECT => S::InvalidMemObject,
            Sc::INVALID_IMAGE_FORMAT_DESCRIPTOR => S::InvalidImageFormatDescriptor,
            Sc::INVALID_IMAGE_SIZE => S::InvalidImageSize,
            Sc::INVALID_SAMPLER => S::InvalidSampler,
            Sc::INVALID_BINARY => S::InvalidBinary,
            Sc::INVALID_BUILD_OPTIONS => S::InvalidBuildOptions,
            Sc::INVALID_PROGRAM => S::InvalidProgram,
            Sc::INVALID_PROGRAM_EXECUTABLE => S::InvalidProgramExecutable,
            Sc::INVALID_KERNEL_NAME => S::InvalidKernelName,
            Sc::INVALID_KERNEL_DEFINITION => S::InvalidKernelDefinition,
            Sc::INVALID_KERNEL => S::InvalidKernel,
            Sc::INVALID_ARG_INDEX => S::InvalidArgIndex,
            Sc::INVALID_ARG_VALUE => S::InvalidArgValue,
            Sc::INVALID_ARG_SIZE => S::InvalidArgSize,
            Sc::INVALID_KERNEL_ARGS => S::InvalidKernelArgs,
            Sc::INVALID_WORK_DIMENSION => S::InvalidWorkDimension,
            Sc::INVALID_WORK_GROUP_SIZE => S::InvalidWorkGroupSize,
            Sc::INVALID_WORK_ITEM_SIZE => S::InvalidWorkItemSize,
            Sc::INVALID_GLOBAL_OFFSET => S::InvalidGlobalOffset,
            Sc::INVALID_EVENT_WAIT_LIST => S::InvalidEventWaitList,
            Sc::INVALID_EVENT => S::InvalidEvent,
            Sc::INVALID_OPERATION => S::InvalidOperation,
            Sc::INVALID_GL_OBJECT => S::InvalidGLObject,
            Sc::INVALID_BUFFER_SIZE => S::InvalidBufferSize,
            Sc::INVALID_PROPERTY => S::InvalidProperty,
            Sc::INVALID_IMAGE_DESCRIPTOR => S::InvalidImageDescriptor,
            Sc::INVALID_COMPILER_OPTIONS => S::InvalidCompilerOptions,
            Sc::INVALID_LINKER_OPTIONS => S::InvalidLinkerOptions,
            Sc::INVALID_DEVICE_PARTITION_COUNT => S::InvalidDevicePartitionCount,
            Sc::INVALID_PIPE_SIZE => S::InvalidPipeSize,
            Sc::INVALID_DEVICE_QUEUE => S::InvalidDeviceQueue,
            Sc::INVALID_SPEC_ID => S::InvalidSpecId,
            Sc::MAX_SIZE_RESTRICTION_EXCEEDED => S::MaxSizeRestrictionExceeded,
            x => S::InvalidStatusCode(x),
        }
    }
}

/// Retrieves Status Code from Status
impl From<Status> for cl_int {
    fn from(status: Status) -> cl_int {
        type Sc = StatusCode;
        type S = Status;
        match status {
            S::Success => Sc::SUCCESS,
            S::DeviceNotFound => Sc::DEVICE_NOT_FOUND,
            S::DeviceNotAvailable => Sc::DEVICE_NOT_AVAILABLE,
            S::CompilerNotAvailable => Sc::COMPILER_NOT_AVAILABLE,
            S::MemObjectAllocationFailure => Sc::MEM_OBJECT_ALLOCATION_FAILURE,
            S::OutOfResources => Sc::MEM_OBJECT_ALLOCATION_FAILURE,
            S::OutOfHostMemory => Sc::OUT_OF_HOST_MEMORY,
            S::ProfilingInfoNotAvailable => Sc::PROFILING_INFO_NOT_AVAILABLE,
            S::MemCopyOverlap => Sc::MEM_COPY_OVERLAP,
            S::ImageFormatMismatch => Sc::IMAGE_FORMAT_MISMATCH,
            S::ImageFormatNotSupported => Sc::IMAGE_FORMAT_NOT_SUPPORTED,
            S::BuildProgramFailure => Sc::BUILD_PROGRAM_FAILURE,
            S::MapFailure => Sc::MAP_FAILURE,
            S::MisalignedSubBufferOffset => Sc::MISALIGNED_SUB_BUFFER_OFFSET,
            S::ExecStatusErrorForEventsInWaitList => {
                CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST
            }
            S::CompileProgramFailure => Sc::COMPILE_PROGRAM_FAILURE,
            S::LinkerNotAvailable => Sc::LINKER_NOT_AVAILABLE,
            S::LinkProgramFailure => Sc::LINK_PROGRAM_FAILURE,
            S::DevicePartitionFailed => Sc::DEVICE_PARTITION_FAILED,
            S::KernelArgInfoNotAvailable => Sc::KERNEL_ARG_INFO_NOT_AVAILABLE,
            S::InvalidValue => Sc::INVALID_VALUE,
            S::InvalidDeviceType => Sc::INVALID_DEVICE_TYPE,
            S::InvalidPlatform => Sc::INVALID_PLATFORM,
            S::InvalidDevice => Sc::INVALID_DEVICE,
            S::InvalidContext => Sc::INVALID_CONTEXT,
            S::InvalidQueueProperties => Sc::INVALID_QUEUE_PROPERTIES,
            S::InvalidCommandQueue => Sc::INVALID_COMMAND_QUEUE,
            S::InvalidHostPtr => Sc::INVALID_HOST_PTR,
            S::InvalidMemObject => Sc::INVALID_MEM_OBJECT,
            S::InvalidImageFormatDescriptor => Sc::INVALID_IMAGE_FORMAT_DESCRIPTOR,
            S::InvalidImageSize => Sc::INVALID_IMAGE_SIZE,
            S::InvalidSampler => Sc::INVALID_SAMPLER,
            S::InvalidBinary => Sc::INVALID_BINARY,
            S::InvalidBuildOptions => Sc::INVALID_BUILD_OPTIONS,
            S::InvalidProgram => Sc::INVALID_PROGRAM,
            S::InvalidProgramExecutable => Sc::INVALID_PROGRAM_EXECUTABLE,
            S::InvalidKernelName => Sc::INVALID_KERNEL_NAME,
            S::InvalidKernelDefinition => Sc::INVALID_KERNEL_DEFINITION,
            S::InvalidKernel => Sc::INVALID_KERNEL,
            S::InvalidArgIndex => Sc::INVALID_ARG_INDEX,
            S::InvalidArgValue => Sc::INVALID_ARG_VALUE,
            S::InvalidArgSize => Sc::INVALID_ARG_SIZE,
            S::InvalidKernelArgs => Sc::INVALID_KERNEL_ARGS,
            S::InvalidWorkDimension => Sc::INVALID_WORK_DIMENSION,
            S::InvalidWorkGroupSize => Sc::INVALID_WORK_GROUP_SIZE,
            S::InvalidWorkItemSize => Sc::INVALID_WORK_ITEM_SIZE,
            S::InvalidGlobalOffset => Sc::INVALID_GLOBAL_OFFSET,
            S::InvalidEventWaitList => Sc::INVALID_EVENT_WAIT_LIST,
            S::InvalidEvent => Sc::INVALID_EVENT,
            S::InvalidOperation => Sc::INVALID_OPERATION,
            S::InvalidGLObject => Sc::INVALID_GL_OBJECT,
            S::InvalidBufferSize => Sc::INVALID_BUFFER_SIZE,
            S::InvalidMIPLevel => Sc::INVALID_MIP_LEVEL,
            S::InvalidGlobalWorkSize => Sc::INVALID_GLOBAL_WORK_SIZE,
            S::InvalidProperty => Sc::INVALID_PROPERTY,
            S::InvalidImageDescriptor => Sc::INVALID_IMAGE_DESCRIPTOR,
            S::InvalidCompilerOptions => Sc::INVALID_COMPILER_OPTIONS,
            S::InvalidLinkerOptions => Sc::INVALID_LINKER_OPTIONS,
            S::InvalidDevicePartitionCount => Sc::INVALID_DEVICE_PARTITION_COUNT,
            S::InvalidPipeSize => Sc::INVALID_PIPE_SIZE,
            S::InvalidDeviceQueue => Sc::INVALID_DEVICE_QUEUE,
            S::InvalidSpecId => Sc::INVALID_SPEC_ID,
            S::MaxSizeRestrictionExceeded => Sc::MAX_SIZE_RESTRICTION_EXCEEDED,
            S::InvalidStatusCode(x) => x,
        }
    }
}

impl Status {
    pub fn reason(&self) -> &'static str {
        type S = Status;
        match self {
            S::Success => "[cl_success] api executed successfully, without errors",
            S::BuildProgramFailure => "[cl_build_program_failure] function `clBuildProgram` failed to build the specified program",
            S::CompileProgramFailure => "[cl_compile_program_failure] function `clCompileProgram` failed to compile the specified program",
            S::CompilerNotAvailable => "[cl_compiler_not_available] compiling or building a program from source or IL when `CL_DEVICE_COMPILER_AVAILABLE` is `CL_FALSE`",
            S::DeviceNotFound => "[cl_device_not_found] no devices were found that match the specified device type",
            S::DeviceNotAvailable => "[cl_device_not_available] no devices were found that match the specified device type",
            S::DevicePartitionFailed => "[cl_device_partition_failed] device partitioning is supported but the device could not be further partitioned",
            S::ExecStatusErrorForEventsInWaitList => "[cl_exec_status_error_for_events_in_wait_list] any APIs being blocked when an event in the event wait list has a negative value, indicating it is in an error state",
            S::ImageFormatMismatch => "[cl_image_format_mismatch] attempting to copy images that do not use the same image format",
            S::ImageFormatNotSupported => "[cl_image_format_not_supported] attempting to create or use an image format that is not supported",
            S::InvalidArgIndex => "[cl_invalid_arg_index] attempting to get or set a kernel argument using an invalid index for the specified kernel",
            S::InvalidArgSize => "[cl_invalid_arg_size] the specified size of a kernel argument does not match the size of the kernel argument",
            S::InvalidArgValue => "[cl_invalid_arg_value] attempting to set a kernel argument that is not valid",
            S::InvalidBinary => "[cl_invalid_binary] a program binary is not valid for a device",
            S::InvalidBufferSize => "[cl_invalid_buffer_size] attempting to create a buffer or a sub-buffer with an invalid size",
            S::InvalidBuildOptions => "[cl_invalid_build_options] build options passed to function `clBuildProgram` are not valid",
            S::InvalidCommandQueue => "[cl_invalid_command_queue] the specified command queue is not a valid command queue",
            S::InvalidCompilerOptions => "[cl_invalid_compiler_options] compiler options passed to function `clCompileProgram` are not valid",
            S::InvalidContext => "[cl_invalid_context] a specified context is not a valid context, or when mixing objects from multiple contexts",
            S::InvalidDevice => "[cl_invalid_device] a specified device is not valid",
            S::InvalidDevicePartitionCount => "[cl_invalid_device_partition_count] the requested device partitioning using `CL_DEVICE_PARTITION_BY_COUNTS` is not valid",
            S::InvalidDeviceQueue => "[cl_invalid_device_queue] setting a device queue kernel argument to a value that is not a valid device command queue",
            S::InvalidDeviceType => "[cl_invalid_device_type] the requested device type is not a valid value",
            S::InvalidEvent => "[cl_invalid_event] a specified event object is not a valid event object",
            S::InvalidEventWaitList => "[cl_invalid_event_wait_list] the specified event wait list or number of events in the wait list is not valid",
            S::InvalidGlobalOffset => "[cl_invalid_global_offset] the specified global offset and global work size exceeds the limits of the device",
            S::InvalidGlobalWorkSize => "[cl_invalid_global_work_size] the specified global work size exceeds the limits of the device",
            S::InvalidHostPtr => "[cl_invalid_host_ptr] the specified host pointer is not valid for the specified flags",
            S::InvalidImageDescriptor => "[cl_invalid_image_descriptor] the specified image descriptor is `NULL` or specifies invalid values",
            S::InvalidImageFormatDescriptor => "[cl_invalid_image_format_descriptor] the specified image format descriptor is `NULL` or specifies invalid value",
            S::InvalidImageSize => "[cl_invalid_image_size] the specified image dimensions exceed the maximum dimensions for a device or all devices in a context",
            S::InvalidKernel => "[cl_invalid_kernel] the specified kernel is not a valid kernel",
            S::InvalidKernelArgs => "[cl_invalid_kernel_args] enqueing a kernel when some kernel arguments have not been set or are invalid",
            S::InvalidKernelDefinition => "[cl_invalid_kernel_definition] creating a kernel for multiple devices where the number of kernel arguments or kernel argument types are not the same for all devices",
            S::InvalidKernelName => "[cl_invalid_kernel_name] creating a kernel when no kernel with the specified name exists in the program object",
            S::InvalidLinkerOptions => "[cl_invalid_linker_options] build options passed to `clLinkProgram` are not valid",
            S::InvalidMemObject => "[cl_invalid_mem_object] a specified memory object is not a valid memory object",
            S::InvalidOperation => "[cl_invalid_operation] generic error code, the requested operation is not a valid operation",
            S::InvalidPipeSize => "[cl_invalid_pipe_size] attempting to create a pipe with an invalid packet size or number of packets",
            S::InvalidPlatform => "[cl_invalid_platform] the specified platform is not a valid platform",
            S::InvalidProgram => "[cl_invalid_program] a specified program is not a valid program object",
            S::InvalidProgramExecutable => "[cl_invalid_program_executable] the specified program is valid but has not been successfully built",
            S::InvalidProperty => "[cl_invalid_property] a specified property name is invalid, when the value for a property name is invalid, or when the same property name is specified more than once",
            S::InvalidQueueProperties => "[cl_invalid_queue_properties] specified queue properties are valid but are not supported by the device",
            S::InvalidSampler => "[cl_invalid_sampler] a specified sampler is not a valid sampler object",
            S::InvalidSpecId => "[cl_invalid_spec_id] the specified specialization constant ID is not valid for the specified program",
            S::InvalidValue => "[cl_invalid_value] generic error code, specified value is not a valid value",
            S::InvalidWorkDimension => "[cl_invalid_work_dimension] function `clEnqueueNDRangeKernel` when the specified work dimension is not valid",
            S::InvalidWorkGroupSize => "[cl_invalid_work_group_size] function `clEnqueueNDRangeKernel` when the specified total work-group size is not valid for the specified kernel or device",
            S::InvalidWorkItemSize => "[cl_invalid_work_item_size] function `clEnqueueNDRangeKernel` when the specified work-group size in one dimension is not valid for the device",
            S::KernelArgInfoNotAvailable => "[cl_kernel_arg_info_not_available] function `clGetKernelArgInfo` when kernel argument information is not available for the specified kernel",
            S::LinkProgramFailure => "[cl_link_program_failure] function `clLinkProgram` when there is a failure to link the specified binaries or libraries",
            S::LinkerNotAvailable => "[cl_linker_not_available] function `clLinkProgram` when `CL_DEVICE_LINKER_AVAILABLE` is `CL_FALSE`",
            S::MapFailure => "[cl_map_failure] there is a failure to map the specified region into the host address space",
            S::MemCopyOverlap => "[cl_mem_copy_overlap] copying from one region of a memory object to another where the source and destination regions overlap",
            S::MemObjectAllocationFailure => "[cl_mem_object_allocation_failure] there is a failure to allocate memory for a memory object",
            S::MisalignedSubBufferOffset => "[cl_misaligned_sub_buffer_offset] a sub-buffer object is created or used that is not aligned to `CL_DEVICE_MEM_BASE_ADDR_ALIGN` for the device",
            S::OutOfHostMemory => "[cl_out_of_host_memory] generic error code, memory could not be allocated on the host",
            S::OutOfResources => "[cl_out_of_resources] generic error code, resources could not be allocated on the device",
            S::MaxSizeRestrictionExceeded => "[cl_max_size_restriction_exceeded] the size of the specified kernel argument value exceeds the maximum size defined for the kernel argument",
            S::ProfilingInfoNotAvailable => "[cl_profiling_info_not_available] function `clGetEventProfilingInfo` when the command associated with the specified event was not enqueued into a command queue with `CL_QUEUE_PROFILING_ENABLE`",
            S::InvalidGLObject => "placeholder",
            S::InvalidMIPLevel => "placeholder",
            S::InvalidStatusCode(_) => "current status code does not match with any of the valid opencl codes"
        }
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
    ImageFormat(Vec<cl_image_format>),
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
        let status = Status::from(status_code);
        assert_eq!(Status::InvalidPipeSize, status);
    }

    #[test]
    fn test_status_to_status_code() {
        let status = Status::InvalidPipeSize;
        let status_code: cl_int = status.into();
        assert_eq!(status_code, -69)
    }

    #[test]
    fn test_invalid_status_to_code() {
        let status = Status::InvalidStatusCode(80085);
        let status_code: cl_int = status.into();
        assert_eq!(status_code, 80085);
    }
}
