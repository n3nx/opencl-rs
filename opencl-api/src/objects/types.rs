/*
 * types.rs - defined types for opencl apis.
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
use crate::errors::*;
use crate::objects::wrappers::*;
use libc::c_void;
use opencl_heads::types::*;

// Aliases
pub type WrapPtr<T> = WrappedPointer<T>;
pub type WrapMutPtr<T> = WrappedMutablePointer<T>;
pub type NullMutPtr = WrappedMutablePointer<c_void>;

pub type APIResult<T> = ::std::result::Result<T, OpenCLAPILibraryError>;
pub type StatusCodeResult = ::std::result::Result<cl_int, ValidationError>;
pub type HelperResult<T> = ::std::result::Result<T, OpenCLAPILibraryError>;
pub type BitfieldResult<T> = ::std::result::Result<T, ValidationError>;
pub type PropertyResult<T> = ::std::result::Result<T, ValidationError>;

pub type Properties = Option<Vec<intptr_t>>;
pub type LongProperties = Option<Vec<cl_properties>>;

pub type PlatformPtr = NullMutPtr;
pub type DevicePtr = NullMutPtr;
pub type ContextPtr = NullMutPtr;
pub type QueuePtr = NullMutPtr;
pub type MemPtr = NullMutPtr;
pub type ProgramPtr = NullMutPtr;
pub type KernelPtr = NullMutPtr;
pub type EventPtr = NullMutPtr;
pub type SamplerPtr = NullMutPtr;
pub type SVMPtr = NullMutPtr;

pub type PlatformList = Vec<cl_platform_id>;
pub type DeviceList = Vec<cl_device_id>;
pub type MemFormatList = Vec<cl_image_format>;
