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

use crate::enums::Status;
use crate::errors::*;
use crate::structs::StatusCode;
use opencl_heads::types::*;

#[macro_export]
macro_rules! size_getter {
    ($name:ident, $fn:ident) => {
        let $name = |obj: *mut libc::c_void, param_name: cl_uint| -> APIResult<size_t> {
            let mut size: size_t = 0;
            let status_code = unsafe {
                $fn(
                    obj,
                    param_name,
                    0,
                    ptr::null_mut(),
                    &mut size as *mut size_t,
                )
            };
            status_update(status_code, size)
        };
    };
}

pub type APIResult<T> = ::std::result::Result<T, Error>;
pub type UtilResult<T> = ::std::result::Result<T, Error>;

pub fn status_update<T>(status_code: cl_int, result: T) -> APIResult<T> {
    if StatusCode::SUCCESS != status_code {
        #[cfg(feature = "debug_mode")]
        eprintln!("ERROR Status Code: {}", status_code);
        Err(Error::StatusError(Status::from(status_code)))
    } else {
        Ok(result)
    }
}

/// Converts a byte Vec into a string, removing the trailing null byte if it
/// exists.
pub fn bytes_into_string(mut bytes: Vec<u8>) -> UtilResult<String> {
    if bytes.last() == Some(&0u8) {
        bytes.pop();
    }
    let output = String::from_utf8(bytes).map(|str| String::from(str.trim()));
    match output {
        Ok(x) => Ok(x),
        Err(_) => Err(Error::UtilError(UtilError::BytesIntoString)),
    }
}

pub fn to_mut_ptr<T>(x: &mut T) -> *mut T {
    &mut *x
}
