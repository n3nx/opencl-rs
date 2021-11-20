/*
 * functions.rs - opencl api helper functions.
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
use crate::objects::enums::Status;
use crate::objects::structs::StatusCode;
use crate::objects::types::{APIResult, HelperResult};
// use libc::c_void;
use opencl_heads::types::*;
// use std::ptr;

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