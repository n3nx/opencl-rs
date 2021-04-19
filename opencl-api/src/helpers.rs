/*
 * helpers.rs - Helper functions for OpenCL APIs.
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
use crate::enums::Status;
use crate::errors::*;
use crate::structs::StatusCode;
use libc::c_void;
use opencl_heads::types::*;
use std::ptr;

/**********************************************************************
 *
 *
 *                      Helper Macros
 *
 */

#[macro_export]
macro_rules! get_count {
    // 0 parameters
    ($fn: ident) => {{
        let mut count = cl_uint::default();
        let status_code = unsafe { $fn(0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
    //2 parameters
    ($fn: ident, $param_1: ident, $param_2: ident) => {{
        let mut count = cl_uint::default();
        let status_code = unsafe { $fn($param_1, $param_2, 0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
    //3 parameters
    ($fn: ident, $param_1: ident, $param_2: ident, $param_3: ident) => {{
        let mut count = cl_uint::default();
        let status_code =
            unsafe { $fn($param_1, $param_2, $param_3, 0, ptr::null_mut(), &mut count) };
        status_update(status_code, stringify!($fn), count)?
    }};
}

#[macro_export]
macro_rules! size_getter {
    ($name:ident, $fn:ident) => {
        let $name = |ret_dat_ptr: *mut libc::c_void, param_name: cl_uint| -> APIResult<size_t> {
            let mut size: size_t = 0;
            let status_code = unsafe {
                $fn(
                    ret_dat_ptr,
                    param_name,
                    0,
                    ptr::null_mut(),
                    &mut size as *mut size_t,
                )
            };
            status_update(status_code, stringify!($fn), size)
        };
    };
}

#[macro_export]
macro_rules! gen_param_value {
    // return vector object
    ($fn:ident, $typ:tt, $ptr: ident, $param_name: ident, $size: ident) => {{
        if $size == 0 {
            Vec::default()
        } else {
            let arr_len = $size / Size::$typ.get();
            let mut param_value: Vec<$typ> = std::vec::from_elem($typ::default(), arr_len);
            let status_code = unsafe {
                $fn(
                    $ptr,
                    $param_name,
                    $size,
                    param_value.as_mut_ptr() as *mut c_void,
                    ptr::null_mut(),
                )
            };
            status_update(status_code, stringify!($fn), param_value)?
        }
    }};

    // return single object
    ($fn:ident, $typ:tt, $ptr: ident, $param_name: ident) => {{
        let size = Size::$typ.get();
        let mut param_value = $typ::default();
        let status_code = unsafe {
            $fn(
                $ptr,
                $param_name,
                size,
                crate::helpers::to_mut_ptr(&mut param_value) as *mut c_void,
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), param_value)?
    }};
}

#[macro_export]
macro_rules! gen_object_list {
    // 0 parameters
    ($fn:ident, $typ:tt, $count: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: $typ = std::vec::from_elem(ptr::null_mut(), arr_len);
        let status_code = unsafe { $fn($count, all_objects.as_mut_ptr(), ptr::null_mut()) };
        status_update(status_code, stringify!($fn), all_objects)
    }};

    // 2 parameters
    ($fn:ident, $typ:tt, $count: ident, $param_1: ident, $param_2: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: $typ = std::vec::from_elem(ptr::null_mut(), arr_len);
        let status_code = unsafe {
            $fn(
                $param_1,
                $param_2,
                $count,
                all_objects.as_mut_ptr(),
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), all_objects)
    }};

    // 3 parameters (Non Null)
    ($fn:ident, $typ:tt, $count: ident, $param_1: ident, $param_2: ident, $param_3: ident) => {{
        let arr_len = $count as usize;
        let mut all_objects: Vec<$typ> = std::vec::from_elem($typ::default(), arr_len);
        let status_code = unsafe {
            $fn(
                $param_1,
                $param_2,
                $param_3,
                $count,
                all_objects.as_mut_ptr() as *mut $typ,
                ptr::null_mut(),
            )
        };
        status_update(status_code, stringify!($fn), all_objects)
    }};
}

#[macro_export]
macro_rules! gen_add_trait {
    ($name:ident) => {
        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }
    };
}

/******************************************************************
 *
 *
 *              Helper Types, Aliases and Structs
 *
 */

// Aliases
pub type WrapPtr<T> = WrappedPointer<T>;
pub type WrapMutPtr<T> = WrappedMutablePointer<T>;
pub type NullMutPtr = WrappedMutablePointer<c_void>;

pub type APIResult<T> = ::std::result::Result<T, OpenCLAPILibraryError>;
pub type StatusCodeResult = ::std::result::Result<cl_int, ValidationError>;
pub type HelperResult<T> = ::std::result::Result<T, OpenCLAPILibraryError>;
pub type BitfieldResult<T> = ::std::result::Result<T, ValidationError>;

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

pub type PlatformList = Vec<cl_platform_id>;
pub type DeviceList = Vec<cl_device_id>;
pub type MemFormatList = Vec<cl_image_format>;

// Structs
#[derive(PartialEq, Debug)]
pub struct WrappedPointer<T>(*const T);
impl<T> WrappedPointer<T> {
    pub fn from<U>(x: &U) -> Self {
        // AVOIDED: This method uses heap allocation to gain non mutable pointer over data
        // Box::into_raw(Box::new(x)) as *const c_void
        Self(ptr::NonNull::from(x).as_ptr() as *const T)
    }
    pub fn from_owned<U>(x: U) -> Self {
        Self::from(&x)
    }
    /// Generally considered unsafe to run
    pub unsafe fn from_raw(x: intptr_t) -> Self {
        Self(x as *const T)
    }
    pub fn null() -> Self {
        Self(ptr::null())
    }
    pub fn unwrap(&self) -> *const T {
        self.0
    }
}

#[derive(PartialEq, Debug)]
pub struct WrappedMutablePointer<T>(*mut T);
impl<T> WrappedMutablePointer<T> {
    pub fn from<U>(x: &mut U) -> Self {
        // AVOIDED: This method uses heap allocation to gain non mutable pointer over data
        // Box::into_raw(Box::new(x)) as *const c_void
        Self(ptr::NonNull::from(x).as_ptr() as *mut T)
    }
    pub fn from_owned<U>(mut x: U) -> Self {
        Self::from(&mut x)
    }
    /// Generally considered unsafe to run
    pub unsafe fn from_raw(x: intptr_t) -> Self {
        Self(x as *mut T)
    }
    pub fn from_ptr(x: *mut T, fn_name: &'static str) -> HelperResult<Self> {
        match ptr::NonNull::new(x) {
            Some(dat) => Ok(Self(dat.as_ptr())),
            None => Err(HelperError::NullPointerException(fn_name).to_error()),
        }
    }
    pub fn null() -> Self {
        Self(ptr::null_mut())
    }
    pub fn unwrap(&self) -> *mut T {
        self.0
    }
}

/*******************************************************************
 *
 *
 *                      Helper Traits
 *
 */

pub trait GetSetGo
where
    Self: std::marker::Sized,
{
    fn get(&self) -> cl_bitfield;
    fn set(&mut self, value: cl_bitfield) -> BitfieldResult<()>;
    fn new(value: cl_bitfield) -> BitfieldResult<Self>;
}

/*******************************************************************
 *
 *
 *                      Helper Functions
 *
 */

pub fn status_update<T>(
    status_code: cl_int,
    function_name: &'static str,
    result: T,
) -> APIResult<T> {
    if StatusCode::SUCCESS != status_code {
        #[cfg(feature = "debug_mode")]
        eprintln!(
            "ERROR Status Code: {} from function {}",
            status_code, function_name
        );
        Err(Status::from(status_code, function_name).to_error())
    } else {
        Ok(result)
    }
}

/// Converts a byte Vec into a string, removing the trailing null byte if it
/// exists.
pub fn bytes_into_string(mut bytes: Vec<u8>) -> HelperResult<String> {
    if bytes.last() == Some(&0u8) {
        bytes.pop();
    }
    let output = String::from_utf8(bytes).map(|str| String::from(str.trim()));
    match output {
        Ok(x) => Ok(x),
        Err(_) => Err(HelperError::BytesIntoString.to_error()),
    }
}

pub fn to_mut_ptr<T>(x: &mut T) -> *mut T {
    &mut *x
}
