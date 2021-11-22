/*
 * errors.rs - Error handling for OpenCL APIs.
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

use crate::objects::enums::Status;
use thiserror::Error;

// All Error Traits here
pub trait ToLibraryError {
    fn to_error(self) -> OpenCLAPIError;
}

// Main Library Error
#[derive(Error, Debug, PartialEq)]
pub enum OpenCLAPIError {
    #[error("status code {int_code} error {code:?} at `{func}`, with reason: {reason}")]
    StatusCodeError {
        code: Status,
        int_code: i32,
        func: &'static str,
        reason: &'static str,
    },
    #[error("object validation error: `{0}`")]
    ObjectError(ValidationError),
    #[error("runtime error: `{0}`")]
    RuntimeError(RuntimeError),
}

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("unsupported bitfield configuration applied at api function `{0}`")]
    InvalidBitfield(&'static str),
    #[error("invalid property configuration found at api function `{0}`")]
    InvalidProperty(&'static str),
}

impl ToLibraryError for ValidationError {
    fn to_error(self) -> OpenCLAPIError {
        OpenCLAPIError::ObjectError(self)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("string conversion error, invalid bytearray returned from opencl native api")]
    CorruptedByteArray,
    #[error("null pointer has been returned to rust! this exception occured at function `{0}`")]
    NullPointer(&'static str),
}

impl ToLibraryError for RuntimeError {
    fn to_error(self) -> OpenCLAPIError {
        OpenCLAPIError::RuntimeError(self)
    }
}
