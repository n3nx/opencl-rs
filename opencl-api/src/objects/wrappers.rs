/*
 * wrappers.rs - Object wrappers structures for OpenCL APIs.
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

use crate::errors::*;
use crate::objects::types::HelperResult;
use opencl_heads::types::*;
use std::ptr;

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
            None => Err(RuntimeError::NullPointer(fn_name).to_error()),
        }
    }
    pub fn null() -> Self {
        Self(ptr::null_mut())
    }
    pub fn unwrap(&self) -> *mut T {
        self.0
    }
}
