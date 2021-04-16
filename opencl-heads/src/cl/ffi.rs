/*
 * ffi.rs - All OpenCL foreign interfaces (types, constants, functions).
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

// Types associated with functions
pub use super::types::*;
// Constants associated with functions
pub use super::consts::*;

// Functions
pub use super::cl::{
    clBuildProgram, clCreateBuffer, clCreateContext, clCreateContextFromType, clCreateKernel,
    clCreateKernelsInProgram, clCreateProgramWithBinary, clCreateProgramWithSource,
    clEnqueueCopyBuffer, clEnqueueCopyBufferToImage, clEnqueueCopyImage,
    clEnqueueCopyImageToBuffer, clEnqueueMapBuffer, clEnqueueMapImage, clEnqueueNDRangeKernel,
    clEnqueueNativeKernel, clEnqueueReadBuffer, clEnqueueReadImage, clEnqueueUnmapMemObject,
    clEnqueueWriteBuffer, clEnqueueWriteImage, clFinish, clFlush, clGetCommandQueueInfo,
    clGetContextInfo, clGetDeviceIDs, clGetDeviceInfo, clGetEventInfo, clGetEventProfilingInfo,
    clGetImageInfo, clGetKernelInfo, clGetKernelWorkGroupInfo, clGetMemObjectInfo,
    clGetPlatformIDs, clGetPlatformInfo, clGetProgramBuildInfo, clGetSamplerInfo,
    clGetSupportedImageFormats, clReleaseCommandQueue, clReleaseContext, clReleaseEvent,
    clReleaseKernel, clReleaseMemObject, clReleaseProgram, clReleaseSampler, clRetainCommandQueue,
    clRetainContext, clRetainEvent, clRetainKernel, clRetainMemObject, clRetainProgram,
    clRetainSampler, clSetKernelArg, clSetKernelArgSVMPointer, clSetKernelExecInfo,
    clWaitForEvents,
};

// Functions
#[cfg(feature = "cl_1_1")]
pub use super::cl::{
    clCreateSubBuffer, clCreateUserEvent, clEnqueueCopyBufferRect, clEnqueueReadBufferRect,
    clEnqueueWriteBufferRect, clSetEventCallback, clSetMemObjectDestructorCallback,
    clSetUserEventStatus,
};
// Functions
#[cfg(feature = "cl_1_2")]
pub use super::cl::{
    clCompileProgram, clCreateCommandQueue, clCreateImage, clCreateProgramWithBuiltInKernels,
    clCreateSampler, clCreateSubDevices, clEnqueueBarrierWithWaitList, clEnqueueFillBuffer,
    clEnqueueFillImage, clEnqueueMarkerWithWaitList, clEnqueueMigrateMemObjects, clEnqueueTask,
    clGetExtensionFunctionAddressForPlatform, clGetKernelArgInfo, clLinkProgram, clReleaseDevice,
    clRetainDevice, clUnloadPlatformCompiler,
};

// Functions
#[cfg(feature = "cl_2_0")]
pub use super::cl::{
    clCreateCommandQueueWithProperties, clCreatePipe, clCreateSamplerWithProperties,
    clEnqueueSVMFree, clEnqueueSVMMap, clEnqueueSVMMemFill, clEnqueueSVMMemcpy, clEnqueueSVMUnmap,
    clGetPipeInfo, clSVMAlloc, clSVMFree,
};

// Functions
#[cfg(feature = "cl_2_1")]
pub use super::cl::{
    clCloneKernel, clCreateProgramWithIL, clEnqueueSVMMigrateMem, clGetDeviceAndHostTimer,
    clGetHostTimer, clGetKernelSubGroupInfo, clSetDefaultDeviceCommandQueue,
};
// Functions
#[cfg(feature = "cl_2_2")]
pub use super::cl::{clSetProgramReleaseCallback, clSetProgramSpecializationConstant};
// Functions
#[cfg(feature = "cl_3_0")]
pub use super::cl::{
    clCreateBufferWithProperties, clCreateImageWithProperties, clSetContextDestructorCallback,
};
