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
use crate::objects::types::StatusCodeResult;
use crate::objects::wrappers::*;
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
    InvalidStatusCode(cl_int),
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
            Status::InvalidStatusCode(x) => *x,
        };
        Ok(data)
    }

    /// Generates Status from Status Code;
    pub fn from(status_code: cl_int) -> Status {
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
            x => Status::InvalidStatusCode(x)
        }
    }

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
        // let fn_name = "test_status_code_to_status";
        let status = Status::from(status_code);
        assert_eq!(Status::InvalidPipeSize, status);
    }
    #[test]
    fn test_status_to_status_code() {
        let status = Status::Success;
        let status_code = status.to_status_code().unwrap();
        assert_eq!(status_code, 0)
    }
    // #[test]
    // fn test_status_from_status_code() {
    //     let fn_name = "test_status_from_status_code";
    //     let status_code = -9999;
    //     let status = Status::from(status_code, fn_name);
    //     assert_eq!(
    //         status,
    //         Status::InvalidStatusCode {
    //             code: status_code,
    //             func: fn_name
    //         }
    //     )
    // }
    // #[test]
    // fn test_undefined_error_invalid_status_code() {
    //     let fn_name = "test_undefined_error_invalid_status_code";
    //     let status = Status::InvalidStatusCode {
    //         code: 80085,
    //         func: fn_name,
    //     };
    //     let status_code = status.to_status_code().expect_err("FUNDS ARE SAIFU");
    //     assert_eq!(
    //         status_code,
    //         OpenCLAPIError::StatusCodeError {
    //             int_code: 80085,
    //             code: Status::InvalidStatusCode {
    //                 code: 80085,
    //                 func: fn_name
    //             },
    //             func: fn_name,
    //             reason: "current status code does not match with any of the valid opencl codes",
    //         }
    //     );
    // }
}
